#![forbid(unsafe_code)]
#![cfg_attr(windows, windows_subsystem = "windows")]
fn main() {
    #[cfg(windows)]
    run();
}
#[cfg(windows)]
fn run() {
    use gametune_desktop_backend::*;
    use std::sync::Arc;
    #[tauri::command]
    async fn bootstrap() -> Result<Bootstrap, String> {
        tauri::async_runtime::spawn_blocking(gametune_desktop_backend::bootstrap)
            .await
            .map_err(|_| "operation_failed".to_string())?
    }
    #[tauri::command]
    fn analysis_start(state: tauri::State<'_, Arc<Desktop>>) -> Result<u64, String> {
        state.analysis_start()
    }
    #[tauri::command]
    fn analysis_state(state: tauri::State<'_, Arc<Desktop>>) -> Result<Analysis, String> {
        state.analysis_state()
    }
    #[tauri::command]
    fn analysis_cancel(state: tauri::State<'_, Arc<Desktop>>) -> Result<(), String> {
        state.analysis_cancel()
    }
    #[tauri::command]
    async fn prepare_review(
        state: tauri::State<'_, Arc<Desktop>>,
        selection: Selection,
    ) -> Result<Review, String> {
        let state = state.inner().clone();
        tauri::async_runtime::spawn_blocking(move || state.prepare_review(selection))
            .await
            .map_err(|_| "operation_failed".to_string())?
    }
    #[tauri::command]
    async fn execute_review(
        state: tauri::State<'_, Arc<Desktop>>,
        approval: Approval,
    ) -> Result<gametune_desktop_backend::IpcReply, String> {
        let command = state.consume_review(approval)?;
        execute(command).await
    }
    let result = tauri::Builder::default()
        .manage(Arc::new(Desktop::default()))
        .invoke_handler(tauri::generate_handler![
            bootstrap,
            analysis_start,
            analysis_state,
            analysis_cancel,
            prepare_review,
            execute_review
        ])
        .setup(|app| {
            let config = app
                .config()
                .app
                .windows
                .first()
                .ok_or("missing main window")?;
            tauri::WebviewWindowBuilder::from_config(app, config)?
                .on_navigation(|url| {
                    (url.scheme() == "http" && url.host_str() == Some("tauri.localhost"))
                        || (url.scheme() == "tauri" && url.host_str() == Some("localhost"))
                        || (cfg!(debug_assertions)
                            && url.scheme() == "http"
                            && url.host_str() == Some("127.0.0.1")
                            && url.port() == Some(1420))
                })
                .on_new_window(|_, _| tauri::webview::NewWindowResponse::Deny)
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!());
    if result.is_err() {
        std::process::exit(7);
    }
}
