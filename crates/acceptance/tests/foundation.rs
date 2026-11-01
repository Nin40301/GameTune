use gametune_catalog::{
    EMBEDDED_ITEMS, load_embedded, validate_catalog_json, validate_profile_json, verify_hash,
};
use gametune_core::*;
use gametune_exec_mock::{MockHost, Stage};
use serde_json::json;
use std::fs;

type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

fn machine() -> Machine {
    Machine {
        build: 26100,
        edition: Edition::Pro,
        disk: DiskKind::Ssd,
    }
}
fn manual(ids: &[&str]) -> Request {
    Request {
        item_ids: ids.iter().map(|s| (*s).into()).collect(),
        high_risk_confirmations: ids.iter().map(|s| (*s).into()).collect(),
        unverified_confirmations: ids.iter().map(|s| (*s).into()).collect(),
    }
}
fn original(item: &Item) -> Snapshot {
    match item.action_type {
        ActionType::Registry => Snapshot::Registry {
            value: Some(RegistryValue {
                kind: 3,
                bytes: vec![0, 255, 128, 7],
            }),
            created_keys: vec![],
        },
        ActionType::Service => Snapshot::Service {
            start_mode: 2,
            delayed_auto_start: Some(true),
            running: true,
        },
        ActionType::ScheduledTask => Snapshot::ScheduledTask {
            enabled: true,
            definition: None,
        },
        ActionType::Appx => Snapshot::Appx {
            packages: vec!["package_1.0_x64".into()],
            provisioned: vec!["provisioned".into()],
            families: vec!["family".into()],
        },
        ActionType::Startup => Snapshot::Startup {
            approved: Some(RegistryValue {
                kind: 3,
                bytes: vec![2, 0, 0, 0, 12, 34, 56, 78, 0, 0, 0, 0],
            }),
            created_keys: vec![],
        },
        ActionType::Power => Snapshot::Power {
            active_guid: "381b4222-f694-41f0-9685-ff5bb260df2e".into(),
            existing_guids: vec!["381b4222-f694-41f0-9685-ff5bb260df2e".into()],
        },
    }
}
fn executors(host: &MockHost) -> Executors {
    let mut executors = Executors::default();
    executors.insert(gametune_exec_registry::MockExecutor::new(host.clone()));
    executors.insert(gametune_exec_service::MockExecutor::new(host.clone()));
    executors.insert(gametune_exec_scheduled_task::MockExecutor::new(
        host.clone(),
    ));
    executors.insert(gametune_exec_appx::MockExecutor::new(host.clone()));
    executors.insert(gametune_exec_startup::MockExecutor::new(host.clone()));
    executors.insert(gametune_exec_power::MockExecutor::new(host.clone()));
    executors
}

#[test]
fn embedded_catalog_is_complete_and_schema_valid() -> TestResult {
    let catalog = load_embedded()?;
    assert_eq!(catalog.items().count(), 86);
    assert!(catalog.items().filter(|i| i.verified).count() >= 68);
    assert!(catalog.get("svc.diagtrack").is_ok());
    assert!(catalog.get("appx.microsoft.gamingservices").is_ok());
    Ok(())
}

#[test]
fn schema_rejects_unknown_fields_wrong_target_and_missing_translation() -> TestResult {
    for mutation in 0..4 {
        let mut value: serde_json::Value = serde_json::from_str(EMBEDDED_ITEMS)?;
        match mutation {
            0 => value["items"][0]["command"] = json!("calc.exe"),
            1 => value["items"][0]["target"]["value"] = json!("1"),
            2 => {
                value["items"][0]["name"]
                    .as_object_mut()
                    .ok_or("object")?
                    .remove("en");
            }
            _ => value["items"][0]["type"] = json!("service"),
        }
        assert!(validate_catalog_json(&value.to_string()).is_err());
    }
    Ok(())
}

#[test]
fn embedded_hash_detects_tampering() {
    assert!(verify_hash(EMBEDDED_ITEMS.as_bytes(), &"0".repeat(64)).is_err());
}

#[test]
fn catalog_rejects_duplicate_ids_and_duplicate_targets() -> TestResult {
    let item = load_embedded()?.get("reg.cortana")?.clone();
    assert!(Catalog::try_new(vec![item.clone(), item.clone()]).is_err());
    let mut other = item.clone();
    other.id = "another.id".into();
    assert!(Catalog::try_new(vec![item, other]).is_err());
    Ok(())
}

