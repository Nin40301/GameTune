#![forbid(unsafe_code)]
//! Fictitious Windows 10 Pro host. Never reads or changes the real computer.
use gametune_core::*;
use gametune_exec_mock::{MockExecutor, MockHost};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let catalog = gametune_catalog::load_embedded()?;
    let machine = Machine {
        build: 19045,
        edition: Edition::Pro,
        disk: DiskKind::Unknown,
    };
    let item_ids = Preset::Safe.select(&catalog, machine);
    let host = MockHost::default();
    for id in &item_ids {
        host.seed(
            catalog.get(id)?,
            Snapshot::Registry {
                value: None,
                created_keys: Vec::new(),
            },
        )?;
    }
    let mut executors = Executors::default();
    executors.insert(MockExecutor::new(ActionType::Registry, host));
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
