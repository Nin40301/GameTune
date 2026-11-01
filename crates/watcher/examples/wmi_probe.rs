#![forbid(unsafe_code)]
#[cfg(windows)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use ametista_watcher::*;
    let exe = std::env::args()
        .nth(1)
        .ok_or("synthetic executable required")?;
    let game = Game {
        action_ids: vec!["reg.game_mode".into()],
        consent: true,
        enabled: true,
        ..Game::manual(exe.clone())?
    };
    let mut detector = Detector::new(Library {
        games: vec![game],
        ..Library::default()
    })?;
    let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let timeout = stop.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(18));
        timeout.store(true, std::sync::atomic::Ordering::Relaxed);
    });
    let mut started = false;
    let mut ended = false;
    ametista_watcher::wmi::listen(
        |event| {
            if event.path.as_deref().and_then(|p| normalize_exe(p).ok()) == normalize_exe(&exe).ok()
            {
                println!(
                    "event started={} pid={} path_present=true",
                    event.started, event.key.pid
                );
            }
            let result = detector.observe(event);
            if result.is_err() {
                return false;
            }
            if !detector.desired().is_empty() {
                started = true;
            }
            if started && detector.desired().is_empty() {
                ended = true;
                return false;
            }
            true
        },
        &stop,
    )?;
    if !started || !ended {
        return Err("start and stop were not both observed with a full path".into());
    }
    println!("synthetic detection passed; no executor was called");
    Ok(())
}
#[cfg(not(windows))]
fn main() {}