#[test]
fn protected_services_are_rejected_even_with_innocent_ids_and_case_changes() -> TestResult {
    let template = load_embedded()?.get("svc.diagtrack")?.clone();
    for name in [
        "WinDefend",
        "WdNisSvc",
        "SecurityHealthService",
        "wuauserv",
        "UsoSvc",
        "WaaSMedicSvc",
        "BFE",
        "MpsSvc",
        "mpssvc",
        "Dhcp",
        "Dnscache",
        "NlaSvc",
        "LanmanWorkstation",
        "Audiosrv",
        "AudioEndpointBuilder",
        "Power",
        "LSM",
        "SamSs",
        "vgc",
        "BEService",
        "EasyAntiCheat",
        "nvlddmkm",
    ] {
        let mut item = template.clone();
        item.target = Target::Service {
            name: name.into(),
            mode: ServiceMode::Disabled,
        };
        assert!(Catalog::try_new(vec![item]).is_err(), "{name}");
    }
    Ok(())
}

#[test]
fn protected_registry_and_packages_are_rejected() -> TestResult {
    let catalog = load_embedded()?;
    for path in [
        r"SOFTWARE\Policies\Microsoft\Windows Defender",
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
        r"SYSTEM\CurrentControlSet\Control\DeviceGuard",
        r"SYSTEM\CurrentControlSet\Control\CI",
        r"SYSTEM\CurrentControlSet\Control\SecureBoot",
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
        r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate",
    ] {
        let mut item = catalog.get("reg.cortana")?.clone();
        item.target = Target::Registry {
            hive: Hive::HKLM,
            path: path.into(),
            name: "Disabled".into(),
            value: 1,
        };
        assert!(Catalog::try_new(vec![item]).is_err());
    }
    for name in [
        "Microsoft.WindowsStore",
        "Microsoft.StorePurchaseApp",
        "Microsoft.MicrosoftEdge",
        "Microsoft.SecHealthUI",
    ] {
        let mut item = catalog.get("appx.microsoft.bingnews")?.clone();
        item.target = Target::Appx { name: name.into() };
        assert!(Catalog::try_new(vec![item]).is_err());
    }
    Ok(())
}

#[test]
fn only_approved_edge_preload_and_delivery_optimization_targets_are_allowed() -> TestResult {
    let catalog = load_embedded()?;
    let mut edge = catalog.get("reg.edge_preload")?.clone();
    if let Target::Registry { name, .. } = &mut edge.target {
        *name = "BrowserSignin".into();
    }
    assert!(Catalog::try_new(vec![edge]).is_err());
    let mut dosvc = catalog.get("svc.dosvc")?.clone();
    if let Target::Service { mode, .. } = &mut dosvc.target {
        *mode = ServiceMode::Disabled;
    }
    assert!(Catalog::try_new(vec![dosvc]).is_err());
    Ok(())
}

#[test]
fn safety_metadata_cannot_be_downgraded() -> TestResult {
    let catalog = load_embedded()?;
    for id in [
        "svc.xblauthmanager",
        "svc.xblgamesave",
        "svc.xboxnetapisvc",
        "svc.xboxgipsvc",
        "appx.microsoft.gamingservices",
        "appx.microsoft.xboxidentityprovider",
        "reg.background_apps",
    ] {
        let mut item = catalog.get(id)?.clone();
        item.risk = Risk::Low;
        assert!(Catalog::try_new(vec![item]).is_err(), "{id}");
    }
    let mut sysmain = catalog.get("svc.sysmain")?.clone();
    sysmain.requires_ssd = false;
    assert!(Catalog::try_new(vec![sysmain]).is_err());
    let mut hags = catalog.get("reg.hags")?.clone();
    hags.requires_reboot = false;
    assert!(Catalog::try_new(vec![hags]).is_err());
    let mut app = catalog.get("appx.microsoft.bingnews")?.clone();
    app.revert = Revert::Auto;
    assert!(Catalog::try_new(vec![app]).is_err());
    Ok(())
}

