use crate::{
    WindowsContext,
    transport::{self, Script},
};
use gametune_core::{ExecError, ExecResult, Journal, RemovedApps};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{
    fs::{File, OpenOptions},
    io::Write,
};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Owner {
    scope: String,
    sid: String,
}

fn lease(root: &Path, scope: &str, sid: &str) -> ExecResult<File> {
    let lock = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(root.join("machine.lock"))
        .map_err(crate::io_error)?;
    lock.try_lock().map_err(|_| ExecError::Conflict)?;
    // Recover ownership from intact WALs even if a power loss lost the owner pointer.
    let machine = scope.rsplit_once(':').ok_or(ExecError::Protected)?.0;
    for child in std::fs::read_dir(root).map_err(crate::io_error)? {
        let child = child.map_err(crate::io_error)?;
        let name = child.file_name();
        let Some(other_sid) = name.to_str() else {
            continue;
        };
        if other_sid == sid || !crate::valid_sid(other_sid) {
            continue;
        }
        let kind = child.file_type().map_err(crate::io_error)?;
        if kind.is_symlink() {
            return Err(ExecError::Protected);
        }
        if !kind.is_dir() {
            continue;
        }
        let wal = child.path().join("journal.wal");
        if wal.exists() {
            let journal = Journal::open_scoped(wal, &format!("{machine}:{other_sid}"))
                .map_err(crate::failed)?;
            if journal
                .entries()
                .any(|e| e.status != gametune_core::Status::Reverted)
            {
                return Err(ExecError::Conflict);
            }
        }
    }
    let path = root.join("scope-owner.json");
    match std::fs::read(&path) {
        Ok(bytes) => {
            let owner: Owner = serde_json::from_slice(&bytes).map_err(crate::failed)?;
            if !crate::valid_sid(&owner.sid) || !owner.scope.ends_with(&format!(":{}", owner.sid)) {
                return Err(ExecError::Protected);
            }
            if owner.scope != scope {
                let old = root.join(&owner.sid).join("journal.wal");
                if !old.exists() {
                    return Err(ExecError::Conflict);
                }
                {
                    let journal = Journal::open_scoped(old, &owner.scope).map_err(crate::failed)?;
                    if journal
                        .entries()
                        .any(|e| e.status != gametune_core::Status::Reverted)
                    {
                        return Err(ExecError::Conflict);
                    }
                }
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
        Err(e) => return Err(crate::io_error(e)),
    }
    let mut temp = tempfile::NamedTempFile::new_in(root).map_err(crate::io_error)?;
    temp.write_all(
        &serde_json::to_vec(&Owner {
            scope: scope.into(),
            sid: sid.into(),
        })
        .map_err(crate::failed)?,
    )
    .map_err(crate::io_error)?;
    temp.as_file().sync_all().map_err(crate::io_error)?;
    temp.persist(path)
        .map_err(|e| crate::io_error(e.error))?
        .sync_all()
        .map_err(crate::io_error)?;
    Ok(lock)
}

pub struct ProtectedStore {
    context: WindowsContext,
    directory: PathBuf,
    _machine_lock: File,
}
impl WindowsContext {
    pub fn protected_store(&self) -> ExecResult<ProtectedStore> {
        self.ensure_elevated()?;
        let directory: String = transport::run(
            Script::Storage,
            &serde_json::json!({"sid":self.interactive_sid()}),
        )?;
        let directory = PathBuf::from(directory);
        let root = directory.parent().ok_or(ExecError::Protected)?;
        let machine_lock = lease(root, self.scope(), self.interactive_sid())?;
        // Even an Appx-only session leaves an empty reversible WAL slot. A missing
        // previous owner's WAL then fails closed instead of silently losing originals.
        drop(
            Journal::open_scoped(directory.join("journal.wal"), self.scope())
                .map_err(crate::failed)?,
        );
        Ok(ProtectedStore {
            context: self.clone(),
            directory,
            _machine_lock: machine_lock,
        })
    }
}

impl ProtectedStore {
    pub fn directory(&self) -> &Path {
        &self.directory
    }
    fn verify(&self) -> ExecResult<()> {
        let directory: String = transport::run(
            Script::Storage,
            &serde_json::json!({"sid":self.context.interactive_sid()}),
        )?;
        if Path::new(&directory) != self.directory {
            return Err(ExecError::Protected);
        }
        Ok(())
    }
    pub fn journal(&self) -> gametune_core::Result<Journal> {
        self.verify()
            .map_err(|e| gametune_core::Error::CorruptJournal(e.to_string()))?;
        Journal::open_scoped(self.directory.join("journal.wal"), self.context.scope())
    }
    pub fn removed_apps(&self) -> gametune_core::Result<RemovedApps> {
        self.verify()
            .map_err(|e| gametune_core::Error::CorruptJournal(e.to_string()))?;
        RemovedApps::open(
            self.directory.join("removed-apps.json"),
            Some(self.context.scope()),
        )
    }
    pub fn export_journal(&self, journal: &Journal) -> gametune_core::Result<()> {
        self.verify()
            .map_err(|e| gametune_core::Error::CorruptJournal(e.to_string()))?;
        if journal.scope() != Some(self.context.scope()) {
            return Err(gametune_core::Error::CorruptJournal(
                "scope mismatch".into(),
            ));
        }
        // Projection only. Recovery always uses the synced WAL.
        std::fs::write(self.directory.join("journal.json"), journal.export_json()?)?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use gametune_core::*;
    type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;
    struct Noop;
    impl Executor for Noop {
        fn action_type(&self) -> ActionType {
            ActionType::Registry
        }
        fn capture(&self, _: &Item) -> ExecResult<Snapshot> {
            Ok(Snapshot::Registry {
                value: None,
                created_keys: vec![],
            })
        }
        fn apply(&mut self, _: &Item) -> ExecResult<()> {
            Ok(())
        }
        fn restore(&mut self, _: &Item, _: &Snapshot) -> ExecResult<()> {
            Ok(())
        }
    }
    #[test]
    fn machine_lease_preserves_first_original_across_interactive_users() -> TestResult {
        let dir = tempfile::tempdir()?;
        let a = "S-1-5-21-1";
        let b = "S-1-5-21-2";
        let scope_a = format!("machine:{a}");
        let scope_b = format!("machine:{b}");
        std::fs::create_dir(dir.path().join(a))?;
        let first = lease(dir.path(), &scope_a, a)?;
        assert!(matches!(
            lease(dir.path(), &scope_a, a),
            Err(ExecError::Conflict)
        ));
        drop(first);
        let catalog = gametune_catalog::load_embedded()?;
        let mut executors = Executors::default();
        executors.insert(Noop);
        let mut engine = Engine {
            catalog: &catalog,
            machine: Machine {
                build: 26100,
                edition: Edition::Pro,
                disk: DiskKind::Unknown,
            },
            executors: &mut executors,
        };
        let mut journal = Journal::open_scoped(dir.path().join(a).join("journal.wal"), &scope_a)?;
        engine.apply(
            &Request {
                item_ids: vec!["reg.cortana".into()],
                ..Request::default()
            },
            &mut journal,
        )?;
        drop(journal);
        std::fs::remove_file(dir.path().join("scope-owner.json"))?;
        assert!(matches!(
            lease(dir.path(), &scope_b, b),
            Err(ExecError::Conflict)
        ));
        let mut journal = Journal::open_scoped(dir.path().join(a).join("journal.wal"), &scope_a)?;
        engine.undo(&[], &mut journal)?;
        drop(journal);
        assert!(lease(dir.path(), &scope_b, b).is_ok());
        Ok(())
    }
    #[test]
    fn owner_metadata_cannot_redirect_to_a_path_outside_the_store() -> TestResult {
        let dir = tempfile::tempdir()?;
        std::fs::write(
            dir.path().join("scope-owner.json"),
            br#"{"scope":"x:../other","sid":"../other"}"#,
        )?;
        assert!(matches!(
            lease(dir.path(), "machine:S-1-5-21-1", "S-1-5-21-1"),
            Err(ExecError::Protected)
        ));
        Ok(())
    }
}
