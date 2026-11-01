use ametista_watcher::{storage::*, *};
use std::{
    collections::BTreeMap,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::Duration,
};
pub fn labels(english: bool) -> BTreeMap<String, String> {
    serde_json::from_str(if english {
        include_str!("../locales/en.json")
    } else {
        include_str!("../locales/pt-BR.json")
    })
    .unwrap_or_default()
}
pub fn run() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a != "--readonly") || args.len() > 1 {
        return Err(Error::Invalid);
    }
    let readonly = args.iter().any(|a| a == "--readonly");
    let directory = crate::tray::data_directory().map_err(|_| Error::Storage)?;
    let _lock = lock(&directory)?;
    // A corrupt editable library must not prevent recovery of the protected WAL.
    // Use no profiles in that case; never infer consent/actions from corrupt data.
    let library = load_for_monitor(&directory)?;
    let english = library.language == "en";
    let title = labels(library.language == "en")
        .get("title")
        .cloned()
        .ok_or(Error::Invalid)?;
    let done = Arc::new(AtomicBool::new(false));
    let completed = done.clone();
    std::thread::spawn(move || {
        let result = observe(&directory, library, readonly);
        if let Err(e) = result {
            let mut failed = status(&directory);
            failed.running = true;
            failed.error = Some(e.to_string());
            failed.recovery_pending = debt(&directory);
            let _ = save(&directory.join("monitor-status.json"), &failed);
            crate::tray::set_status(
                labels(english)
                    .get("error")
                    .map(String::as_str)
                    .unwrap_or("Ametista"),
            );
            while !crate::tray::stopped() {
                let control = bounded_read(&directory.join("monitor-stop.json"), 1024)
                    .ok()
                    .and_then(|b| serde_json::from_slice::<String>(&b).ok());
                if control.as_ref() == Some(&failed.instance) {
                    break;
                }
                std::thread::sleep(Duration::from_millis(500));
            }
            failed.running = false;
            let _ = save(&directory.join("monitor-status.json"), &failed);
        }
        completed.store(true, Ordering::Relaxed);
    });
    crate::tray::run(done, &title, english).map_err(|_| Error::Storage)
}
fn observe(directory: &std::path::Path, library: Library, readonly: bool) -> Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| Error::Invalid)?;
    runtime.block_on(observe_async(directory, library, readonly))
}
async fn observe_async(
    directory: &std::path::Path,
    library: Library,
    readonly: bool,
) -> Result<()> {
    use ametista_ipc::{GameStep, game_pipe::Client};
    let allowed = library.allowed_ids();
    let messages = labels(library.language == "en");
    let mut detector = Detector::new(library)?;
    let mut status = MonitorStatus {
        running: true,
        pid: std::process::id(),
        instance: ametista_ipc::nonce().map_err(|_| Error::Invalid)?,
        readonly,
        recovery_pending: debt(directory),
        ..MonitorStatus::default()
    };
    save(&directory.join("monitor-status.json"), &status)?;
    let mut client: Option<Client> = None;
    let (send, mut receive) = tokio::sync::mpsc::channel(128);
    let stop = Arc::new(AtomicBool::new(false));
    let stopping = stop.clone();
    let started = std::time::Instant::now();
    let health = Arc::new(AtomicU64::new(0));
    let heartbeat = health.clone();
    let source = std::thread::spawn(move || {
        let result = ametista_watcher::wmi::listen_health(
            |event| send.try_send(Ok(event)).is_ok(),
            || {
                heartbeat.store(started.elapsed().as_secs(), Ordering::Relaxed);
            },
            &stopping,
        );
        if result.is_err() {
            let _ = send.try_send(Err(Error::Invalid));
        }
    });
    let result: Result<()> = async {
        if status.recovery_pending && !readonly {
            crate::tray::set_status(
                messages
                    .get("recovering")
                    .map(String::as_str)
                    .unwrap_or("Ametista"),
            );
            let mut recovery = Client::open(vec![]).await.map_err(|_| Error::Invalid)?;
            recovery
                .step(GameStep::Finish {})
                .await
                .map_err(|_| Error::Invalid)?;
            save(&directory.join("recovery-pending.json"), &false)?;
            status.recovery_pending = false;
        }
        let mut applied = Vec::<String>::new();
        let mut last = std::time::Instant::now();
        loop {
            if started
                .elapsed()
                .as_secs()
                .saturating_sub(health.load(Ordering::Relaxed))
                > 10
            {
                return Err(Error::Invalid);
            }
            let control = bounded_read(&directory.join("monitor-stop.json"), 1024)
                .ok()
                .and_then(|b| serde_json::from_slice::<String>(&b).ok());
            if crate::tray::stopped() || control.as_ref() == Some(&status.instance) {
                break;
            }
            match tokio::time::timeout(Duration::from_millis(500), receive.recv()).await {
                Ok(Some(Ok(event))) => {
                    detector.observe(event)?;
                }
                Ok(Some(Err(e))) => return Err(e),
                Ok(None) => return Err(Error::Invalid),
                Err(_) => {}
            }
            while let Ok(event) = receive.try_recv() {
                detector.observe(event?)?;
            }
            let mut desired = detector.desired();
            if !readonly && client.is_none() && !desired.is_empty() {
                // This marker is durable BEFORE launching UAC. It is a recovery
                // hint only, never a source of elevated actions or catalog targets.
                save(&directory.join("recovery-pending.json"), &true)?;
                status.recovery_pending = true;
                crate::tray::set_status(
                    messages
                        .get("consent")
                        .map(String::as_str)
                        .unwrap_or("Ametista"),
                );
                client = Some(
                    Client::open(allowed.clone())
                        .await
                        .map_err(|_| Error::Invalid)?,
                );
                status.helper_pid = client.as_ref().map(Client::helper_pid);
                save(&directory.join("monitor-status.json"), &status)?;
                // Drain events received during UAC/recovery before applying anything.
                while let Ok(event) = receive.try_recv() {
                    detector.observe(event?)?;
                }
                desired = detector.desired();
            }
            if let Some(helper) = &mut client {
                if desired != applied {
                    let reply = helper
                        .step(GameStep::Set {
                            ids: desired.clone(),
                        })
                        .await
                        .map_err(|_| Error::Invalid)?;
                    if reply
                        .result
                        .get("items")
                        .and_then(|v| v.as_array())
                        .is_some_and(|items| {
                            items.iter().any(|i| {
                                i.get("outcome").and_then(|v| v.as_str()) == Some("skipped")
                            })
                        })
                    {
                        status.error = Some("monitor_items_skipped".into());
                    }
                    applied = desired.clone();
                    last = std::time::Instant::now();
                } else if last.elapsed() > Duration::from_secs(3) {
                    helper
                        .step(GameStep::Ping {})
                        .await
                        .map_err(|_| Error::Invalid)?;
                    last = std::time::Instant::now();
                }
                if desired.is_empty() {
                    helper
                        .step(GameStep::Finish {})
                        .await
                        .map_err(|_| Error::Invalid)?;
                    client = None;
                    status.helper_pid = None;
                    status.recovery_pending = false;
                    save(&directory.join("recovery-pending.json"), &false)?;
                }
            }
            let active = detector.active_games();
            if active != status.active_games {
                status.active_games = active;
                save(&directory.join("monitor-status.json"), &status)?;
            }
            let key = if readonly {
                "readonly"
            } else if client.is_some() {
                "active"
            } else {
                "idle"
            };
            crate::tray::set_status(messages.get(key).map(String::as_str).unwrap_or("Ametista"));
        }
        Ok(())
    }
    .await;
    stop.store(true, Ordering::Relaxed);
    let mut restore_ok = true;
    if let Some(helper) = &mut client {
        crate::tray::set_status(
            messages
                .get("recovering")
                .map(String::as_str)
                .unwrap_or("Ametista"),
        );
        restore_ok = helper.step(GameStep::Finish {}).await.is_ok();
        if restore_ok {
            save(&directory.join("recovery-pending.json"), &false)?;
            status.recovery_pending = false;
        }
    }
    drop(client);
    let _ = source.join();
    status.running = false;
    status.helper_pid = None;
    if result.is_err() || !restore_ok {
        status.error = Some("monitor_event_error".into());
    }
    save(&directory.join("monitor-status.json"), &status)?;
    if !restore_ok {
        return Err(Error::Storage);
    }
    result
}
