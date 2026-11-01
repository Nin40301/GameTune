use gametune_catalog::load_embedded;
use gametune_core::*;
use gametune_exec_mock::{MockExecutor, MockHost};

type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;
fn machine() -> Machine {
    Machine {
        build: 26100,
        edition: Edition::Pro,
        disk: DiskKind::Ssd,
    }
}
fn verified_catalog() -> Result<Catalog> {
    let items = load_embedded()?
        .items()
        .cloned()
        .map(|mut i| {
            i.verified = true;
            i.sources = vec!["https://example.test/fixture-not-evidence".into()];
            i
        })
        .collect();
    Catalog::try_new(items)
}

#[test]
fn risk_and_level_filters_hold_independently_of_verification() -> TestResult {
    let catalog = verified_catalog()?;
    for preset in [Preset::Safe, Preset::Balanced, Preset::Aggressive] {
        let ids = preset.select(&catalog, machine());
        for id in &ids {
            let item = catalog.get(id)?;
            assert_ne!(item.risk, Risk::High);
            assert_ne!(item.level, 0);
            assert_eq!(item.revert, Revert::Auto);
            assert!(!gametune_core::protection::is_protected(item));
        }
    }
    let safe = Preset::Safe.select(&catalog, machine());
    let balanced = Preset::Balanced.select(&catalog, machine());
    let aggressive = Preset::Aggressive.select(&catalog, machine());
    assert!(safe.contains(&"svc.diagtrack".into()));
    assert!(!safe.contains(&"svc.mapsbroker".into()));
    assert!(balanced.contains(&"svc.mapsbroker".into()));
    assert!(!balanced.contains(&"svc.wsearch".into()));
    assert!(aggressive.contains(&"svc.wsearch".into()));
    Ok(())
}

#[test]
fn network_conflicts_in_appendix_never_execute_even_with_manual_confirmation() -> TestResult {
    let catalog = verified_catalog()?;
    let host = MockHost::default();
    let mut executors = Executors::default();
    executors.insert(MockExecutor::new(ActionType::Service, host.clone()));
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    let ids: Vec<String> = vec![
        "svc.icssvc".into(),
        "svc.wmpnetworksvc".into(),
        "svc.xboxnetapisvc".into(),
    ];
    let request = Request {
        item_ids: ids.clone(),
        high_risk_confirmations: ids.into_iter().collect(),
        ..Request::default()
    };
    let dry = engine.run(&request)?;
    assert!(
        dry.items
            .iter()
            .all(|i| i.error == Some(ExecError::Protected))
    );
    let dir = tempfile::tempdir()?;
    let mut journal = Journal::open(dir.path().join("journal.wal"))?;
    let real = engine.apply(&request, &mut journal)?;
    assert!(
        real.items
            .iter()
            .all(|i| i.error == Some(ExecError::Protected))
    );
    assert_eq!(journal.entries().count(), 0);
    assert_eq!(host.writes()?, 0);
    Ok(())
}

#[test]
fn high_risk_requires_confirmation_even_when_verified() -> TestResult {
    let catalog = verified_catalog()?;
    let mut executors = Executors::default();
    let engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    let request = Request {
        item_ids: vec!["svc.xblauthmanager".into()],
        ..Request::default()
    };
    assert!(matches!(
        engine.run(&request),
        Err(Error::ConfirmationRequired(_))
    ));
    Ok(())
}

#[test]
fn windows_10_only_policy_does_not_enter_windows_11_profiles() -> TestResult {
    let catalog = load_embedded()?;
    let ten = Machine {
        build: 19045,
        ..machine()
    };
    assert!(
        Preset::Safe
            .select(&catalog, ten)
            .contains(&"reg.game_dvr_policy".into())
    );
    assert!(
        !Preset::Safe
            .select(&catalog, machine())
            .contains(&"reg.game_dvr_policy".into())
    );
    let mut executors = Executors::default();
    let engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    assert!(matches!(
        engine.run(&Request {
            item_ids: vec!["reg.game_dvr_policy".into()],
            ..Request::default()
        }),
        Err(Error::Incompatible(_))
    ));
    Ok(())
}

#[test]
fn simulation_and_apply_share_failures_and_do_not_overpromise_appx() -> TestResult {
    let catalog = load_embedded()?;
    let host = MockHost::default();
    let mut executors = Executors::default();
    executors.insert(MockExecutor::new(ActionType::Service, host));
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    let ids = vec!["svc.diagtrack".into(), "appx.microsoft.bingnews".into()];
    let request = Request {
        unverified_confirmations: ids.iter().cloned().collect(),
        item_ids: ids,
        ..Request::default()
    };
    let dry = engine.run(&request)?;
    let dir = tempfile::tempdir()?;
    let mut journal = Journal::open(dir.path().join("journal.wal"))?;
    let apply = engine.apply(&request, &mut journal)?;
    for (a, b) in dry.items.iter().zip(apply.items.iter()) {
        assert_eq!(a.item_id, b.item_id);
        assert_eq!(a.outcome, b.outcome);
        assert_eq!(a.error, b.error);
    }
    assert_eq!(journal.entries().count(), 0);
    Ok(())
}

#[test]
fn reapply_after_undo_captures_new_original_instead_of_stale_history() -> TestResult {
    let catalog = load_embedded()?;
    let item = catalog.get("reg.cortana")?;
    let host = MockHost::default();
    host.seed(
        item,
        Snapshot::Registry {
            value: None,
            created_keys: vec![],
        },
    )?;
    let mut executors = Executors::default();
    executors.insert(MockExecutor::new(ActionType::Registry, host.clone()));
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("journal.wal");
    let mut journal = Journal::open(&path)?;
    let request = Request {
        item_ids: vec![item.id.clone()],
        ..Request::default()
    };
    engine.apply(&request, &mut journal)?;
    engine.undo(&[], &mut journal)?;
    let changed = Snapshot::Registry {
        value: Some(RegistryValue {
            kind: 11,
            bytes: 123456789_u64.to_le_bytes().to_vec(),
        }),
        created_keys: vec![],
    };
    host.seed(item, changed.clone())?;
    engine.apply(&request, &mut journal)?;
    drop(journal);
    let mut reopened = Journal::open(path)?;
    assert_eq!(
        reopened.entry(&item.target.key()).ok_or("entry")?.original,
        changed
    );
    engine.undo(&[], &mut reopened)?;
    assert_eq!(host.snapshot()?.get(&item.target.key()), Some(&changed));
    Ok(())
}

#[test]
fn checksum_detects_modified_complete_record_and_retains_bytes() -> TestResult {
    let catalog = load_embedded()?;
    let item = catalog.get("reg.cortana")?;
    let host = MockHost::default();
    host.seed(
        item,
        Snapshot::Registry {
            value: None,
            created_keys: vec![],
        },
    )?;
    let mut executors = Executors::default();
    executors.insert(MockExecutor::new(ActionType::Registry, host));
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("journal.wal");
    let mut journal = Journal::open(&path)?;
    Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    }
    .apply(
        &Request {
            item_ids: vec![item.id.clone()],
            ..Request::default()
        },
        &mut journal,
    )?;
    drop(journal);
    let text = std::fs::read_to_string(&path)?;
    let changed = text.replacen("prepared", "applied", 1);
    std::fs::write(&path, &changed)?;
    assert!(matches!(
        Journal::open(&path),
        Err(Error::CorruptJournal(_))
    ));
    assert_eq!(std::fs::read_to_string(&path)?, changed);
    Ok(())
}
