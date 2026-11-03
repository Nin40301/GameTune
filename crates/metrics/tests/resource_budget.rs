use ametista_metrics::{Sample, resource_budget::*};

fn samples(mib: u64, cpu: f64) -> Vec<Sample> {
    (0..60)
        .map(|i| Sample {
            uptime_ms: i * 1000,
            timestamp_ms: i * 1000,
            total_bytes: 16 << 30,
            available_bytes: 8 << 30,
            cpu_percent: Some(1.0),
            process_count: 1,
            top: vec![],
            app_private_bytes: mib * 1024 * 1024,
            app_cpu_percent: Some(cpu),
            app_processes: 1,
            boot: "synthetic-boot".into(),
            power_plan: "synthetic-plan".into(),
        })
        .collect()
}
fn runs(mib: u64, cpu: f64) -> Result<Vec<RunSummary>, BudgetError> {
    (0..5).map(|_| summarize_run(&samples(mib, cpu))).collect()
}
fn baseline(mib: u64, cpu: f64) -> Result<ModeBaseline, BudgetError> {
    let runs = runs(mib, cpu)?;
    Ok(ModeBaseline {
        summary: summarize_cohort(&runs)?,
        runs,
    })
}
#[test]
fn interface_uses_medians_and_cpu_p95_is_only_an_alert() -> Result<(), BudgetError> {
    let mut s = samples(100, 0.2);
    for sample in s.iter_mut().take(4) {
        sample.app_private_bytes = 500 * 1024 * 1024;
        sample.app_cpu_percent = Some(50.0);
    }
    let result = evaluate_interface_v1(&vec![summarize_run(&s)?; 5], &baseline(100, 0.2)?)?;
    assert!(result.passed);
    assert!(result.cpu_p95_alert);
    assert_eq!(
        result.summary.run_median_ram_bytes.median,
        100.0 * 1024.0 * 1024.0
    );
    assert_eq!(result.summary.observed_maximum_cpu_percent, 50.0);
    Ok(())
}
#[test]
fn regression_boundary_is_more_than_fifteen_percent_for_ram_and_cpu() -> Result<(), BudgetError> {
    let original = baseline(100, 0.2)?;
    assert!(evaluate_interface_v1(&runs(115, 0.23)?, &original)?.passed);
    assert!(evaluate_interface_v1(&runs(116, 0.2)?, &original)?.ram_regression);
    assert!(evaluate_interface_v1(&runs(100, 0.231)?, &original)?.cpu_median_regression);
    assert!(!evaluate_interface_v1(&runs(100, 0.231)?, &original)?.passed);
    Ok(())
}
#[test]
fn zero_cpu_baseline_is_not_replaced_with_a_relaxed_floor() -> Result<(), BudgetError> {
    let original = baseline(100, 0.0)?;
    assert!(evaluate_interface_v1(&runs(100, 0.0)?, &original)?.passed);
    assert!(!evaluate_interface_v1(&runs(100, 0.001)?, &original)?.passed);
    Ok(())
}
#[test]
fn ram_ceiling_cannot_be_relaxed_by_a_large_baseline() -> Result<(), BudgetError> {
    let original = baseline(200, 0.2)?;
    assert!(evaluate_interface_v1(&runs(200, 0.2)?, &original)?.passed);
    assert!(evaluate_interface_v1(&runs(201, 0.2)?, &original)?.ram_cap_exceeded);
    assert!(!evaluate_interface_v1(&runs(210, 0.2)?, &baseline(210, 0.2)?)?.passed);
    Ok(())
}
#[test]
fn cohort_requires_five_runs_and_rejects_a_tampered_baseline() -> Result<(), BudgetError> {
    let original = baseline(100, 0.2)?;
    let mut cohort = runs(100, 0.2)?;
    cohort[0] = summarize_run(&samples(400, 0.2))?;
    assert!(evaluate_interface_v1(&cohort, &original)?.passed);
    assert!(summarize_cohort(&cohort[..4]).is_err());
    let mut tampered = original;
    tampered.summary.run_median_ram_bytes.median += 1.0;
    assert!(evaluate_interface_v1(&cohort, &tampered).is_err());
    Ok(())
}
#[test]
fn monitor_preserves_inclusive_ram_and_strict_peak_cpu_limits() -> Result<(), BudgetError> {
    let mut s = samples(15, 0.49);
    assert!(evaluate_monitor(&s)?.passed);
    s[0].app_private_bytes += 1;
    assert!(!evaluate_monitor(&s)?.passed);
    s[0].app_private_bytes -= 1;
    s[0].app_cpu_percent = Some(0.5);
    assert!(!evaluate_monitor(&s)?.passed);
    Ok(())
}
#[test]
fn incomplete_nonfinite_missing_cpu_and_time_gaps_are_errors() {
    let mut s = samples(10, 0.2);
    assert!(summarize_run(&s[..59]).is_err());
    s[2].app_cpu_percent = None;
    assert!(evaluate_monitor(&s).is_err());
    s[2].app_cpu_percent = Some(f64::NAN);
    assert!(summarize_run(&s).is_err());
    s[2].app_cpu_percent = Some(0.2);
    s[2].timestamp_ms = 9000;
    assert!(summarize_run(&s).is_err());
}
#[test]
fn single_cpu_spike_is_preserved_without_becoming_a_p95_alert() -> Result<(), BudgetError> {
    let mut s = samples(10, 0.2);
    s[0].app_cpu_percent = Some(10.0);
    let result = summarize_run(&s)?;
    assert_eq!(result.cpu_p95_percent, 0.2);
    assert_eq!(result.cpu_percent.maximum, 10.0);
    assert_eq!(result.cpu_percent.median, 0.2);
    Ok(())
}

