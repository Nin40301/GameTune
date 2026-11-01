#![forbid(unsafe_code)]
//! Read-only resource gate. Includes root and descendants; no process handles.
use gametune_metrics::{Collector, budget};
use std::{
    process::ExitCode,
    time::{Duration, Instant},
};
fn run() -> Result<bool, Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let pid = args.first().ok_or("root PID required")?.parse()?;
    let output = args.get(1).ok_or("output JSON required")?;
    let mut collector = Collector::default();
    collector.sample_pid(pid)?;
    let start = Instant::now();
    let mut samples = Vec::new();
    for n in 1..=60 {
        std::thread::sleep(
            (start + Duration::from_secs(n)).saturating_duration_since(Instant::now()),
        );
        let mut sample = collector.sample_pid(pid)?;
        // The resource gate needs only aggregate counters and our own tree.
        // Do not retain names of unrelated applications in its evidence.
        sample.top.clear();
        samples.push(sample);
    }
    let result = budget(&samples);
    let report = serde_json::json!({"budget":result,"durationSeconds":start.elapsed().as_secs_f64(),"definition":"peak private working set of root + all descendants; max 1-second CPU, normalized by total CPU capacity; 60 seconds after 30-second warmup", "finalAppTree":collector.app_memory(),"samples":samples});
    std::fs::write(output, serde_json::to_vec_pretty(&report)?)?;
    println!("{}", serde_json::to_string(&result)?);
    Ok(result.passed)
}
fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(2)
        }
    }
}
