use ametista_catalog::load_embedded;
use ametista_core::*;
type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

#[test]
fn scoped_journal_rejects_other_machine_sid_and_unscoped_open() -> TestResult {
    struct Scoped;
    impl Executor for Scoped {
        fn action_type(&self) -> ActionType {
            ActionType::Registry
        }
        fn execution_scope(&self) -> Option<&str> {
            Some("machine-a:S-1-5-21-1")
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
    let catalog = load_embedded()?;
    let mut executors = Executors::default();
    executors.insert(Scoped);
    let mut engine = Engine {
        catalog: &catalog,
        machine: Machine {
            build: 26100,
            edition: Edition::Pro,
            disk: DiskKind::Unknown,
        },
        executors: &mut executors,
    };
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("journal.wal");
    let mut wrong = Journal::open(&path)?;
    let request = Request {
        item_ids: vec!["reg.cortana".into()],
        ..Request::default()
    };
    assert!(engine.apply(&request, &mut wrong).is_err());
    drop(wrong);
    let mut correct = Journal::open_scoped(&path, "machine-a:S-1-5-21-1")?;
    engine.apply(&request, &mut correct)?;
    drop(correct);
    assert!(Journal::open_scoped(&path, "machine-b:S-1-5-21-1").is_err());
    assert!(Journal::open_scoped(&path, "machine-a:S-1-5-21-2").is_err());
    assert!(Journal::open(&path).is_err());
    Ok(())
}

#[test]
fn source_review_prevents_home_policy_overclaims_and_recall_data_loss() -> TestResult {
    let catalog = load_embedded()?;
    let documentation = include_str!("../../../docs/dev/catalog-sources.md");
    for item in catalog.items() {
        assert!(documentation.contains(&format!("`{}`", item.id)));
        if item.verified {
            assert!(!item.sources.is_empty());
            for source in &item.sources {
                assert!(
                    source.starts_with("https://learn.microsoft.com/")
                        || source.starts_with("https://github.com/The-Virtual-Desktop-Team/")
                );
                assert!(documentation.contains(source));
            }
        }
    }
    for id in [
        "reg.copilot",
        "reg.widgets",
        "reg.advertising",
        "reg.telemetry",
        "reg.consumer_features",
    ] {
        assert!(!catalog.get(id)?.supported_editions.contains(&Edition::Home));
    }
    for id in [
        "svc.sysmain",
        "svc.dosvc",
        "svc.wersvc",
        "svc.remoteregistry",
        "svc.diagnosticshub.standardcollector.service",
    ] {
        assert!(!catalog.get(id)?.automatic);
    }
    let recall = catalog.get("reg.recall")?;
    assert!(protection::is_protected(recall));
    assert_eq!(recall.revert, Revert::Manual);
    assert_eq!(recall.risk, Risk::High);
    Ok(())
}
