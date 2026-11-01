//! Reuses the ordinary WAL, snapshots and undo; temporary services only stop.
use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GameAction {
    pub id: String,
    pub pause_service: bool,
    pub name: Localized,
    pub description: Localized,
}
pub fn validate_actions(catalog: &Catalog, actions: &[GameAction]) -> Result<()> {
    let mut seen = BTreeSet::new();
    for action in actions {
        let item = catalog.get(&action.id)?;
        if !seen.insert(&action.id)
            || !item.verified
            || item.risk == Risk::High
            || item.revert != Revert::Auto
            || item.requires_reboot
            || protection::is_protected(item)
        {
            return Err(Error::InvalidProfile("unsafe temporary action".into()));
        }
        let valid = match &item.target {
            Target::Service { .. } => action.pause_service,
            Target::Power { .. } => !action.pause_service,
            Target::Registry {
                hive: Hive::HKCU,
                path,
                name,
                value,
            } => {
                !action.pause_service
                    && path.eq_ignore_ascii_case(r"Software\Microsoft\GameBar")
                    && name == "AutoGameModeEnabled"
                    && *value == 1
            }
            _ => false,
        };
        if !valid {
            return Err(Error::InvalidProfile("not temporary".into()));
        }
    }
    Ok(())
}
pub fn active_ids(journal: &Journal) -> Vec<String> {
    journal
        .entries()
        .filter(|e| e.status != Status::Reverted)
        .map(|e| e.item_id.clone())
        .collect()
}
pub fn reconcile(
    engine: &mut Engine<'_>,
    journal: &mut Journal,
    actions: &[GameAction],
    desired: &[String],
) -> Result<Report> {
    validate_actions(engine.catalog, actions)?;
    if desired.len() > actions.len()
        || desired.iter().collect::<BTreeSet<_>>().len() != desired.len()
    {
        return Err(Error::InvalidProfile("duplicate temporary ID".into()));
    }
    for id in desired.iter().chain(active_ids(journal).iter()) {
        if !actions.iter().any(|a| &a.id == id) {
            return Err(Error::InvalidProfile("unknown temporary ID".into()));
        }
    }
    let request = Request {
        item_ids: desired.to_vec(),
        ..Request::default()
    };
    engine.validate_request(&request)?;
    if journal.needs_recovery() && !desired.is_empty() {
        return Err(Error::RecoveryRequired);
    }
    let current = active_ids(journal);
    let removed: Vec<_> = current
        .iter()
        .filter(|id| !desired.contains(id))
        .cloned()
        .collect();
    let mut report = Report {
        dry_run: false,
        items: vec![],
    };
    if !removed.is_empty() {
        report.items.extend(engine.undo(&removed, journal)?.items);
    }
    if report.items.iter().any(|i| i.outcome == Outcome::Failed) {
        return Err(Error::RecoveryRequired);
    }
    let added: Vec<_> = desired
        .iter()
        .filter(|id| !current.contains(id))
        .cloned()
        .collect();
    if !added.is_empty() {
        let next = engine.apply_temporary(
            &Request {
                item_ids: added,
                ..Request::default()
            },
            journal,
        )?;
        let failed = next.items.iter().any(|i| i.outcome == Outcome::Failed);
        report.items.extend(next.items);
        if failed {
            report.items.extend(engine.undo(&[], journal)?.items);
            return Err(Error::RecoveryRequired);
        }
    }
    Ok(report)
}
