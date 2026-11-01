#![forbid(unsafe_code)]
use gametune_core::temporary::*;
use gametune_core::*;
use gametune_exec_mock::{MockExecutor, MockHost};
type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;
fn machine() -> Machine {
    Machine {
        build: 26200,
        edition: Edition::Pro,
        disk: DiskKind::Unknown,
    }
}
#[test]
fn temporary_pause_preserves_start_mode_and_reverts_exact_snapshot() -> TestResult {
    let catalog = gametune_catalog::load_embedded()?;
    let actions = gametune_catalog::game_actions(&catalog)?;
    let item = catalog.get("svc.diagtrack")?;
    let host = MockHost::default();
    host.seed(
        item,
        Snapshot::Service {
            start_mode: 2,
            delayed_auto_start: Some(true),
            running: true,
        },
    )?;
    let before = host.snapshot()?;
    let mut executors = Executors::default();
    executors.insert(MockExecutor::new(ActionType::Service, host.clone()));
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    let directory = tempfile::tempdir()?;
    let mut wal = Journal::open(directory.path().join("games.wal"))?;
    reconcile(
        &mut engine,
        &mut wal,
        &actions,
        std::slice::from_ref(&item.id),
    )?;
    assert_eq!(
        host.snapshot()?.get(&item.target.key()),
        Some(&Snapshot::Service {
            start_mode: 2,
            delayed_auto_start: Some(true),
            running: false
        })
    );
    let writes = host.writes()?;
    reconcile(
        &mut engine,
        &mut wal,
        &actions,
        std::slice::from_ref(&item.id),
    )?;
    assert_eq!(writes, host.writes()?);
    reconcile(&mut engine, &mut wal, &actions, &[])?;
    assert_eq!(before, host.snapshot()?);
    Ok(())
}
#[test]
fn temporary_original_is_current_state_not_permanent_original() -> TestResult {
    let catalog = gametune_catalog::load_embedded()?;
    let actions = gametune_catalog::game_actions(&catalog)?;
    let item = catalog.get("reg.game_mode")?;
    let host = MockHost::default();
    host.seed(
        item,
        Snapshot::Registry {
            value: Some(RegistryValue {
                kind: 4,
                bytes: 0u32.to_le_bytes().to_vec(),
            }),
            created_keys: vec![],
        },
    )?;
    let before = host.snapshot()?;
    let mut executors = Executors::default();
    executors.insert(MockExecutor::new(ActionType::Registry, host.clone()));
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    let directory = tempfile::tempdir()?;
    let mut permanent = Journal::open(directory.path().join("permanent.wal"))?;
    engine.apply(
        &Request {
            item_ids: vec![item.id.clone()],
            ..Request::default()
        },
        &mut permanent,
    )?;
    let tuned = host.snapshot()?;
    let mut temporary = Journal::open(directory.path().join("temporary.wal"))?;
    reconcile(
        &mut engine,
        &mut temporary,
        &actions,
        std::slice::from_ref(&item.id),
    )?;
    reconcile(&mut engine, &mut temporary, &actions, &[])?;
    assert_eq!(host.snapshot()?, tuned);
    engine.undo(&[], &mut permanent)?;
    assert_eq!(host.snapshot()?, before);
    Ok(())
}
#[test]
fn unknown_or_permanent_ids_cannot_enter_temporary_journal() -> TestResult {
    let catalog = gametune_catalog::load_embedded()?;
    let actions = gametune_catalog::game_actions(&catalog)?;
    let mut executors = Executors::default();
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    let directory = tempfile::tempdir()?;
    let mut wal = Journal::open(directory.path().join("games.wal"))?;
    for id in [
        "unknown",
        "reg.cortana",
        "appx.microsoft.gamingservices",
        "svc.xblauthmanager",
    ] {
        assert!(reconcile(&mut engine, &mut wal, &actions, &[id.into()]).is_err());
    }
    assert_eq!(wal.entries().count(), 0);
    Ok(())
}
struct FileService {
    path: std::path::PathBuf,
    crash: bool,
}
impl Executor for FileService {
    fn action_type(&self) -> ActionType {
        ActionType::Service
    }
    fn capture(&self, _: &Item) -> ExecResult<Snapshot> {
        serde_json::from_slice(
            &std::fs::read(&self.path).map_err(|e| ExecError::Failed(e.to_string()))?,
        )
        .map_err(|e| ExecError::Failed(e.to_string()))
    }
    fn apply(&mut self, _: &Item) -> ExecResult<()> {
        Err(ExecError::Protected)
    }
    fn apply_temporary(&mut self, item: &Item) -> ExecResult<()> {
        let mut snapshot = self.capture(item)?;
        if let Snapshot::Service { running, .. } = &mut snapshot {
            *running = false;
        }
        self.restore(item, &snapshot)?;
        if self.crash {
            std::process::exit(73);
        }
        Ok(())
    }
    fn restore(&mut self, _: &Item, snapshot: &Snapshot) -> ExecResult<()> {
        std::fs::write(
            &self.path,
            serde_json::to_vec(snapshot).map_err(|e| ExecError::Failed(e.to_string()))?,
        )
        .map_err(|e| ExecError::Failed(e.to_string()))
    }
}
#[test]
fn crash_child() -> TestResult {
    if std::env::var("GAMETUNE_SYNTHETIC_CRASH").as_deref() != Ok("MOCK_ONLY") {
        return Ok(());
    }
    let directory = std::path::PathBuf::from(std::env::var("GAMETUNE_SESSION_TEST_DIR")?);
    if !directory.starts_with(std::env::temp_dir())
        || !directory
            .file_name()
            .is_some_and(|s| s.to_string_lossy().starts_with("gametune-session-test-"))
    {
        return Err("invalid mock sandbox".into());
    }
    let catalog = gametune_catalog::load_embedded()?;
    let actions = gametune_catalog::game_actions(&catalog)?;
    let mut executors = Executors::default();
    executors.insert(FileService {
        path: directory.join("state.json"),
        crash: true,
    });
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    let mut wal = Journal::open(directory.join("games.wal"))?;
    reconcile(&mut engine, &mut wal, &actions, &["svc.diagtrack".into()])?;
    Err("child did not crash".into())
}
#[test]
fn actual_child_crash_between_write_ahead_and_commit_recovers() -> TestResult {
    let directory = tempfile::Builder::new()
        .prefix("gametune-session-test-")
        .tempdir()?;
    let original = Snapshot::Service {
        start_mode: 2,
        delayed_auto_start: Some(true),
        running: true,
    };
    let bytes = serde_json::to_vec(&original)?;
    std::fs::write(directory.path().join("state.json"), &bytes)?;
    let status = std::process::Command::new(std::env::current_exe()?)
        .args(["--exact", "crash_child", "--nocapture"])
        .env("GAMETUNE_SYNTHETIC_CRASH", "MOCK_ONLY")
        .env("GAMETUNE_SESSION_TEST_DIR", directory.path())
        .status()?;
    assert_eq!(status.code(), Some(73));
    let mut wal = Journal::open(directory.path().join("games.wal"))?;
    assert!(wal.needs_recovery());
    assert_eq!(
        wal.entries().next().ok_or("no prepared entry")?.status,
        Status::Prepared
    );
    let catalog = gametune_catalog::load_embedded()?;
    let actions = gametune_catalog::game_actions(&catalog)?;
    let mut executors = Executors::default();
    executors.insert(FileService {
        path: directory.path().join("state.json"),
        crash: false,
    });
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    reconcile(&mut engine, &mut wal, &actions, &[])?;
    assert_eq!(bytes, std::fs::read(directory.path().join("state.json"))?);
    assert!(active_ids(&wal).is_empty());
    Ok(())
}
