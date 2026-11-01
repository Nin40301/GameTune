use crate::{
    Catalog, Error, ExecError, Executors, Item, Journal, Machine, Result, Revert, Risk, Status,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Default)]
pub struct Request {
    pub item_ids: Vec<String>,
    /// Fresh interactive confirmations; never deserialize these from profiles.
    pub high_risk_confirmations: BTreeSet<String>,
    pub unverified_confirmations: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Planned,
    Applied,
    AlreadyApplied,
    Reverted,
    Skipped,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemReport {
    pub item_id: String,
    pub outcome: Outcome,
    pub error: Option<ExecError>,
    pub requires_reboot: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub dry_run: bool,
    pub items: Vec<ItemReport>,
}

/// No public API accepts commands, registry paths, service names or arbitrary targets.
pub struct Engine<'a> {
    pub catalog: &'a Catalog,
    pub machine: Machine,
    pub executors: &'a mut Executors,
}

impl Engine<'_> {
    fn resolve<'a>(&'a self, request: &Request) -> Result<Vec<&'a Item>> {
        let mut seen = BTreeSet::new();
        let mut items = Vec::new();
        for id in &request.item_ids {
            let item = self.catalog.get(id)?;
            if !seen.insert(id) {
                continue;
            }
            if !self.machine.supports(item) {
                return Err(Error::Incompatible(id.clone()));
            }
            if item.risk == Risk::High && !request.high_risk_confirmations.contains(id) {
                return Err(Error::ConfirmationRequired(id.clone()));
            }
            if !item.verified && !request.unverified_confirmations.contains(id) {
                return Err(Error::ConfirmationRequired(id.clone()));
            }
            items.push(item);
        }
        Ok(items)
    }

    /// Default entrypoint: reads executors only, requires no journal and creates no files.
    pub fn run(&self, request: &Request) -> Result<Report> {
        let items = self.resolve(request)?;
        let mut report = Report {
            dry_run: true,
            items: Vec::new(),
        };
        for item in items {
            let result = if crate::protection::is_protected(item) {
                Err(ExecError::Protected)
            } else if item.revert == Revert::Manual {
                Err(ExecError::ManualReinstallRequired)
            } else {
                self.executors
                    .get(item.action_type)
                    .and_then(|e| e.capture(item))
                    .map(|_| ())
            };
            report
                .items
                .push(item_report(item, result, Outcome::Planned));
        }
        Ok(report)
    }

    /// Caller explicitly chooses real application and supplies a durable writer.
    pub fn apply(&mut self, request: &Request, journal: &mut Journal) -> Result<Report> {
        // Validate the entire request before the first mutation, even if a later ID is invalid.
        let ids: Vec<_> = self
            .resolve(request)?
            .iter()
            .map(|i| i.id.clone())
            .collect();
        self.validate_journal(journal)?;
        if journal.needs_recovery() {
            return Err(Error::RecoveryRequired);
        }
        let mut report = Report {
            dry_run: false,
            items: Vec::new(),
        };
        for id in ids {
            let item = self.catalog.get(&id)?;
            if crate::protection::is_protected(item) {
                report.items.push(item_report(
                    item,
                    Err(ExecError::Protected),
                    Outcome::Applied,
                ));
                continue;
            }
            if journal
                .entry(&item.target.key())
                .is_some_and(|e| e.status == Status::Applied)
            {
                report
                    .items
                    .push(item_report(item, Ok(()), Outcome::AlreadyApplied));
                continue;
            }
            if item.revert == Revert::Manual {
                report.items.push(item_report(
                    item,
                    Err(ExecError::ManualReinstallRequired),
                    Outcome::Applied,
                ));
                continue;
            }
            self.check_scope(item, journal)?;
            if let Err(error) = self
                .executors
                .get(item.action_type)
                .and_then(|e| e.ensure_write_ready())
            {
                report
                    .items
                    .push(item_report(item, Err(error), Outcome::Applied));
                continue;
            }
            let original = match self
                .executors
                .get(item.action_type)
                .and_then(|e| e.capture(item))
            {
                Ok(original) => original,
                Err(error) => {
                    report
                        .items
                        .push(item_report(item, Err(error), Outcome::Applied));
                    continue;
                }
            };
            journal.prepare(item, original)?; // sync_all completes before apply can run
            let result = self
                .executors
                .get_mut(item.action_type)
                .and_then(|e| e.apply(item));
            let status = if result.is_ok() {
                Status::Applied
            } else {
                Status::Failed
            };
            journal.transition(&item.target.key(), status, result.clone().err())?;
            report
                .items
                .push(item_report(item, result, Outcome::Applied));
        }
        Ok(report)
    }

    fn validate_journal(&self, journal: &Journal) -> Result<()> {
        for entry in journal.entries().filter(|e| e.status != Status::Reverted) {
            let item = self.catalog.get(&entry.item_id)?;
            self.check_scope(item, journal)?;
            if crate::protection::is_protected(item)
                || item.target != entry.target
                || item.target.key() != entry.key
            {
                return Err(Error::CorruptJournal(
                    "catalog target changed; recovery migration required".into(),
                ));
            }
        }
        Ok(())
    }

    fn check_scope(&self, item: &Item, journal: &Journal) -> Result<()> {
        if let Ok(executor) = self.executors.get(item.action_type)
            && let Some(scope) = executor.execution_scope()
            && journal.scope() != Some(scope)
        {
            return Err(Error::CorruptJournal(
                "machine/interactive SID scope mismatch".into(),
            ));
        }
        Ok(())
    }

    /// Empty IDs means undo all; otherwise the entire ID list is validated first.
    /// Recovery is rollback-first. Unknown or changed catalog targets fail closed.
    pub fn undo(&mut self, ids: &[String], journal: &mut Journal) -> Result<Report> {
        for id in ids {
            self.catalog.get(id)?;
        }
        self.validate_journal(journal)?;
        let entries: Vec<_> = journal
            .undo_order()
            .into_iter()
            .filter(|e| {
                e.status != Status::Reverted && (ids.is_empty() || ids.contains(&e.item_id))
            })
            .collect();
        let mut report = Report {
            dry_run: false,
            items: Vec::new(),
        };
        for entry in entries {
            let item = self.catalog.get(&entry.item_id)?;
            if let Err(error) = self
                .executors
                .get(item.action_type)
                .and_then(|e| e.ensure_write_ready())
            {
                report
                    .items
                    .push(item_report(item, Err(error), Outcome::Reverted));
                continue;
            }
            journal.transition(&entry.key, Status::Reverting, None)?;
            let result = self
                .executors
                .get_mut(item.action_type)
                .and_then(|e| e.restore(item, &entry.original));
            let status = if result.is_ok() {
                Status::Reverted
            } else {
                Status::UndoFailed
            };
            journal.transition(&entry.key, status, result.clone().err())?;
            report
                .items
                .push(item_report(item, result, Outcome::Reverted));
        }
        Ok(report)
    }
}

fn item_report(
    item: &Item,
    result: std::result::Result<(), ExecError>,
    success: Outcome,
) -> ItemReport {
    let outcome = match &result {
        Ok(()) => success,
        Err(ExecError::NotFound | ExecError::Unavailable | ExecError::ManualReinstallRequired) => {
            Outcome::Skipped
        }
        Err(_) => Outcome::Failed,
    };
    ItemReport {
        item_id: item.id.clone(),
        outcome,
        error: result.err(),
        requires_reboot: item.requires_reboot,
    }
}
