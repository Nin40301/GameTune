#![forbid(unsafe_code)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let desktop = gametune_desktop_backend::Desktop::default();
    let started = std::time::Instant::now();
    desktop.analysis_start()?;
    loop {
        let state = desktop.analysis_state()?;
        if state.status == "complete" || state.status == "error" {
            println!(
                "{}",
                serde_json::json!({"status":state.status,"rows":state.observations.len(),"readFailures":state.observations.iter().filter(|o|o.error.is_some()).count(),"elapsedSeconds":started.elapsed().as_secs_f64()})
            );
            if state.status != "complete" {
                return Err("analysis failed".into());
            }
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    Ok(())
}
