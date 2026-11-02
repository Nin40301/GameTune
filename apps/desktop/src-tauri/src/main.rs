#![forbid(unsafe_code)]
#![cfg_attr(windows, windows_subsystem = "windows")]
#[cfg(windows)]
mod games;
fn main() {
    #[cfg(windows)]
    run();
}
#[cfg(windows)]
fn run() {
    use ametista_benchmark::{
        report::{self, Format},
        service::{Action, FileAction as LabFile, Lab, Status},
    };
    use ametista_desktop_backend::*;
    use std::sync::Arc;
    use tauri::Manager;
    #[tauri::command]
    async fn support_action(
        support: tauri::State<'_, Arc<support::Support>>,
        desktop: tauri::State<'_, Arc<Desktop>>,
        action: support::Action,
    ) -> Result<Option<support::Preview>, String> {
        let support = support.inner().clone();
        let desktop = desktop.inner().clone();
        tauri::async_runtime::spawn_blocking(move || {
            match action {
                support::Action::Preview {} => return support.preview(&desktop).map(Some),
                support::Action::Save { token, confirm } => {
                    let files = support.consume(&token, confirm)?;
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("ZIP", &["zip"])
                        .set_file_name("Ametista-diagnostic.zip")
                        .save_file()
                    {
                        support::save_zip(&path, files)?;
                    }
                }
                support::Action::Open { tool } => {
                    ametista_windows_runtime::WindowsContext::detect()
                        .map_err(|_| "operation_failed".to_string())?
                        .open_support_tool(tool)
                        .map_err(|_| "operation_failed".to_string())?;
                }
            }
            Ok(None)
        })
        .await
        .map_err(|_| "operation_failed".to_string())?
    }
    #[tauri::command]
    async fn lab_action(lab: tauri::State<'_, Arc<Lab>>, action: Action) -> Result<Status, String> {
        if matches!(action, Action::Start { .. }) && games::monitor_running()? {
            return Err("monitor_benchmark_conflict".into());
        }
        let lab = lab.inner().clone();
        tauri::async_runtime::spawn_blocking(move || lab.action(action).map_err(|e| e.to_string()))
            .await
            .map_err(|_| "operation_failed".to_string())?
    }
    #[tauri::command]
    async fn games_action(
        lab: tauri::State<'_, Arc<Lab>>,
        state: tauri::State<'_, Arc<games::Games>>,
        action: games::Action,
    ) -> Result<games::View, String> {
        let _guard = if matches!(action, games::Action::Start { .. }) {
            Some(lab.guard_mutation().map_err(|e| e.to_string())?)
        } else {
            None
        };
        let state = state.inner().clone();
        tauri::async_runtime::spawn_blocking(move || state.action(action))
            .await
            .map_err(|_| "operation_failed".to_string())?
    }
    #[tauri::command]
    async fn lab_file(lab: tauri::State<'_, Arc<Lab>>, action: LabFile) -> Result<Status, String> {
        let lab = lab.inner().clone();
        tauri::async_runtime::spawn_blocking(move || {
            match action {
                LabFile::ChoosePresentmon {} => {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("PresentMon", &["exe"])
                        .pick_file()
                    {
                        lab.choose_presentmon(path).map_err(|e| e.to_string())?;
                    }
                }
                LabFile::ImportCapture {} => {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("PresentMon CSV", &["csv"])
                        .pick_file()
                    {
                        lab.import_capture(&path).map_err(|e| e.to_string())?;
                    }
                }
                LabFile::Export {
                    format,
                    anonymize,
                    english,
                } => {
                    let extension = match format {
                        Format::Json => "json",
                        Format::Html => "html",
                    };
                    let name = format!("Ametista-report.{extension}");
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter(extension, &[extension])
                        .set_file_name(name)
                        .save_file()
                    {
                        if path
                            .extension()
                            .is_none_or(|e| !e.eq_ignore_ascii_case(extension))
                        {
                            return Err("benchmark_storage".into());
                        }
                        let text = report::render(
                            &lab.protocol().map_err(|e| e.to_string())?,
                            format,
                            anonymize,
                            english,
                        )
                        .map_err(|e| e.to_string())?;
                        std::fs::write(path, text).map_err(|_| "benchmark_storage".to_string())?;
                    }
                }
            }
            lab.status().map_err(|e| e.to_string())
        })
        .await
        .map_err(|_| "operation_failed".to_string())?
    }
    #[tauri::command]
    async fn bootstrap() -> Result<Bootstrap, String> {
        tauri::async_runtime::spawn_blocking(ametista_desktop_backend::bootstrap)
            .await
            .map_err(|_| "operation_failed".to_string())?
    }
    #[tauri::command]
    fn analysis_start(state: tauri::State<'_, Arc<Desktop>>) -> Result<u64, String> {
        state.analysis_start()
    }
    #[tauri::command]
    fn analysis_state(
        state: tauri::State<'_, Arc<Desktop>>,
        support: tauri::State<'_, Arc<support::Support>>,
    ) -> Result<Analysis, String> {
        let result = state.analysis_state()?;
        let _ = support.remember(&result);
        Ok(result)
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
        lab: tauri::State<'_, Arc<Lab>>,
        approval: Approval,
    ) -> Result<ametista_desktop_backend::IpcReply, String> {
        if std::env::var_os("AMETISTA_RESOURCE_MODE").is_some() {
            return Err("read_only".into());
        }
        let _mutation = lab.guard_mutation().map_err(|e| e.to_string())?;
        let command = state.consume_review(approval)?;
        let applied = matches!(&command, ametista_ipc::Command::Apply { .. });
        let reply = execute(command).await?;
        let ids = applied_ids(&reply);
        if applied && !ids.is_empty() {
            // Keep the real apply result visible even if saving benchmark metadata fails.
            // Lab exposes that storage error separately in its status.
            let _ = lab.record_applied(ids);
        }
        Ok(reply)
    }
    let result = tauri::Builder::default()
        .manage(Arc::new(Desktop::default()))
        .invoke_handler(tauri::generate_handler![
            bootstrap,
            analysis_start,
            analysis_state,
            analysis_cancel,
            prepare_review,
            execute_review,
            lab_action,
            lab_file,
            games_action,
            support_action
        ])
        .setup(|app| {
            app.manage(Arc::new(support::Support::new(
                app.path().app_local_data_dir()?.join("support"),
            )));
            let directory = app.path().app_local_data_dir()?.join("measurements");
            let monitor = std::env::var("AMETISTA_RESOURCE_MODE").as_deref() != Ok("idle");
            app.manage(Arc::new(Lab::open(&directory, monitor)?));
            app.manage(Arc::new(games::Games::new(
                app.path().app_local_data_dir()?.join("games"),
            )?));
            let config = app
                .config()
                .app
                .windows
                .first()
                .ok_or("missing main window")?;
            tauri::WebviewWindowBuilder::from_config(app, config)?
                // Empty supported options property suppresses wry's experimental
                // default switches too. No custom Chromium/WebView flags.
                .additional_browser_args("")
                .disable_drag_drop_handler()
                .data_directory(app.path().app_local_data_dir()?.join("webview-standard"))
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