#[test]
fn automatic_profiles_filter_build_edition_risk_verification_and_manual_revert() -> TestResult {
    let catalog = load_embedded()?;
    for preset in [Preset::Safe, Preset::Balanced, Preset::Aggressive] {
        let selected = preset.select(&catalog, machine());
        assert!(!selected.is_empty());
        for id in selected {
            let item = catalog.get(&id)?;
            assert!(
                item.verified
                    && item.risk != Risk::High
                    && item.level > 0
                    && item.revert == Revert::Auto
            );
        }
        assert!(
            preset
                .select(
                    &catalog,
                    Machine {
                        build: 19044,
                        ..machine()
                    }
                )
                .is_empty()
        );
        let home = preset.select(
            &catalog,
            Machine {
                edition: Edition::Home,
                ..machine()
            },
        );
        assert!(!home.is_empty());
        for id in home {
            if let Target::Registry { path, .. } = &catalog.get(&id)?.target {
                assert_eq!(path, "SOFTWARE\\Policies\\Microsoft\\Edge");
            }
        }
    }
    Ok(())
}

#[test]
fn sysmain_is_preselected_only_when_verified_and_disk_is_ssd() -> TestResult {
    let mut item = load_embedded()?.get("svc.sysmain")?.clone();
    item.verified = true;
    item.automatic = true; // synthetic fixture isolates SSD gating; real SysMain is manual-only
    item.sources = vec!["https://example.test/test-fixture".into()];
    let catalog = Catalog::try_new(vec![item])?;
    assert!(Preset::Safe.select(&catalog, machine()).is_empty());
    assert_eq!(
        Preset::Balanced.select(&catalog, machine()),
        vec!["svc.sysmain"]
    );
    for disk in [DiskKind::Hdd, DiskKind::Unknown] {
        assert!(
            Preset::Aggressive
                .select(&catalog, Machine { disk, ..machine() })
                .is_empty()
        );
    }
    Ok(())
}

#[test]
fn import_accepts_only_catalog_ids_and_no_authorization_or_commands() -> TestResult {
    let catalog = load_embedded()?;
    let valid = r#"{"version":1,"name":"Meu perfil","itemIds":["reg.cortana"]}"#;
    validate_profile_json(valid)?;
    assert_eq!(
        Profile::import(valid, &catalog)?.item_ids(),
        &["reg.cortana"]
    );
    let exported = Profile::import(valid, &catalog)?.export()?;
    validate_profile_json(&exported)?;
    for value in [
        json!({"version":1,"name":"x","itemIds":["unknown"]}),
        json!({"version":1,"name":"x","itemIds":["reg.cortana","reg.cortana"]}),
        json!({"version":1,"name":"x","itemIds":[],"command":"whoami"}),
        json!({"version":1,"name":"x","itemIds":[],"highRiskConfirmations":["svc.xblauthmanager"]}),
        json!({"version":2,"name":"x","itemIds":[]}),
    ] {
        assert!(Profile::import(&value.to_string(), &catalog).is_err());
        assert!(validate_profile_json(&value.to_string()).is_err());
    }
    assert!(Profile::import(&" ".repeat(65537), &catalog).is_err());
    Ok(())
}

#[test]
fn dry_run_leaves_all_mock_state_and_files_unchanged() -> TestResult {
    let catalog = load_embedded()?;
    let host = MockHost::default();
    let ids = [
        "reg.cortana",
        "svc.diagtrack",
        "task.consolidator",
        "startup.hkcu",
        "power.high_performance",
    ];
    for id in ids {
        let item = catalog.get(id)?;
        host.seed(item, original(item))?;
    }
    let before = host.snapshot()?;
    let dir = tempfile::tempdir()?;
    let mut executors = executors(&host);
    let engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    let report = engine.run(&manual(&ids))?;
    assert!(report.dry_run);
    assert!(report.items.iter().all(|i| i.outcome == Outcome::Planned));
    assert_eq!(before, host.snapshot()?);
    assert_eq!(host.writes()?, 0);
    assert_eq!(fs::read_dir(dir.path())?.count(), 0);
    Ok(())
}