#[test]
fn v2_absolute_cpu_boundaries_and_optional_mode_are_explicit() -> Result<(), BudgetError> {
    let base = baseline(160, 0.0)?;
    let good = evaluate_interface(&runs(163, 0.39)?, &base, InterfaceMode::Full)?;
    assert!(good.passed && good.blocking_passed);
    assert_eq!(good.policy_version, 2);
    assert!(!evaluate_interface(&runs(163, 1.0)?, &base, InterfaceMode::Full)?.passed);
    assert!(evaluate_interface(&runs(163, 0.999)?, &base, InterfaceMode::Full)?.passed);
    let bad = evaluate_interface(&runs(244, 0.39)?, &base, InterfaceMode::Reduced)?;
    assert!(!bad.passed && bad.blocking_passed && bad.informational);
    assert!(bad.ram_regression && bad.ram_cap_exceeded);
    assert!(
        evaluate_interface(&runs(200, 0.2)?, &baseline(200, 0.0)?, InterfaceMode::Full)?.passed
    );
    assert!(
        evaluate_interface(&runs(201, 0.2)?, &baseline(200, 0.0)?, InterfaceMode::Full)?
            .ram_cap_exceeded
    );
    assert!(
        evaluate_interface(&runs(115, 0.2)?, &baseline(100, 0.0)?, InterfaceMode::Full)?.passed
    );
    assert!(
        evaluate_interface(&runs(116, 0.2)?, &baseline(100, 0.0)?, InterfaceMode::Full)?
            .ram_regression
    );
    assert!(evaluate_interface(&runs(160, 0.2)?[..4], &base, InterfaceMode::Reduced).is_err());
    Ok(())
}
#[test]
fn v2_p95_is_strictly_above_two_and_never_blocks() -> Result<(), BudgetError> {
    let mut data = samples(100, 0.2);
    for sample in data.iter_mut().take(4) {
        sample.app_cpu_percent = Some(2.0);
    }
    let base = baseline(100, 0.0)?;
    let result = evaluate_interface(&vec![summarize_run(&data)?; 5], &base, InterfaceMode::Full)?;
    assert!(result.passed && !result.cpu_p95_alert);
    for sample in data.iter_mut().take(4) {
        sample.app_cpu_percent = Some(3.0);
    }
    let result = evaluate_interface(&vec![summarize_run(&data)?; 5], &base, InterfaceMode::Full)?;
    assert!(result.passed && result.cpu_p95_alert);
    Ok(())
}
