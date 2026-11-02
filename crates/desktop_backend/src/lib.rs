#![forbid(unsafe_code)]
mod models;
pub mod support;
use ametista_core::*;
use ametista_ipc::Command;
pub use ametista_ipc::Reply as IpcReply;
use ametista_windows_runtime::{WindowsSession, scan};
pub use models::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::{Duration, Instant},
};
type UiResult<T> = std::result::Result<T, String>;
/// Only outcomes actually confirmed by the helper enter a benchmark intervention.
/// A partial apply is still a real intervention; skipped/failed items never enter it.
pub fn applied_ids(reply: &IpcReply) -> Vec<String> {
    if !reply.ok {
        return Vec::new();
    }
    let Some(value) = reply.result.get("operation") else {
        return Vec::new();
    };
    let Ok(report) = serde_json::from_value::<Report>(value.clone()) else {
        return Vec::new();
    };
    if report.dry_run {
        return Vec::new();
    }
    report
        .items
        .into_iter()
        .filter(|i| matches!(i.outcome, Outcome::Applied | Outcome::AlreadyApplied))
        .map(|i| i.item_id)
        .collect()
}
struct Evidence {
    catalog: Catalog,
    machine: Machine,
    at: Instant,
    rows: Vec<Observation>,
}
struct Ticket {
    review: Review,
    at: Instant,
    catalog: Catalog,
}
#[derive(Default)]
pub struct Desktop {
    state: Arc<Mutex<Analysis>>,
    evidence: Arc<Mutex<Option<Evidence>>>,
    ticket: Mutex<Option<Ticket>>,
    cancel: Arc<AtomicBool>,
    busy: Arc<AtomicBool>,
    sequence: AtomicU64,
}
fn error<T>(_: T) -> String {
    "operation_failed".into()
}
fn connect() -> UiResult<WindowsSession> {
    let session = WindowsSession::connect().map_err(error)?;
    if session.context.elevated() {
        return Err("ui_must_not_elevate".into());
    }
    Ok(session)
}
pub fn bootstrap() -> UiResult<Bootstrap> {
    let session = connect()?;
    let catalog = session.catalog();
    let machine = session.context.machine();
    let presets = [
        ("safe", Preset::Safe),
        ("balanced", Preset::Balanced),
        ("aggressive", Preset::Aggressive),
    ]
    .into_iter()
    .map(|(name, preset)| (name.into(), preset.select(catalog, machine)))
    .collect();
    let journal = session.history_readonly();
    let apps = session.removed_apps_readonly();
    let available = journal.is_ok() && apps.is_ok();
    Ok(Bootstrap {
        system: session.context.system_info().map_err(error)?,
        items: catalog.items().cloned().collect(),
        presets,
        compatible: catalog
            .items()
            .map(|i| (i.id.clone(), machine.supports(i)))
            .collect(),
        blocked_ids: catalog
            .items()
            .filter(|i| protection::is_protected(i))
            .map(|i| i.id.clone())
            .collect(),
        history: History {
            available,
            journal: journal.ok(),
            apps: apps.unwrap_or_default(),
        },
    })
}
impl Desktop {
    pub fn analysis_start(&self) -> UiResult<u64> {
        if self.busy.swap(true, Ordering::AcqRel) {
            return Err("analysis_busy".into());
        }
        self.cancel.store(false, Ordering::Release);
        *self.ticket.lock().map_err(error)? = None;
        *self.evidence.lock().map_err(error)? = None;
        let id = self.sequence.fetch_add(1, Ordering::Relaxed) + 1;
        *self.state.lock().map_err(error)? = Analysis {
            id,
            status: "running".into(),
            ..Analysis::default()
        };
        let (state, evidence, cancel, busy) = (
            self.state.clone(),
            self.evidence.clone(),
            self.cancel.clone(),
            self.busy.clone(),
        );
        std::thread::spawn(move || {
            let result = (|| -> UiResult<Evidence> {
                let session = connect()?;
                let rows = scan(&session.context, session.catalog(), &cancel, |progress| {
                    if let Ok(mut state) = state.lock()
                        && !cancel.load(Ordering::Acquire)
                    {
                        state.completed = state.completed.max(progress.completed);
                        state.total = progress.total;
                        state.item_id = progress.item_id;
                    }
                })
                .map_err(error)?;
                Ok(Evidence {
                    catalog: session.catalog().clone(),
                    machine: session.context.machine(),
                    at: Instant::now(),
                    rows,
                })
            })();
            if let Ok(mut state) = state.lock() {
                if cancel.load(Ordering::Acquire) {
                    state.status = "cancelled".into();
                } else {
                    match result {
                        Ok(result) => {
                            state.observations = result.rows.clone();
                            state.status = "complete".into();
                            if let Ok(mut evidence) = evidence.lock() {
                                *evidence = Some(result);
                            }
                        }
                        Err(message) => {
                            state.status = "error".into();
                            state.error = Some(message);
                        }
                    }
                }
            }
            busy.store(false, Ordering::Release);
        });
        Ok(id)
    }
    pub fn analysis_state(&self) -> UiResult<Analysis> {
        Ok(self.state.lock().map_err(error)?.clone())
    }
    pub fn analysis_cancel(&self) -> UiResult<()> {
        self.cancel.store(true, Ordering::Release);
        *self.ticket.lock().map_err(error)? = None;
        *self.evidence.lock().map_err(error)? = None;
        let mut state = self.state.lock().map_err(error)?;
        if state.status == "running" {
            state.status = "cancelling".into();
        }
        Ok(())
    }
    pub fn prepare_review(&self, selection: Selection) -> UiResult<Review> {
        if self.busy.load(Ordering::Acquire) {
            return Err("analysis_busy".into());
        }
        let stored = self.evidence.lock().map_err(error)?;
        let catalog = if matches!(selection.kind, ReviewKind::Apply | ReviewKind::RemoveApps) {
            let evidence = stored.as_ref().ok_or("analyze_first")?;
            if evidence.at.elapsed() > Duration::from_secs(120) {
                return Err("analysis_expired".into());
            }
            let catalog = &evidence.catalog;
            for id in &selection.ids {
                let item = catalog.get(id).map_err(|_| "invalid_item")?;
                if !evidence.machine.supports(item) || protection::is_protected(item) {
                    return Err("invalid_item".into());
                }
                if selection.kind == ReviewKind::RemoveApps {
                    let row = evidence
                        .rows
                        .iter()
                        .find(|r| &r.item_id == id)
                        .ok_or("invalid_item")?;
                    if item.action_type != ActionType::Appx
                        || row.removable != Some(true)
                        || row.exists != Some(true)
                    {
                        return Err("invalid_item".into());
                    }
                } else if item.revert != Revert::Auto {
                    return Err("invalid_item".into());
                }
            }
            catalog.clone()
        } else if selection.ids.iter().any(|id| id.starts_with("startup.")) {
            // Startup IDs are bound to the current user/Run value, including undo.
            connect()?.catalog().clone()
        } else {
            ametista_catalog::load_embedded().map_err(error)?
        };
        let unique: BTreeSet<_> = selection.ids.iter().collect();
        if unique.len() != selection.ids.len() || selection.ids.len() > 512 {
            return Err("invalid_item".into());
        }
        if !matches!(selection.kind, ReviewKind::History | ReviewKind::UndoAll)
            && selection.ids.is_empty()
        {
            return Err("empty_selection".into());
        }
        let items: Vec<_> = selection
            .ids
            .iter()
            .map(|id| {
                catalog
                    .get(id)
                    .cloned()
                    .map_err(|_| "invalid_item".to_string())
            })
            .collect::<UiResult<_>>()?;
        let mut counts = BTreeMap::new();
        for item in &items {
            *counts
                .entry(
                    serde_json::to_value(item.action_type)
                        .map_err(error)?
                        .as_str()
                        .ok_or("invalid_item")?
                        .into(),
                )
                .or_insert(0) += 1;
        }
        let review = Review {
            ticket: ametista_ipc::nonce().map_err(error)?,
            kind: selection.kind,
            ids: selection.ids,
            counts,
            items,
        };
        *self.ticket.lock().map_err(error)? = Some(Ticket {
            review: review.clone(),
            at: Instant::now(),
            catalog,
        });
        Ok(review)
    }
    pub fn consume_review(&self, approval: Approval) -> UiResult<Command> {
        let ticket = self
            .ticket
            .lock()
            .map_err(error)?
            .take()
            .ok_or("review_required")?;
        if approval.ticket != ticket.review.ticket || ticket.at.elapsed() > Duration::from_secs(120)
        {
            return Err("review_expired".into());
        }
        // Never authorize a new write from evidence retained across an uncertain transaction.
        *self.evidence.lock().map_err(error)? = None;
        let ids = ticket.review.ids;
        let command = match ticket.review.kind {
            ReviewKind::Apply => Command::Apply {
                ids,
                confirm: approval.confirm,
                high_risk_ids: approval.high_risk_ids,
                unverified_ids: approval.unverified_ids,
            },
            ReviewKind::RemoveApps => {
                if approval
                    .apps
                    .iter()
                    .map(|a| &a.item_id)
                    .collect::<BTreeSet<_>>()
                    != ids.iter().collect()
                {
                    return Err("invalid_item".into());
                }
                Command::RemoveApps {
                    apps: approval.apps,
                    confirm: approval.confirm,
                }
            }
            ReviewKind::Undo => Command::Undo {
                ids,
                confirm: approval.confirm,
            },
            ReviewKind::UndoAll => Command::UndoAll {
                confirm: approval.confirm,
            },
            ReviewKind::History => {
                if !approval.confirm {
                    return Err("confirmation_required".into());
                }
                Command::History
            }
            ReviewKind::Reinstall => Command::Reinstall {
                item_id: ids.first().cloned().ok_or("invalid_item")?,
                confirm: approval.confirm,
            },
        };
        command
            .validate_shape(&ticket.catalog)
            .map_err(|e| e.to_string())?;
        Ok(command)
    }
}
pub async fn execute(command: Command) -> std::result::Result<ametista_ipc::Reply, String> {
    #[cfg(windows)]
    {
        ametista_ipc::pipe::request(command)
            .await
            .map_err(|e| e.to_string())
    }
    #[cfg(not(windows))]
    {
        let _ = command;
        Err(ametista_ipc::Error::Windows.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn only_confirmed_real_apply_outcomes_enter_benchmark() {
        let mut reply = IpcReply {
            ok: true,
            result: serde_json::json!({"operation":{"dryRun":false,"items":[{"itemId":"reg.copilot","outcome":"applied","error":null,"requiresReboot":false},{"itemId":"svc.fax","outcome":"skipped","error":null,"requiresReboot":false}]}}),
            error: None,
        };
        assert_eq!(applied_ids(&reply), vec!["reg.copilot"]);
        reply.result["operation"]["dryRun"] = true.into();
        assert!(applied_ids(&reply).is_empty());
        reply.ok = false;
        assert!(applied_ids(&reply).is_empty());
    }
    type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;
    fn fixture() -> TestResultWith<Desktop> {
        let state = Desktop::default();
        *state.evidence.lock().map_err(error)? = Some(Evidence {
            catalog: ametista_catalog::load_embedded()?,
            machine: Machine {
                build: 26200,
                edition: Edition::Pro,
                disk: DiskKind::Unknown,
            },
            at: Instant::now(),
            rows: vec![],
        });
        Ok(state)
    }
    type TestResultWith<T> = std::result::Result<T, Box<dyn std::error::Error>>;
    fn approval(ticket: String) -> Approval {
        Approval {
            ticket,
            confirm: true,
            high_risk_ids: BTreeSet::new(),
            unverified_ids: BTreeSet::new(),
            apps: vec![],
        }
    }
    #[test]
    fn execution_requires_one_use_review_bound_to_selection() -> TestResult {
        let state = fixture()?;
        assert!(state.consume_review(approval("no ticket".into())).is_err());
        let review = state.prepare_review(Selection {
            kind: ReviewKind::Apply,
            ids: vec!["reg.advertising".into()],
        })?;
        let command = state.consume_review(approval(review.ticket.clone()))?;
        assert_eq!(command.ids(), vec!["reg.advertising"]);
        assert!(state.consume_review(approval(review.ticket)).is_err());
        Ok(())
    }
    #[test]
    fn stale_analysis_and_invalid_protected_ids_cannot_be_reviewed() -> TestResult {
        let state = fixture()?;
        for id in ["no.such.id", "reg.recall"] {
            assert!(
                state
                    .prepare_review(Selection {
                        kind: ReviewKind::Apply,
                        ids: vec![id.into()]
                    })
                    .is_err()
            );
        }
        state
            .evidence
            .lock()
            .map_err(error)?
            .as_mut()
            .ok_or("evidence")?
            .at = Instant::now() - Duration::from_secs(121);
        assert!(
            state
                .prepare_review(Selection {
                    kind: ReviewKind::Apply,
                    ids: vec!["reg.advertising".into()]
                })
                .is_err()
        );
        Ok(())
    }
    #[test]
    fn cancellation_discards_evidence_and_review() -> TestResult {
        let state = fixture()?;
        let review = state.prepare_review(Selection {
            kind: ReviewKind::Apply,
            ids: vec!["reg.advertising".into()],
        })?;
        state.analysis_cancel()?;
        assert!(state.consume_review(approval(review.ticket)).is_err());
        assert!(
            state
                .prepare_review(Selection {
                    kind: ReviewKind::Apply,
                    ids: vec!["reg.advertising".into()]
                })
                .is_err()
        );
        Ok(())
    }
    #[test]
    fn wrong_ticket_is_consumed() -> TestResult {
        let state = fixture()?;
        let review = state.prepare_review(Selection {
            kind: ReviewKind::Apply,
            ids: vec!["reg.advertising".into()],
        })?;
        assert!(state.consume_review(approval("wrong".into())).is_err());
        assert!(state.consume_review(approval(review.ticket)).is_err());
        Ok(())
    }
    #[test]
    fn app_consent_cannot_substitute_a_reviewed_id() -> TestResult {
        let state = fixture()?;
        {
            let mut guard = state.evidence.lock().map_err(error)?;
            let evidence = guard.as_mut().ok_or("evidence")?;
            let item = evidence.catalog.get("appx.microsoft.copilot")?;
            let mut row = observe(
                item,
                Ok(Snapshot::Appx {
                    packages: vec!["installed".into()],
                    provisioned: vec![],
                    families: vec![],
                }),
                ReadScope::CurrentUserOnly,
            );
            row.removable = Some(true);
            evidence.rows.push(row);
        }
        let review = state.prepare_review(Selection {
            kind: ReviewKind::RemoveApps,
            ids: vec!["appx.microsoft.copilot".into()],
        })?;
        let mut approved = approval(review.ticket);
        approved.apps.push(ametista_ipc::AppConsent {
            item_id: "appx.microsoft.gamingservices".into(),
            no_undo: true,
            gaming_risk: true,
            unverified: true,
        });
        assert!(state.consume_review(approved).is_err());
        Ok(())
    }
}