#[test]
fn apply_undo_all_reversible_types_restores_exact_snapshot_and_order() -> TestResult {
    let catalog = load_embedded()?;
    let host = MockHost::default();
    let ids = [
        "reg.cortana",
        "svc.diagtrack",
        "task.consolidator",
        "startup.hkcu",
        "power.high_performance",
    ];
    for id in ids {
        let item = catalog.get(id)?;
        host.seed(item, original(item))?;
    }
    let before = host.snapshot()?;
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("journal.wal");
    let mut journal = Journal::open(&path)?;
    let mut executors = executors(&host);
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    let applied = engine.apply(&manual(&ids), &mut journal)?;
    assert!(applied.items.iter().all(|i| i.outcome == Outcome::Applied));
    assert_ne!(before, host.snapshot()?);
    let undone = engine.undo(&[], &mut journal)?;
    assert_eq!(
        undone
            .items
            .iter()
            .map(|r| r.item_id.as_str())
            .collect::<Vec<_>>(),
        ids.into_iter().rev().collect::<Vec<_>>()
    );
    assert!(undone.items.iter().all(|i| i.outcome == Outcome::Reverted));
    assert_eq!(before, host.snapshot()?);
    assert!(journal.entries().all(|e| e.status == Status::Reverted));
    assert!(!journal.needs_recovery());
    assert!(engine.undo(&[], &mut journal)?.items.is_empty());
    let projection: serde_json::Value = serde_json::from_str(&journal.export_json()?)?;
    assert_eq!(projection.as_array().ok_or("array")?.len(), 5);
    drop(journal);
    assert!(
        Journal::open(path)?
            .entries()
            .all(|e| e.status == Status::Reverted)
    );
    Ok(())
}

#[test]
fn second_apply_preserves_first_original_and_does_not_write() -> TestResult {
    let catalog = load_embedded()?;
    let item = catalog.get("reg.cortana")?;
    let host = MockHost::default();
    host.seed(item, original(item))?;
    let dir = tempfile::tempdir()?;
    let mut journal = Journal::open(dir.path().join("journal.wal"))?;
    let mut executors = executors(&host);
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    let request = manual(&["reg.cortana", "reg.cortana"]);
    assert_eq!(engine.apply(&request, &mut journal)?.items.len(), 1);
    let first = journal.export_json()?;
    assert_eq!(
        engine.apply(&request, &mut journal)?.items[0].outcome,
        Outcome::AlreadyApplied
    );
    assert_eq!(host.writes()?, 1);
    assert_eq!(first, journal.export_json()?);
    engine.undo(&[], &mut journal)?;
    assert_eq!(
        host.snapshot()?.get(&item.target.key()),
        Some(&original(item))
    );
    Ok(())
}

#[test]
fn absent_registry_value_and_absent_startup_approval_are_restored() -> TestResult {
    let catalog = load_embedded()?;
    let host = MockHost::default();
    host.seed(
        catalog.get("reg.cortana")?,
        Snapshot::Registry {
            value: None,
            created_keys: vec!["parent".into(), "leaf".into()],
        },
    )?;
    host.seed(
        catalog.get("startup.hkcu")?,
        Snapshot::Startup {
            approved: None,
            created_keys: vec![],
        },
    )?;
    let before = host.snapshot()?;
    let dir = tempfile::tempdir()?;
    let mut journal = Journal::open(dir.path().join("journal.wal"))?;
    let mut executors = executors(&host);
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    engine.apply(&manual(&["reg.cortana", "startup.hkcu"]), &mut journal)?;
    engine.undo(&[], &mut journal)?;
    assert_eq!(before, host.snapshot()?);
    Ok(())
}

#[test]
fn unknown_late_id_and_missing_confirmations_abort_before_first_write() -> TestResult {
    let catalog = load_embedded()?;
    let host = MockHost::default();
    let item = catalog.get("reg.cortana")?;
    host.seed(item, original(item))?;
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("journal.wal");
    let mut journal = Journal::open(&path)?;
    let mut executors = executors(&host);
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    assert!(matches!(
        engine.apply(&manual(&["reg.cortana", "missing"]), &mut journal),
        Err(Error::UnknownItem(_))
    ));
    for id in ["reg.game_dvr_user", "svc.xblauthmanager"] {
        let request = Request {
            item_ids: vec![id.into()],
            ..Request::default()
        };
        assert!(matches!(
            engine.run(&request),
            Err(Error::ConfirmationRequired(_))
        ));
    }
    assert_eq!(host.writes()?, 0);
    assert_eq!(journal.entries().count(), 0);
    Ok(())
}

