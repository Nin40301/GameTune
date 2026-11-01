#![cfg(all(windows, feature = "vm-tests"))]
use ametista_core::*;
use ametista_windows_runtime::fixture::VmFixture;
type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

#[test]
#[ignore = "requires an elevated interactive shell in a disposable Windows VM with a checkpoint"]
fn dedicated_windows_apply_undo_matches_complete_snapshot() -> TestResult {
    let fixture = VmFixture::create()?;
    let evidence = std::env::current_dir()?
        .join("work/windows-vm")
        .join(fixture.ids[0].replace('.', "_"));
    std::fs::create_dir_all(&evidence)?;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> TestResult {
        let before = fixture.snapshot()?;
        std::fs::write(
            evidence.join("before.json"),
            serde_json::to_vec_pretty(&before)?,
        )?;
        let mut executors = fixture.context.executors();
        let mut engine = Engine {
            catalog: &fixture.catalog,
            machine: fixture.context.machine(),
            executors: &mut executors,
        };
        let request = fixture.request();
        let dry = engine.run(&request)?;
        assert!(
            dry.items.iter().all(|r| r.outcome == Outcome::Planned),
            "{dry:?}"
        );
        assert_eq!(before, fixture.snapshot()?, "dry-run mutated Windows");
        let path = evidence.join("journal.wal");
        let mut journal = Journal::open_scoped(&path, fixture.context.scope())?;
        let applied = engine.apply(&request, &mut journal)?;
        assert!(
            applied.items.iter().all(|r| r.outcome == Outcome::Applied),
            "{applied:?}"
        );
        let after = fixture.snapshot()?;
        std::fs::write(
            evidence.join("after.json"),
            serde_json::to_vec_pretty(&after)?,
        )?;
        assert_ne!(before, after);
        for key in ["registry", "service", "task", "startup", "power"] {
            assert_ne!(before[key], after[key], "{key} did not change");
        }
        drop(journal);
        let mut journal = Journal::open_scoped(&path, fixture.context.scope())?;
        let undone = engine.undo(&[], &mut journal)?;
        assert!(
            undone.items.iter().all(|r| r.outcome == Outcome::Reverted),
            "{undone:?}"
        );
        let restored = fixture.snapshot()?;
        for (name, value) in [
            ("before", &before),
            ("after", &after),
            ("restored", &restored),
        ] {
            std::fs::write(
                evidence.join(format!("{name}.json")),
                serde_json::to_vec_pretty(value)?,
            )?;
        }
        assert_eq!(before, restored, "complete configuration snapshot mismatch");
        assert!(engine.undo(&[], &mut journal)?.items.is_empty());
        Ok(())
    }));
    let cleanup = fixture.cleanup();
    match result {
        Ok(result) => result?,
        Err(payload) => std::panic::resume_unwind(payload),
    }
    cleanup?;
    Ok(())
}

#[test]
#[ignore = "requires an elevated interactive disposable VM; creates only a dedicated protected test store"]
fn dedicated_store_has_admin_system_acl() -> TestResult {
    let fixture = VmFixture::create()?;
    let result = fixture.test_protected_store();
    let store_cleanup = fixture.cleanup_test_store();
    let cleanup = fixture.cleanup();
    let directory = result?;
    assert!(
        directory
            .as_str()
            .is_some_and(|s| s.contains("AmetistaTestStore"))
    );
    cleanup?;
    store_cleanup?;
    Ok(())
}
