#![forbid(unsafe_code)]
use gametune_benchmark::{store::Store, *};
use gametune_metrics::{Group, Sample};
fn sample(t: u64, boot: &str) -> Sample {
    Sample {
        uptime_ms: 1_000_000 + t * 1000,
        timestamp_ms: t * 1000,
        total_bytes: 100000,
        available_bytes: 40000,
        cpu_percent: Some(5.),
        process_count: 100,
        top: vec![Group {
            name: "C:\\Users\\PrivateName\\game.exe".into(),
            count: 1,
            memory_bytes: 10,
        }],
        app_private_bytes: 10,
        app_cpu_percent: Some(0.2),
        app_processes: 2,
        boot: boot.into(),
        power_plan: "plan".into(),
    }
}
fn conditions() -> Conditions {
    Conditions {
        apps_closed: true,
        same_workload: true,
        cooled: true,
        same_power: true,
    }
}
fn run(p: &mut Protocol, boot: &str) -> Result<()> {
    p.begin(&sample(0, boot), &conditions())?;
    p.finish(
        (1..=60).map(|t| sample(t, boot)).collect(),
        60000,
        conditions(),
    )
}
#[test]
fn five_runs_first_warmup_and_requires_actual_reboot() -> Result<()> {
    let mut p = Protocol::new();
    for _ in 0..5 {
        run(&mut p, "a")?;
    }
    assert_eq!(p.phase, Phase::AwaitApply);
    assert_eq!(p.before.iter().filter(|r| r.warmup).count(), 1);
    p.applied(vec!["reg.copilot".into()], 2_000_000)?;
    assert!(matches!(
        p.begin(&sample(0, "a"), &conditions()),
        Err(Error::Reboot)
    ));
    for _ in 0..5 {
        run(&mut p, "b")?;
    }
    assert_eq!(p.phase, Phase::Complete);
    assert_eq!(stats::comparisons(&p).len(), 3);
    assert!(
        stats::comparisons(&p)
            .iter()
            .all(|c| c.conclusion == "inconclusive")
    );
    Ok(())
}
#[test]
fn simulated_crash_restores_completed_runs_and_discards_partial()
-> std::result::Result<(), Box<dyn std::error::Error>> {
    let d = tempfile::tempdir()?;
    {
        let store = Store::open(d.path())?;
        let mut p = Protocol::new();
        run(&mut p, "a")?;
        p.begin(&sample(0, "a"), &conditions())?;
        store.save(&p)?;
    }
    let store = Store::open(d.path())?;
    let mut p = store.load()?;
    p.resume();
    assert_eq!(p.before.len(), 1);
    assert!(p.interrupted);
    assert!(!p.pending_run);
    run(&mut p, "b")?;
    store.save(&p)?;
    assert_eq!(store.load()?.before.len(), 2);
    Ok(())
}
#[test]
fn power_changes_short_runs_and_sample_gaps_are_rejected() -> Result<()> {
    let mut p = Protocol::new();
    p.begin(&sample(0, "a"), &conditions())?;
    assert!(
        p.finish(
            (1..=60).map(|t| sample(t, "a")).collect(),
            59000,
            conditions()
        )
        .is_err()
    );
    let mut changed = sample(1, "a");
    changed.power_plan = "other".into();
    assert!(matches!(
        p.begin(&changed, &conditions()),
        Err(Error::Power)
    ));
    p.begin(&sample(0, "a"), &conditions())?;
    assert!(
        p.finish(
            (1..=60).map(|t| sample(t * 3, "a")).collect(),
            180000,
            conditions()
        )
        .is_err()
    );
    Ok(())
}
#[test]
fn changing_wall_clock_alone_is_not_proof_of_reboot() -> Result<()> {
    let mut p = Protocol::new();
    for _ in 0..5 {
        run(&mut p, "old-boot")?;
    }
    p.applied(vec!["reg.copilot".into()], 2_000_000)?;
    let mut changed = sample(0, "different-wall-clock");
    changed.uptime_ms = 2_000_001;
    assert!(matches!(
        p.begin(&changed, &conditions()),
        Err(Error::Reboot)
    ));
    changed.uptime_ms = 100_000;
    p.begin(&changed, &conditions())?;
    assert_eq!(p.phase, Phase::After);
    Ok(())
}
#[test]
fn anonymization_removes_all_names_and_paths() -> Result<()> {
    let mut p = Protocol::new();
    run(&mut p, "a")?;
    let raw = report::render(&p, report::Format::Json, false, false)?;
    assert!(raw.contains("PrivateName"));
    let anon = report::render(&p, report::Format::Json, true, false)?;
    assert!(!anon.contains("PrivateName"));
    assert!(!anon.contains("game.exe"));
    assert!(!anon.contains("\"uptimeMs\""));
    Ok(())
}
#[test]
fn measured_float_values_survive_checksum_roundtrip_exactly()
-> std::result::Result<(), Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let store = Store::open(directory.path())?;
    let mut p = Protocol::new();
    run(&mut p, "boot")?;
    for (index, s) in p.before[0].samples.iter_mut().enumerate() {
        s.cpu_percent = Some(100.0 * (index as f64 + 0.1) / (index as f64 + 71.3));
    }
    store.save(&p)?;
    let loaded = store.load()?;
    for (original, restored) in p.before[0].samples.iter().zip(&loaded.before[0].samples) {
        assert_eq!(
            original.cpu_percent.map(f64::to_bits),
            restored.cpu_percent.map(f64::to_bits)
        );
    }
    Ok(())
}
#[test]
fn persisted_full_protocol_survives_reboot_and_excludes_warmup_outliers()
-> std::result::Result<(), Box<dyn std::error::Error>> {
    let d = tempfile::tempdir()?;
    {
        let store = Store::open(d.path())?;
        let mut p = Protocol::new();
        for _ in 0..5 {
            run(&mut p, "before-boot")?;
        }
        for s in &mut p.before[0].samples {
            s.cpu_percent = Some(99.);
        }
        p.applied(vec!["reg.copilot".into()], 2_000_000)?;
        store.save(&p)?;
    }
    let store = Store::open(d.path())?;
    let mut p = store.load()?;
    p.resume();
    assert!(matches!(
        p.begin(&sample(0, "before-boot"), &conditions()),
        Err(Error::Reboot)
    ));
    for _ in 0..5 {
        run(&mut p, "after-boot")?;
    }
    store.save(&p)?;
    let results = stats::comparisons(&store.load()?);
    let cpu = results
        .iter()
        .find(|c| c.metric == "cpu")
        .ok_or("missing CPU")?;
    assert_eq!(cpu.before.median, 5.);
    assert_eq!(cpu.before.runs, 4);
    assert_eq!(cpu.conclusion, "inconclusive");
    Ok(())
}
#[test]
fn concurrent_store_and_corruption_fail_closed()
-> std::result::Result<(), Box<dyn std::error::Error>> {
    let d = tempfile::tempdir()?;
    let store = Store::open(d.path())?;
    assert!(Store::open(d.path()).is_err());
    store.save(&Protocol::new())?;
    std::fs::write(
        d.path().join("benchmark.json"),
        b"{\"checksum\":\"fake\",\"protocol\":{}}",
    )?;
    assert!(store.load().is_err());
    Ok(())
}
