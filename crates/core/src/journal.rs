use crate::{Error, ExecError, Item, Result, Snapshot, Target};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Prepared,
    Applied,
    Failed,
    Reverting,
    Reverted,
    UndoFailed,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ActionType, Catalog, DiskKind, Edition, Engine, ExecResult, Executor, Executors, Machine,
        Request,
    };
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

    fn item() -> Result<Item> {
        let document: serde_json::Value =
            serde_json::from_str(include_str!("../../../catalog/items.json"))?;
        let items: Vec<Item> = serde_json::from_value(document["items"].clone())?;
        items
            .into_iter()
            .find(|i| i.id == "reg.cortana")
            .ok_or_else(|| Error::UnknownItem("fixture".into()))
    }
    fn snapshot() -> Snapshot {
        Snapshot::Registry {
            value: None,
            created_keys: vec![],
        }
    }

    #[test]
    fn prepared_record_survives_reopen_before_any_action() -> TestResult {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("journal.wal");
        let mut journal = Journal::open(&path)?;
        let item = item()?;
        journal.prepare(&item, snapshot())?;
        drop(journal);
        let reopened = Journal::open(path)?;
        assert!(reopened.needs_recovery());
        assert_eq!(
            reopened.entry(&item.target.key()).ok_or("entry")?.original,
            snapshot()
        );
        Ok(())
    }

    #[test]
    fn active_original_and_illegal_state_transitions_are_rejected() -> TestResult {
        let dir = tempfile::tempdir()?;
        let mut journal = Journal::open(dir.path().join("journal.wal"))?;
        let item = item()?;
        journal.prepare(&item, snapshot())?;
        assert!(journal.prepare(&item, snapshot()).is_err());
        assert!(
            journal
                .transition(&item.target.key(), Status::Reverted, None)
                .is_err()
        );
        let mut changed = journal.entry(&item.target.key()).ok_or("entry")?.clone();
        changed.original = Snapshot::Registry {
            value: None,
            created_keys: vec!["new".into()],
        };
        changed.status = Status::Applied;
        assert!(journal.append(changed).is_err());
        assert_eq!(
            journal.entry(&item.target.key()).ok_or("entry")?.original,
            snapshot()
        );
        Ok(())
    }

    struct CountingExecutor(Arc<AtomicUsize>);
    impl Executor for CountingExecutor {
        fn action_type(&self) -> ActionType {
            ActionType::Registry
        }
        fn capture(&self, _: &Item) -> ExecResult<Snapshot> {
            Ok(snapshot())
        }
        fn apply(&mut self, _: &Item) -> ExecResult<()> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
        fn restore(&mut self, _: &Item, _: &Snapshot) -> ExecResult<()> {
            Ok(())
        }
    }

    #[test]
    fn wal_write_failure_prevents_action_and_poisons_writer() -> TestResult {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("journal.wal");
        let mut journal = Journal::open(&path)?;
        // Replace only the test sink with a read-only handle to inject a real I/O failure.
        journal.file = File::open(&path)?;
        let item = item()?;
        let id = item.id.clone();
        let catalog = Catalog::try_new(vec![item])?;
        let calls = Arc::new(AtomicUsize::new(0));
        let mut executors = Executors::default();
        executors.insert(CountingExecutor(calls.clone()));
        let mut engine = Engine {
            catalog: &catalog,
            machine: Machine {
                build: 26100,
                edition: Edition::Pro,
                disk: DiskKind::Unknown,
            },
            executors: &mut executors,
        };
        let request = Request {
            item_ids: vec![id],
            ..Request::default()
        };
        assert!(matches!(
            engine.apply(&request, &mut journal),
            Err(Error::Io(_))
        ));
        assert!(matches!(
            engine.apply(&request, &mut journal),
            Err(Error::JournalPoisoned)
        ));
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert_eq!(journal.entries().count(), 0);
        Ok(())
    }

    #[test]
    fn crash_during_undo_retains_original_and_retryable_state() -> TestResult {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("journal.wal");
        let mut journal = Journal::open(&path)?;
        let item = item()?;
        journal.prepare(&item, snapshot())?;
        journal.transition(&item.target.key(), Status::Applied, None)?;
        journal.transition(&item.target.key(), Status::Reverting, None)?;
        drop(journal);
        let mut reopened = Journal::open(path)?;
        assert!(reopened.needs_recovery());
        assert_eq!(
            reopened.entry(&item.target.key()).ok_or("entry")?.original,
            snapshot()
        );
        reopened.transition(&item.target.key(), Status::Reverting, None)?;
        reopened.transition(&item.target.key(), Status::Reverted, None)?;
        assert!(!reopened.needs_recovery());
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct JournalEntry {
    pub key: String,
    pub item_id: String,
    pub target: Target,
    pub original: Snapshot,
    pub applied_at: String,
    pub status: Status,
    pub error: Option<ExecError>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Record {
    version: u8,
    scope: Option<String>,
    seq: u64,
    previous: String,
    entry: JournalEntry,
    hash: String,
}

fn digest(
    seq: u64,
    previous: &str,
    scope: &Option<String>,
    entry: &JournalEntry,
) -> Result<String> {
    Ok(format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&(2_u8, seq, previous, scope, entry))?)
    ))
}

