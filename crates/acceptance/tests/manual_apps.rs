use gametune_catalog::load_embedded;
use gametune_core::*;
use gametune_exec_mock::{MockExecutor, MockHost, Stage};
use std::path::PathBuf;
type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;
const NEWS: &str = "appx.microsoft.bingnews";
fn machine() -> Machine {
    Machine {
        build: 26100,
        edition: Edition::Pro,
        disk: DiskKind::Unknown,
    }
}
fn state(name: &str) -> Snapshot {
    Snapshot::Appx {
        packages: vec![format!("{name}_1.0.0.0_x64__8wekyb3d8bbwe")],
        provisioned: vec![format!("{name}_1.0.0.0_neutral_~_8wekyb3d8bbwe")],
        families: vec![format!("{name}_8wekyb3d8bbwe")],
    }
}
fn consent(id: &str) -> AppRemovalConsent {
    AppRemovalConsent {
        item_id: id.into(),
        acknowledge_no_undo: true,
        acknowledge_gaming_dependencies: false,
        acknowledge_unverified: true,
    }
}

struct InspectBeforeApply {
    inner: MockExecutor,
    path: PathBuf,
    panic_after: bool,
}
impl Executor for InspectBeforeApply {
    fn action_type(&self) -> ActionType {
        ActionType::Appx
    }
    fn capture(&self, item: &Item) -> ExecResult<Snapshot> {
        self.inner.capture(item)
    }
    fn apply(&mut self, item: &Item) -> ExecResult<()> {
        let bytes = std::fs::read(&self.path).map_err(|e| ExecError::Failed(e.to_string()))?;
        let doc: serde_json::Value =
            serde_json::from_slice(&bytes).map_err(|e| ExecError::Failed(e.to_string()))?;
        assert_eq!(doc["entries"][0]["status"], "prepared");
        assert_eq!(
            doc["entries"][0]["packageFamilyNames"][0],
            "Microsoft.BingNews_8wekyb3d8bbwe"
        );
        self.inner.apply(item)?;
        assert!(
            !self.panic_after,
            "simulated crash after uninstall before terminal record"
        );
        Ok(())
    }
    fn restore(&mut self, item: &Item, original: &Snapshot) -> ExecResult<()> {
        self.inner.restore(item, original)
    }
}

#[test]
fn removal_requires_fresh_per_app_and_reinforced_gaming_consent() -> TestResult {
    let catalog = load_embedded()?;
    let host = MockHost::default();
    let mut executors = Executors::default();
    executors.insert(MockExecutor::new(ActionType::Appx, host.clone()));
    let dir = tempfile::tempdir()?;
    let mut store = RemovedApps::open(dir.path().join("removed-apps.json"), None)?;
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    assert!(matches!(
        engine.remove_app(
            &AppRemovalConsent {
                item_id: NEWS.into(),
                ..AppRemovalConsent::default()
            },
            &mut store
        ),
        Err(Error::ConfirmationRequired(_))
    ));
    let gaming = "appx.microsoft.xboxidentityprovider";
    assert!(matches!(
        engine.remove_app(&consent(gaming), &mut store),
        Err(Error::ConfirmationRequired(_))
    ));
    assert!(matches!(
        engine.remove_app(&consent("svc.diagtrack"), &mut store),
        Err(Error::UnknownItem(_))
    ));
    assert_eq!(host.writes()?, 0);
    assert!(store.entries().is_empty());
    Ok(())
}

#[test]
fn manual_removal_persists_name_and_pfn_before_action_and_offers_real_store_button() -> TestResult {
    let catalog = load_embedded()?;
    let item = catalog.get(NEWS)?;
    let host = MockHost::default();
    host.seed(item, state("Microsoft.BingNews"))?;
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("removed-apps.json");
    let mut store = RemovedApps::open(&path, None)?;
    let mut executors = Executors::default();
    executors.insert(InspectBeforeApply {
        inner: MockExecutor::new(ActionType::Appx, host.clone()),
        path: path.clone(),
        panic_after: false,
    });
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    assert!(engine.preview_app_removal(NEWS)?.is_ok());
    assert_eq!(host.writes()?, 0);
    engine.remove_app(&consent(NEWS), &mut store)??;
    assert_eq!(store.entries()[0].status, RemovalStatus::Removed);
    assert_eq!(store.entries()[0].name, "Microsoft.BingNews");
    assert_eq!(
        store.reinstall_actions(&catalog, NEWS)?[0].uri(),
        "ms-windows-store://pdp/?PFN=Microsoft.BingNews_8wekyb3d8bbwe"
    );
    let html = store.export_html(&catalog, false)?;
    assert!(html.contains("Desfazer não reinstala"));
    assert!(html.contains("Reinstalar pela Microsoft Store"));
    assert!(!html.contains("<script"));
    drop(store);
    assert_eq!(
        RemovedApps::open(path, None)?.entries()[0].status,
        RemovalStatus::Removed
    );
    Ok(())
}

