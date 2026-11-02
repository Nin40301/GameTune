#![forbid(unsafe_code)]
//! Read-only collection and a separate five-run interface gate.
use ametista_metrics::{
    Collector, Sample,
    resource_budget::{self, ModeBaseline, RunSummary},
};
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    process::ExitCode,
    time::{Duration, Instant},
};
#[derive(Deserialize)]
struct Measurement {
    samples: Vec<Sample>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BaselineFile {
    schema_version: u32,
    modes: BTreeMap<String, ModeBaseline>,
}
fn read_run(path: &str) -> Result<RunSummary, Box<dyn std::error::Error>> {
    let report: Measurement = serde_json::from_slice(&std::fs::read(path)?)?;
    Ok(resource_budget::summarize_run(&report.samples)?)
}
fn create_baseline(args: &[String]) -> Result<bool, Box<dyn std::error::Error>> {
    if args.len() != 12 {
        return Err(
            "baseline requires metadata, candidate output and ten run files (full first)".into(),
        );
    }
    let mut metadata: serde_json::Value = serde_json::from_slice(&std::fs::read(&args[0])?)?;
    if !metadata.is_object() {
        return Err("baseline metadata object required".into());
    }
    let mut modes = BTreeMap::new();
    let (cohorts, _) = args[2..].as_chunks::<5>();
    for (mode, paths) in ["full", "reduced"].into_iter().zip(cohorts) {
        let runs: Vec<_> = paths
            .iter()
            .map(|path| read_run(path))
            .collect::<Result<_, _>>()?;
        modes.insert(
            mode,
            ModeBaseline {
                summary: resource_budget::summarize_cohort(&runs)?,
                runs,
            },
        );
    }
    metadata["schemaVersion"] = serde_json::json!(1);
    metadata["modes"] = serde_json::to_value(modes)?;
    std::fs::write(&args[1], serde_json::to_vec_pretty(&metadata)?)?;
    println!("Baseline candidate written; review metadata and results before committing");
    Ok(true)
}
fn evaluate(args: &[String]) -> Result<bool, Box<dyn std::error::Error>> {
    if args.len() != 8 {
        return Err(
            "evaluate requires baseline, effects, output and exactly five run files".into(),
        );
    }
    let baseline: BaselineFile = serde_json::from_slice(&std::fs::read(&args[0])?)?;
    if baseline.schema_version != 1 || !matches!(args[1].as_str(), "full" | "reduced") {
        return Err("invalid baseline version or effects".into());
    }
    let mode = baseline
        .modes
        .get(&args[1])
        .ok_or("baseline mode missing")?;
    let runs: Vec<RunSummary> = args[3..]
        .iter()
        .map(|path| read_run(path))
        .collect::<Result<_, _>>()?;
    let result = resource_budget::evaluate_interface(&runs, mode)?;
    std::fs::write(&args[2], serde_json::to_vec_pretty(&result)?)?;
    println!("{}", serde_json::to_string(&result)?);
    Ok(result.passed)
}
fn run() -> Result<bool, Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.first().is_some_and(|arg| arg == "evaluate") {
        return evaluate(&args[1..]);
    }
    if args.first().is_some_and(|arg| arg == "baseline") {
        return create_baseline(&args[1..]);
    }
    if args.len() != 3 {
        return Err("root PID, output JSON and interface|game_monitor required".into());
    }
    let pid = args[0].parse()?;
    let output = &args[1];
    let profile = args[2].as_str();
    if !matches!(profile, "interface" | "game_monitor") {
        return Err("unknown budget profile".into());
    }
    let mut collector = Collector::default();
    collector.sample_pid(pid)?;
    let start = Instant::now();
    let mut samples = Vec::new();
    for n in 1..=60 {
        std::thread::sleep(
            (start + Duration::from_secs(n)).saturating_duration_since(Instant::now()),
        );
        let mut sample = collector.sample_pid(pid)?;
        sample.top.clear();
        samples.push(sample);
    }
    let stats = resource_budget::summarize_run(&samples)?;
    let mut report = serde_json::json!({"profile":profile,"collectionOnly":profile == "interface", "summary":stats,"durationSeconds":start.elapsed().as_secs_f64(),"definition":"private resident memory of root and descendants; 60 one-second samples after external warmup; interface requires separate five-run baseline evaluation", "finalAppTree":collector.app_memory(),"samples":samples});
    let passed = if profile == "game_monitor" {
        let result = resource_budget::evaluate_monitor(&samples)?;
        let passed = result.passed;
        report["budget"] = serde_json::to_value(result)?;
        passed
    } else {
        true
    }; // Collection success is explicitly not interface approval.
    std::fs::write(output, serde_json::to_vec_pretty(&report)?)?;
    println!("{}", serde_json::to_string(&report["summary"])?);
    Ok(passed)
}
fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}
