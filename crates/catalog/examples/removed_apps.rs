#![forbid(unsafe_code)]
//! Mock-only demonstration of the per-app flow and offline Reinstall buttons.
use gametune_core::*;
use gametune_exec_mock::{MockExecutor, MockHost};
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let catalog = gametune_catalog::load_embedded()?;
    let id = "appx.microsoft.bingnews";
    let item = catalog.get(id)?;
    let host = MockHost::default();
    host.seed(
        item,
        Snapshot::Appx {
            packages: vec!["Microsoft.BingNews_1.0.0.0_x64__8wekyb3d8bbwe".into()],
            provisioned: vec![],
            families: vec!["Microsoft.BingNews_8wekyb3d8bbwe".into()],
        },
    )?;
    let mut executors = Executors::default();
    executors.insert(MockExecutor::new(ActionType::Appx, host));
    let dir = tempfile::tempdir()?;
    let mut removed = RemovedApps::open(dir.path().join("removed-apps.json"), None)?;
    let mut engine = Engine {
        catalog: &catalog,
        machine: Machine {
            build: 26100,
            edition: Edition::Home,
            disk: DiskKind::Unknown,
        },
        executors: &mut executors,
    };
    engine.remove_app(
        &AppRemovalConsent {
            item_id: id.into(),
            acknowledge_no_undo: true,
            ..AppRemovalConsent::default()
        },
        &mut removed,
    )??;
    let html = removed.export_html(&catalog, false)?;
    let labels: serde_json::Value =
        serde_json::from_str(include_str!("../../../locales/pt-BR.json"))?;
    let notice = labels["appx.demo"]
        .as_str()
        .ok_or("missing demo label")?
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    let html = html.replacen("<h1>", &format!("<p role=\"note\">{notice}</p><h1>"), 1);
    let output = std::env::args_os()
        .nth(1)
        .ok_or("provide a path for the MOCK demonstration HTML")?;
    std::fs::write(output, html)?;
    println!("Mock-only Appx history exported; no installed app was changed");
    Ok(())
}