#[test]
fn crash_leaves_prepared_history_and_requires_reconciliation_without_undo_claim() -> TestResult {
    let catalog = load_embedded()?;
    let host = MockHost::default();
    host.seed(catalog.get(NEWS)?, state("Microsoft.BingNews"))?;
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("removed-apps.json");
    let mut store = RemovedApps::open(&path, None)?;
    let mut executors = Executors::default();
    executors.insert(InspectBeforeApply {
        inner: MockExecutor::new(ActionType::Appx, host.clone()),
        path: path.clone(),
        panic_after: true,
    });
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(
            || engine.remove_app(&consent(NEWS), &mut store)
        ))
        .is_err()
    );
    drop(store);
    let mut reopened = RemovedApps::open(&path, None)?;
    assert!(reopened.pending());
    engine.reconcile_app_removal(NEWS, &mut reopened)?;
    assert_eq!(reopened.entries()[0].status, RemovalStatus::Removed);
    assert!(!reopened.pending());
    Ok(())
}

#[test]
fn partial_removal_keeps_reinstall_metadata_and_does_not_abort_reversible_undo() -> TestResult {
    let catalog = load_embedded()?;
    let host = MockHost::default();
    host.seed(catalog.get(NEWS)?, state("Microsoft.BingNews"))?;
    host.fail(
        catalog.get(NEWS)?,
        Stage::AfterApply,
        ExecError::AccessDenied,
    )?;
    let registry = catalog.get("reg.cortana")?;
    let prior = Snapshot::Registry {
        value: None,
        created_keys: vec![],
    };
    host.seed(registry, prior.clone())?;
    let dir = tempfile::tempdir()?;
    let mut store = RemovedApps::open(dir.path().join("removed-apps.json"), None)?;
    let mut journal = Journal::open(dir.path().join("journal.wal"))?;
    let mut executors = Executors::default();
    executors.insert(MockExecutor::new(ActionType::Appx, host.clone()));
    executors.insert(MockExecutor::new(ActionType::Registry, host.clone()));
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    assert_eq!(
        engine.remove_app(&consent(NEWS), &mut store)?,
        Err(ExecError::AccessDenied)
    );
    assert_eq!(store.entries()[0].status, RemovalStatus::Failed);
    assert!(!store.reinstall_actions(&catalog, NEWS)?.is_empty());
    engine.apply(
        &Request {
            item_ids: vec![registry.id.clone()],
            ..Request::default()
        },
        &mut journal,
    )?;
    engine.undo(&[], &mut journal)?;
    assert_eq!(host.snapshot()?.get(&registry.target.key()), Some(&prior));
    assert!(matches!(
        engine.preview_app_removal(NEWS)?,
        Err(ExecError::NotFound)
    ));
    assert_eq!(store.entries().len(), 1);
    Ok(())
}

#[test]
fn malicious_pfn_and_wrong_scope_are_rejected_before_uninstall() -> TestResult {
    let catalog = load_embedded()?;
    let host = MockHost::default();
    host.seed(
        catalog.get(NEWS)?,
        Snapshot::Appx {
            packages: vec!["x".into()],
            provisioned: vec![],
            families: vec!["Microsoft.BingNews_8wekyb3d8bbwe&command=calc".into()],
        },
    )?;
    let dir = tempfile::tempdir()?;
    let mut store = RemovedApps::open(dir.path().join("removed-apps.json"), None)?;
    let mut executors = Executors::default();
    executors.insert(MockExecutor::new(ActionType::Appx, host.clone()));
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    assert!(engine.remove_app(&consent(NEWS), &mut store).is_err());
    assert_eq!(host.writes()?, 0);
    let mut scoped = RemovedApps::open(
        dir.path().join("other.json"),
        Some("other-machine:other-SID"),
    )?;
    assert!(engine.remove_app(&consent(NEWS), &mut scoped).is_err());
    assert_eq!(host.writes()?, 0);
    Ok(())
}

#[test]
fn imported_profiles_cannot_authorize_app_removal_and_presets_never_select_appx() -> TestResult {
    let catalog = load_embedded()?;
    let profile = Profile::import(
        &format!(r#"{{"version":1,"name":"Apps","itemIds":["{NEWS}"]}}"#),
        &catalog,
    )?;
    assert_eq!(profile.item_ids(), &[NEWS]);
    for preset in [Preset::Safe, Preset::Balanced, Preset::Aggressive] {
        for id in preset.select(&catalog, machine()) {
            assert_ne!(catalog.get(&id)?.action_type, ActionType::Appx);
        }
    }
    let host = MockHost::default();
    host.seed(catalog.get(NEWS)?, state("Microsoft.BingNews"))?;
    let mut executors = Executors::default();
    executors.insert(MockExecutor::new(ActionType::Appx, host.clone()));
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    let dir = tempfile::tempdir()?;
    let mut journal = Journal::open(dir.path().join("journal.wal"))?;
    let report = engine.apply(
        &Request {
            item_ids: profile.item_ids().to_vec(),
            ..Request::default()
        },
        &mut journal,
    )?;
    assert_eq!(
        report.items[0].error,
        Some(ExecError::ManualReinstallRequired)
    );
    assert_eq!(host.writes()?, 0);
    Ok(())
}
