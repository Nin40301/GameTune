#![forbid(unsafe_code)]
use gametune_watcher::{storage::*, *};
type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;
#[test]
fn library_is_atomic_validated_and_locked_while_monitor_runs() -> TestResult {
    let d = tempfile::tempdir()?;
    let mut library = Library::default();
    library
        .games
        .push(Game::manual(r"C:\Games\Test.exe".into())?);
    save_library(d.path(), &library)?;
    assert_eq!(load(d.path())?.games.len(), 1);
    let lease = lock(d.path())?;
    assert!(matches!(save_library(d.path(), &library), Err(Error::Busy)));
    drop(lease);
    library.games[0].action_ids = vec!["svc.xblauthmanager".into()];
    assert!(save_library(d.path(), &library).is_err());
    assert!(load(d.path())?.games[0].action_ids.is_empty());
    Ok(())
}
#[test]
fn recovery_hint_survives_restart_and_stale_pid_is_not_running() -> TestResult {
    let d = tempfile::tempdir()?;
    save(&d.path().join("recovery-pending.json"), &true)?;
    save(
        &d.path().join("monitor-status.json"),
        &MonitorStatus {
            running: true,
            pid: 123,
            instance: "old".into(),
            ..MonitorStatus::default()
        },
    )?;
    let view = status(d.path());
    assert!(!view.running);
    assert!(view.recovery_pending);
    save(&d.path().join("recovery-pending.json"), &false)?;
    assert!(!status(d.path()).recovery_pending);
    Ok(())
}
#[test]
fn corrupt_editable_library_does_not_block_recovery_or_authorize_new_actions() -> TestResult {
    let d = tempfile::tempdir()?;
    std::fs::write(d.path().join("games.json"), b"not valid JSON")?;
    assert!(load_for_monitor(d.path()).is_err());
    save(&d.path().join("recovery-pending.json"), &true)?;
    assert!(load_for_monitor(d.path())?.allowed_ids().is_empty());
    assert!(load(d.path()).is_err());
    assert_eq!(
        std::fs::read(d.path().join("games.json"))?,
        b"not valid JSON"
    );
    Ok(())
}
