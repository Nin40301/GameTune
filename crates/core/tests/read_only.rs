use gametune_core::*;
type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;
fn item() -> std::result::Result<Item, Box<dyn std::error::Error>> {
    let document: serde_json::Value =
        serde_json::from_str(include_str!("../../../catalog/items.json"))?;
    let items: Vec<Item> = serde_json::from_value(document["items"].clone())?;
    items
        .into_iter()
        .find(|i| i.id == "reg.cortana")
        .ok_or_else(|| "item".into())
}
#[test]
fn registry_observation_distinguishes_absence_type_and_applied() -> TestResult {
    let item = item()?;
    let Target::Registry { value: desired, .. } = item.target else {
        return Err("target".into());
    };
    for (value, keys, presence) in [
        (None, vec!["missing".into()], Presence::Missing),
        (
            Some(RegistryValue {
                kind: 4,
                bytes: desired.to_le_bytes().to_vec(),
            }),
            vec![],
            Presence::AlreadyApplied,
        ),
        (
            Some(RegistryValue {
                kind: 3,
                bytes: desired.to_le_bytes().to_vec(),
            }),
            vec![],
            Presence::Exists,
        ),
    ] {
        let row = observe(
            &item,
            Ok(Snapshot::Registry {
                value,
                created_keys: keys,
            }),
            ReadScope::InteractiveUser,
        );
        assert_eq!(row.presence, presence);
        assert!(row.current.is_some());
    }
    let denied = observe(&item, Err(ExecError::AccessDenied), ReadScope::Machine);
    assert!(denied.exists.is_none());
    assert_eq!(denied.presence, Presence::Unknown);
    Ok(())
}
#[test]
fn reinstall_history_preview_does_not_create_a_lock() -> TestResult {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("removed-apps.json");
    let catalog = Catalog::try_new(vec![item()?])?;
    assert!(RemovedApps::read_reinstall_actions(&path, "scope", &catalog, "unknown").is_err());
    assert_eq!(std::fs::read_dir(dir.path())?.count(), 0);
    Ok(())
}

#[test]
fn service_task_power_and_startup_states_are_interpreted() -> TestResult {
    let mut item = item()?;
    for (target, snapshot, expected) in [
        (
            Target::Service {
                name: "DiagTrack".into(),
                mode: ServiceMode::Disabled,
            },
            Snapshot::Service {
                start_mode: 4,
                delayed_auto_start: None,
                running: true,
            },
            Presence::Exists,
        ),
        (
            Target::Service {
                name: "DiagTrack".into(),
                mode: ServiceMode::Disabled,
            },
            Snapshot::Service {
                start_mode: 4,
                delayed_auto_start: None,
                running: false,
            },
            Presence::AlreadyApplied,
        ),
        (
            Target::ScheduledTask {
                path: "\\test".into(),
                name: "task".into(),
            },
            Snapshot::ScheduledTask {
                enabled: false,
                definition: None,
            },
            Presence::AlreadyApplied,
        ),
        (
            Target::Power {
                guid: "target".into(),
            },
            Snapshot::Power {
                active_guid: "other".into(),
                existing_guids: vec!["other".into()],
            },
            Presence::Missing,
        ),
        (
            Target::Startup {
                hive: Hive::HKCU,
                entry: Some(StartupTarget {
                    name: "Steam".into(),
                    fingerprint: "id".into(),
                }),
            },
            Snapshot::Startup {
                approved: Some(RegistryValue {
                    kind: 3,
                    bytes: vec![3, 0, 0, 0, 9, 8, 7, 6, 5, 4, 3, 2],
                }),
                created_keys: vec![],
            },
            Presence::AlreadyApplied,
        ),
    ] {
        item.target = target;
        let row = observe(&item, Ok(snapshot), ReadScope::Machine);
        assert_eq!(row.presence, expected);
    }
    Ok(())
}

#[test]
fn recorded_reinstall_is_read_only_scope_bound_and_validates_pfn() -> TestResult {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("removed-apps.json");
    let doc: serde_json::Value = serde_json::from_str(include_str!("../../../catalog/items.json"))?;
    let items: Vec<Item> = serde_json::from_value(doc["items"].clone())?;
    let catalog = Catalog::try_new(items)?;
    let id = "appx.microsoft.copilot";
    let document = serde_json::json!({"version":1,"scope":"machine:sid","entries":[{"itemId":id,"name":"Microsoft.Copilot","packageFamilyNames":["Microsoft.Copilot_8wekyb3d8bbwe"],"original":{"type":"appx","packages":[],"provisioned":[],"families":["Microsoft.Copilot_8wekyb3d8bbwe"]},"attemptedAt":"2026-10-31T00:00:00Z","status":"removed","error":null}]});
    std::fs::write(&path, serde_json::to_vec(&document)?)?;
    let before = std::fs::read(&path)?;
    let actions = RemovedApps::read_reinstall_actions(&path, "machine:sid", &catalog, id)?;
    assert_eq!(
        actions[0].uri(),
        "ms-windows-store://pdp/?PFN=Microsoft.Copilot_8wekyb3d8bbwe"
    );
    assert!(RemovedApps::read_reinstall_actions(&path, "another:sid", &catalog, id).is_err());
    assert_eq!(before, std::fs::read(&path)?);
    assert_eq!(std::fs::read_dir(dir.path())?.count(), 1);
    let mut invalid = document;
    invalid["entries"][0]["packageFamilyNames"] =
        serde_json::json!(["Microsoft.Copilot_x;calc.exe"]);
    std::fs::write(&path, serde_json::to_vec(&invalid)?)?;
    assert!(RemovedApps::read_reinstall_actions(&path, "machine:sid", &catalog, id).is_err());
    Ok(())
}