#[test]
fn missing_resources_and_denied_access_do_not_abort_batch() -> TestResult {
    let catalog = load_embedded()?;
    let host = MockHost::default();
    let ids = [
        "svc.diagtrack",
        "task.consolidator",
        "appx.microsoft.bingnews",
        "reg.cortana",
        "reg.game_mode",
    ];
    for id in [&ids[3], &ids[4]] {
        let item = catalog.get(id)?;
        host.seed(item, original(item))?;
    }
    host.fail(
        catalog.get(ids[3])?,
        Stage::Capture,
        ExecError::AccessDenied,
    )?;
    let dir = tempfile::tempdir()?;
    let mut journal = Journal::open(dir.path().join("journal.wal"))?;
    let mut executors = executors(&host);
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    let report = engine.apply(&manual(&ids), &mut journal)?;
    assert_eq!(report.items.len(), 5);
    assert_eq!(report.items[0].error, Some(ExecError::NotFound));
    assert_eq!(report.items[1].error, Some(ExecError::NotFound));
    assert_eq!(
        report.items[2].error,
        Some(ExecError::ManualReinstallRequired)
    );
    assert_eq!(report.items[3].error, Some(ExecError::AccessDenied));
    assert_eq!(report.items[4].outcome, Outcome::Applied);
    Ok(())
}

#[test]
fn partial_failure_keeps_original_continues_batch_and_can_undo() -> TestResult {
    let catalog = load_embedded()?;
    let host = MockHost::default();
    let ids = ["svc.diagtrack", "reg.cortana"];
    for id in ids {
        let item = catalog.get(id)?;
        host.seed(item, original(item))?;
    }
    let before = host.snapshot()?;
    host.fail(
        catalog.get(ids[0])?,
        Stage::AfterApply,
        ExecError::AccessDenied,
    )?;
    let dir = tempfile::tempdir()?;
    let mut journal = Journal::open(dir.path().join("journal.wal"))?;
    let mut executors = executors(&host);
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    let report = engine.apply(&manual(&ids), &mut journal)?;
    assert_eq!(report.items[0].outcome, Outcome::Failed);
    assert_eq!(report.items[1].outcome, Outcome::Applied);
    assert!(journal.needs_recovery());
    assert!(matches!(
        engine.apply(&manual(&ids), &mut journal),
        Err(Error::RecoveryRequired)
    ));
    engine.undo(&[], &mut journal)?;
    assert_eq!(before, host.snapshot()?);
    Ok(())
}

#[test]
fn failed_undo_is_retryable_and_never_loses_original() -> TestResult {
    let catalog = load_embedded()?;
    let host = MockHost::default();
    let item = catalog.get("reg.cortana")?;
    host.seed(item, original(item))?;
    let dir = tempfile::tempdir()?;
    let mut journal = Journal::open(dir.path().join("journal.wal"))?;
    let mut executors = executors(&host);
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    engine.apply(&manual(&["reg.cortana"]), &mut journal)?;
    host.fail(item, Stage::Restore, ExecError::AccessDenied)?;
    assert_eq!(
        engine.undo(&[], &mut journal)?.items[0].outcome,
        Outcome::Failed
    );
    assert_eq!(
        journal.entry(&item.target.key()).ok_or("entry")?.status,
        Status::UndoFailed
    );
    assert_eq!(
        journal.entry(&item.target.key()).ok_or("entry")?.original,
        original(item)
    );
    host.clear_failures()?;
    engine.undo(&[], &mut journal)?;
    assert_eq!(
        host.snapshot()?.get(&item.target.key()),
        Some(&original(item))
    );
    Ok(())
}

#[test]
fn undo_one_item_keeps_other_changes() -> TestResult {
    let catalog = load_embedded()?;
    let host = MockHost::default();
    for id in ["reg.cortana", "svc.diagtrack"] {
        let item = catalog.get(id)?;
        host.seed(item, original(item))?;
    }
    let dir = tempfile::tempdir()?;
    let mut journal = Journal::open(dir.path().join("journal.wal"))?;
    let mut executors = executors(&host);
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    engine.apply(&manual(&["reg.cortana", "svc.diagtrack"]), &mut journal)?;
    assert_eq!(
        engine
            .undo(&["reg.cortana".into()], &mut journal)?
            .items
            .len(),
        1
    );
    assert_eq!(
        journal
            .entry(&catalog.get("svc.diagtrack")?.target.key())
            .ok_or("entry")?
            .status,
        Status::Applied
    );
    Ok(())
}

