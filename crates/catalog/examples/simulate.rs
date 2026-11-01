#![forbid(unsafe_code)]
//! Fictitious Windows 10 Pro host. Never reads or changes the real computer.
use ametista_core::*;
use ametista_exec_mock::{MockExecutor, MockHost};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let catalog = ametista_catalog::load_embedded()?;
    let machine = Machine {
        build: 19045,
        edition: Edition::Pro,
        disk: DiskKind::Unknown,
    };
    let item_ids = Preset::Safe.select(&catalog, machine);
    let host = MockHost::default();
    for id in &item_ids {
        let item = catalog.get(id)?;
        let snapshot = match item.action_type {
            ActionType::Registry => Snapshot::Registry {
                value: None,
                created_keys: vec![],
            },
            ActionType::Service => Snapshot::Service {
                start_mode: 3,
                delayed_auto_start: None,
                running: false,
            },
            ActionType::ScheduledTask => Snapshot::ScheduledTask {
                enabled: true,
                definition: None,
            },
            ActionType::Power => Snapshot::Power {
                active_guid: "381b4222-f694-41f0-9685-ff5bb260df2e".into(),
                existing_guids: vec!["381b4222-f694-41f0-9685-ff5bb260df2e".into()],
            },
            _ => return Err("automatic profile contains a manual-only kind".into()),
        };
        host.seed(item, snapshot)?;
    }
    let mut executors = Executors::default();
    for kind in [
        ActionType::Registry,
        ActionType::Service,
        ActionType::ScheduledTask,
        ActionType::Power,
    ] {
        executors.insert(MockExecutor::new(kind, host.clone()));
    }
    let engine = Engine {
        catalog: &catalog,
        machine,
        executors: &mut executors,
    };
    let report = engine.run(&Request {
        item_ids,
        ..Request::default()
    })?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}
