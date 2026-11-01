fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let attributes = tauri_build::Attributes::new()
            .app_manifest(tauri_build::AppManifest::new().commands(&[
                "bootstrap",
                "analysis_start",
                "analysis_state",
                "analysis_cancel",
                "prepare_review",
                "execute_review",
                "lab_action",
                "lab_file",
                "games_action",
            ]))
            .windows_attributes(
                tauri_build::WindowsAttributes::new().app_manifest(include_str!("app.manifest")),
            );
        if let Err(error) = tauri_build::try_build(attributes) {
            panic!("Tauri build: {error}");
        }
    }
}