#[test]
fn crash_after_os_change_before_commit_recovers_from_prepared_record() -> TestResult {
    let catalog = load_embedded()?;
    let item = catalog.get("reg.cortana")?;
    let host = MockHost::default();
    host.seed(item, original(item))?;
    let before = host.snapshot()?;
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("journal.wal");
    let mut journal = Journal::open(&path)?;
    let mut executors = executors(&host);
    let mut engine = Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    };
    engine.apply(&manual(&["reg.cortana"]), &mut journal)?;
    drop(journal);
    // Crash model: Windows changed, but the terminal Applied record never reached disk.
    let bytes = fs::read(&path)?;
    let boundary = bytes.iter().position(|b| *b == b'\n').ok_or("newline")? + 1;
    fs::write(&path, &bytes[..boundary])?;
    let mut recovered = Journal::open(&path)?;
    assert!(recovered.needs_recovery());
    assert_eq!(
        recovered.entry(&item.target.key()).ok_or("entry")?.status,
        Status::Prepared
    );
    engine.undo(&[], &mut recovered)?;
    assert_eq!(before, host.snapshot()?);
    Ok(())
}

#[test]
fn torn_tail_is_discarded_but_valid_prepared_entry_is_retained() -> TestResult {
    let catalog = load_embedded()?;
    let host = MockHost::default();
    let item = catalog.get("reg.cortana")?;
    host.seed(item, original(item))?;
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("journal.wal");
    let mut journal = Journal::open(&path)?;
    let mut executors = executors(&host);
    Engine {
        catalog: &catalog,
        machine: machine(),
        executors: &mut executors,
    }
    .apply(&manual(&["reg.cortana"]), &mut journal)?;
    drop(journal);
    let bytes = fs::read(&path)?;
    let boundary = bytes.iter().position(|b| *b == b'\n').ok_or("newline")? + 1;
    let mut torn = bytes[..boundary].to_vec();
    torn.extend_from_slice(b"{\"version\":1,\"seq\":2");
    fs::write(&path, &torn)?;
    let recovered = Journal::open(&path)?;
    assert!(recovered.discarded_torn_tail);
    assert!(recovered.needs_recovery());
    drop(recovered);
    assert_eq!(fs::read(&path)?, bytes[..boundary]);
    Ok(())
}

#[test]
fn complete_corrupt_record_fails_closed_without_truncating_file() -> TestResult {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("journal.wal");
    let content = b"{\"version\":1,\"hash\":\"bad\"}\n";
    fs::write(&path, content)?;
    assert!(matches!(
        Journal::open(&path),
        Err(Error::CorruptJournal(_))
    ));
    assert_eq!(fs::read(&path)?, content);
    Ok(())
}

#[test]
fn journal_has_single_writer_lock_and_reopens_after_drop() -> TestResult {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("journal.wal");
    let first = Journal::open(&path)?;
    assert!(matches!(Journal::open(&path), Err(Error::JournalLocked(_))));
    drop(first);
    assert!(Journal::open(&path).is_ok());
    Ok(())
}

#[test]
fn real_windows_boundaries_require_an_interactive_context() -> TestResult {
    #[cfg(not(windows))]
    assert!(matches!(
        gametune_windows_runtime::WindowsContext::detect(),
        Err(ExecError::Unavailable)
    ));
    #[cfg(windows)]
    {
        match gametune_windows_runtime::WindowsContext::detect() {
            Ok(context) => {
                assert!(context.interactive_sid().starts_with("S-1-5-"));
                if !context.elevated() {
                    assert_eq!(context.ensure_elevated(), Err(ExecError::AccessDenied));
                }
            }
            Err(ExecError::Failed(message)) if message.contains("interactive") => (),
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}

#[test]
fn appx_mock_never_claims_exact_automatic_restoration() -> TestResult {
    let catalog = load_embedded()?;
    let item = catalog.get("appx.microsoft.bingnews")?;
    let host = MockHost::default();
    host.seed(item, original(item))?;
    let mut executor = gametune_exec_appx::MockExecutor::new(host);
    executor.apply(item)?;
    assert_eq!(
        executor.restore(item, &original(item)),
        Err(ExecError::ManualReinstallRequired)
    );
    Ok(())
}
