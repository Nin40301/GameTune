#![cfg(all(windows, feature = "real-profile-tests"))]
#[test]
#[ignore = "real catalog, two reboots, responsive desktop apps and full undo snapshot; disposable VM opt-in only"]
fn real_catalog_safe_balanced_reboot_health_and_undo() -> Result<(), Box<dyn std::error::Error>> {
    use gametune_windows_runtime::real_profiles::{self, Stage};
    let result = real_profiles::run(Stage::Verify)?;
    assert_eq!(result.status, "complete");
    Ok(())
}