/// Durable append-only WAL. Never overwrite an original while its change is active.
/// Exclusive OS file lock is held until drop. A write error poisons this instance.
pub struct Journal {
    scope: Option<String>,
    file: File,
    entries: BTreeMap<String, JournalEntry>,
    order: BTreeMap<String, u64>,
    seq: u64,
    hash: String,
    poisoned: bool,
    pub discarded_torn_tail: bool,
}

impl Journal {
    /// The caller creates/protects the parent directory. No implicit ProgramData writes.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Self::open_inner(path.as_ref(), None)
    }

    pub fn open_scoped(path: impl AsRef<Path>, scope: &str) -> Result<Self> {
        Self::open_inner(path.as_ref(), Some(scope.to_owned()))
    }

    pub fn scope(&self) -> Option<&str> {
        self.scope.as_deref()
    }

    fn open_inner(path: &Path, scope: Option<String>) -> Result<Self> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;
        file.try_lock()
            .map_err(|e| Error::JournalLocked(e.to_string()))?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        let complete_len = bytes.iter().rposition(|b| *b == b'\n').map_or(0, |p| p + 1);
        let mut journal = Self {
            scope,
            file,
            entries: BTreeMap::new(),
            order: BTreeMap::new(),
            seq: 0,
            hash: String::new(),
            poisoned: false,
            discarded_torn_tail: complete_len < bytes.len(),
        };
        for line in bytes[..complete_len]
            .split(|b| *b == b'\n')
            .filter(|line| !line.is_empty())
        {
            let record: Record =
                serde_json::from_slice(line).map_err(|e| Error::CorruptJournal(e.to_string()))?;
            if record.version != 2
                || record.scope != journal.scope
                || record.seq != journal.seq + 1
                || record.previous != journal.hash
                || digest(record.seq, &record.previous, &record.scope, &record.entry)?
                    != record.hash
            {
                return Err(Error::CorruptJournal(
                    "sequence or checksum mismatch".into(),
                ));
            }
            journal.validate_transition(&record.entry)?;
            journal.seq = record.seq;
            journal.hash = record.hash;
            if record.entry.status == Status::Prepared {
                journal.order.insert(record.entry.key.clone(), record.seq);
            }
            journal
                .entries
                .insert(record.entry.key.clone(), record.entry);
        }
        // An action runs only after the complete newline-terminated record is synced.
        // A torn completion therefore leaves its earlier prepared/reverting event intact.
        if journal.discarded_torn_tail {
            journal.file.set_len(complete_len as u64)?;
            journal.file.sync_all()?;
        }
        journal.file.seek(SeekFrom::End(0))?;
        Ok(journal)
    }

    pub fn entries(&self) -> impl DoubleEndedIterator<Item = &JournalEntry> {
        self.entries.values()
    }
    pub fn entry(&self, key: &str) -> Option<&JournalEntry> {
        self.entries.get(key)
    }
    pub(crate) fn undo_order(&self) -> Vec<JournalEntry> {
        let mut entries: Vec<_> = self.entries.values().cloned().collect();
        entries.sort_by_key(|e| std::cmp::Reverse(self.order.get(&e.key).copied().unwrap_or(0)));
        entries
    }
    pub fn needs_recovery(&self) -> bool {
        self.entries
            .values()
            .any(|e| !matches!(e.status, Status::Applied | Status::Reverted))
    }

    pub(crate) fn prepare(&mut self, item: &Item, original: Snapshot) -> Result<()> {
        let entry = JournalEntry {
            key: item.target.key(),
            item_id: item.id.clone(),
            target: item.target.clone(),
            original,
            applied_at: time::OffsetDateTime::now_utc()
                .format(&time::format_description::well_known::Rfc3339)?,
            status: Status::Prepared,
            error: None,
        };
        self.append(entry)
    }

    pub(crate) fn transition(
        &mut self,
        key: &str,
        status: Status,
        error: Option<ExecError>,
    ) -> Result<()> {
        let mut entry = self
            .entries
            .get(key)
            .cloned()
            .ok_or_else(|| Error::CorruptJournal("missing prepared entry".into()))?;
        entry.status = status;
        entry.error = error;
        self.append(entry)
    }

    fn validate_transition(&self, next: &JournalEntry) -> Result<()> {
        if next.key != next.target.key() || next.target.action_type() != next.original.action_type()
        {
            return Err(Error::CorruptJournal(
                "invalid target or snapshot type".into(),
            ));
        }
        let valid = match self.entries.get(&next.key) {
            None => next.status == Status::Prepared,
            Some(previous) if previous.status == Status::Reverted => {
                next.status == Status::Prepared
            }
            Some(previous) => {
                let same = previous.original == next.original
                    && previous.target == next.target
                    && previous.item_id == next.item_id
                    && previous.applied_at == next.applied_at;
                same && matches!(
                    (previous.status, next.status),
                    (
                        Status::Prepared,
                        Status::Applied | Status::Failed | Status::Reverting
                    ) | (
                        Status::Applied | Status::Failed | Status::UndoFailed,
                        Status::Reverting
                    ) | (
                        Status::Reverting,
                        Status::Reverting | Status::Reverted | Status::UndoFailed
                    )
                )
            }
        };
        if !valid {
            return Err(Error::CorruptJournal(
                "invalid state transition or changed original".into(),
            ));
        }
        Ok(())
    }

    fn append(&mut self, entry: JournalEntry) -> Result<()> {
        if self.poisoned {
            return Err(Error::JournalPoisoned);
        }
        self.validate_transition(&entry)?;
        let seq = self.seq + 1;
        let hash = digest(seq, &self.hash, &self.scope, &entry)?;
        let record = Record {
            version: 2,
            scope: self.scope.clone(),
            seq,
            previous: self.hash.clone(),
            entry,
            hash,
        };
        let mut bytes = serde_json::to_vec(&record)?;
        bytes.push(b'\n');
        if let Err(error) = self
            .file
            .write_all(&bytes)
            .and_then(|()| self.file.sync_all())
        {
            self.poisoned = true;
            return Err(error.into());
        }
        self.seq = seq;
        self.hash = record.hash;
        if record.entry.status == Status::Prepared {
            self.order.insert(record.entry.key.clone(), seq);
        }
        self.entries.insert(record.entry.key.clone(), record.entry);
        Ok(())
    }

    /// A standard JSON projection for UI/export, not the recovery source of truth.
    pub fn export_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(
            &self.entries.values().collect::<Vec<_>>(),
        )?)
    }
}
