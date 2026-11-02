//! Development-only resource policy. Collection never changes Windows settings.
use crate::Sample;
use serde::{Deserialize, Serialize};

pub const SAMPLES_PER_RUN: usize = 60;
pub const RUNS_PER_MODE: usize = 5;
pub const INTERFACE_RAM_CAP_BYTES: f64 = 200.0 * 1024.0 * 1024.0;
pub const MONITOR_RAM_CAP_BYTES: u64 = 15 * 1024 * 1024;

#[derive(Debug, thiserror::Error)]
pub enum BudgetError {
    #[error("resource_sampling_invalid")]
    InvalidSamples,
    #[error("resource_cohort_requires_five_runs")]
    InvalidCohort,
    #[error("resource_baseline_invalid")]
    InvalidBaseline,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Distribution {
    pub minimum: f64,
    pub median: f64,
    pub maximum: f64,
}

impl Distribution {
    fn valid(self) -> bool {
        [self.minimum, self.median, self.maximum]
            .iter()
            .all(|v| v.is_finite() && *v >= 0.0)
            && self.minimum <= self.median
            && self.median <= self.maximum
    }
}

fn quantile(sorted: &[f64], q: f64) -> f64 {
    let rank = (sorted.len() - 1) as f64 * q;
    let low = rank.floor() as usize;
    let high = rank.ceil() as usize;
    sorted[low] + (sorted[high] - sorted[low]) * rank.fract()
}

fn distribution(mut values: Vec<f64>) -> Distribution {
    values.sort_by(f64::total_cmp);
    Distribution {
        minimum: values[0],
        median: quantile(&values, 0.5),
        maximum: values[values.len() - 1],
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RunSummary {
    pub samples: usize,
    pub span_ms: u64,
    pub ram_bytes: Distribution,
    pub cpu_percent: Distribution,
    pub cpu_p95_percent: f64,
}

impl RunSummary {
    fn valid(&self) -> bool {
        self.samples == SAMPLES_PER_RUN
            && (58_500..=118_000).contains(&self.span_ms)
            && self.ram_bytes.valid()
            && self.ram_bytes.minimum > 0.0
            && self.cpu_percent.valid()
            && self.cpu_percent.maximum <= 100.0
            && self.cpu_p95_percent.is_finite()
            && self.cpu_p95_percent >= self.cpu_percent.median
            && self.cpu_p95_percent <= self.cpu_percent.maximum
    }
}

pub fn summarize_run(samples: &[Sample]) -> Result<RunSummary, BudgetError> {
    if samples.len() != SAMPLES_PER_RUN
        || !samples
            .windows(2)
            .all(|p| (500..=2000).contains(&p[1].timestamp_ms.saturating_sub(p[0].timestamp_ms)))
        || samples.iter().any(|s| {
            s.app_private_bytes == 0
                || s.app_processes == 0
                || !s
                    .app_cpu_percent
                    .is_some_and(|v| v.is_finite() && (0.0..=100.0).contains(&v))
        })
    {
        return Err(BudgetError::InvalidSamples);
    }
    let mut cpu: Vec<_> = samples.iter().filter_map(|s| s.app_cpu_percent).collect();
    cpu.sort_by(f64::total_cmp);
    let result = RunSummary {
        samples: samples.len(),
        span_ms: samples[samples.len() - 1].timestamp_ms - samples[0].timestamp_ms,
        ram_bytes: distribution(samples.iter().map(|s| s.app_private_bytes as f64).collect()),
        cpu_percent: distribution(cpu.clone()),
        cpu_p95_percent: quantile(&cpu, 0.95),
    };
    if !result.valid() {
        return Err(BudgetError::InvalidSamples);
    }
    Ok(result)
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CohortSummary {
    pub executions: usize,
    /// These distributions contain the five per-run medians, not peaks.
    pub run_median_ram_bytes: Distribution,
    pub run_median_cpu_percent: Distribution,
    pub run_cpu_p95_percent: Distribution,
    pub observed_minimum_ram_bytes: f64,
    pub observed_maximum_ram_bytes: f64,
    pub observed_minimum_cpu_percent: f64,
    pub observed_maximum_cpu_percent: f64,
}

pub fn summarize_cohort(runs: &[RunSummary]) -> Result<CohortSummary, BudgetError> {
    if runs.len() != RUNS_PER_MODE || runs.iter().any(|r| !r.valid()) {
        return Err(BudgetError::InvalidCohort);
    }
    Ok(CohortSummary {
        executions: runs.len(),
        run_median_ram_bytes: distribution(runs.iter().map(|r| r.ram_bytes.median).collect()),
        run_median_cpu_percent: distribution(runs.iter().map(|r| r.cpu_percent.median).collect()),
        run_cpu_p95_percent: distribution(runs.iter().map(|r| r.cpu_p95_percent).collect()),
        observed_minimum_ram_bytes: runs
            .iter()
            .map(|r| r.ram_bytes.minimum)
            .fold(f64::INFINITY, f64::min),
        observed_maximum_ram_bytes: runs.iter().map(|r| r.ram_bytes.maximum).fold(0.0, f64::max),
        observed_minimum_cpu_percent: runs
            .iter()
            .map(|r| r.cpu_percent.minimum)
            .fold(f64::INFINITY, f64::min),
        observed_maximum_cpu_percent: runs
            .iter()
            .map(|r| r.cpu_percent.maximum)
            .fold(0.0, f64::max),
    })
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModeBaseline {
    pub runs: Vec<RunSummary>,
    pub summary: CohortSummary,
}

fn regressed(current: f64, baseline: f64) -> bool {
    // More than 15%, with only machine-rounding tolerance at the exact boundary.
    // A zero baseline allows zero only; no host-dependent floor or relaxation.
    let left = current * 100.0;
    let right = baseline * 115.0;
    left - right > f64::EPSILON * left.abs().max(right.abs()) * 8.0
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InterfaceEvaluation {
    pub summary: CohortSummary,
    pub ram_regression: bool,
    pub cpu_median_regression: bool,
    pub ram_cap_exceeded: bool,
    /// p95 >= 1% in any run is an alert, never a blocking CPU-peak criterion.
    pub cpu_p95_alert: bool,
    pub passed: bool,
}

pub fn evaluate_interface(
    runs: &[RunSummary],
    baseline: &ModeBaseline,
) -> Result<InterfaceEvaluation, BudgetError> {
    let original = summarize_cohort(&baseline.runs).map_err(|_| BudgetError::InvalidBaseline)?;
    if original != baseline.summary {
        return Err(BudgetError::InvalidBaseline);
    }
    let summary = summarize_cohort(runs)?;
    let ram_regression = regressed(
        summary.run_median_ram_bytes.median,
        original.run_median_ram_bytes.median,
    );
    let cpu_median_regression = regressed(
        summary.run_median_cpu_percent.median,
        original.run_median_cpu_percent.median,
    );
    let ram_cap_exceeded = summary.run_median_ram_bytes.median > INTERFACE_RAM_CAP_BYTES;
    Ok(InterfaceEvaluation {
        cpu_p95_alert: summary.run_cpu_p95_percent.maximum >= 1.0,
        passed: !ram_regression && !cpu_median_regression && !ram_cap_exceeded,
        summary,
        ram_regression,
        cpu_median_regression,
        ram_cap_exceeded,
    })
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitorEvaluation {
    pub samples: usize,
    pub peak_private_bytes: u64,
    pub peak_cpu_percent: f64,
    pub passed: bool,
}

pub fn evaluate_monitor(samples: &[Sample]) -> Result<MonitorEvaluation, BudgetError> {
    let stats = summarize_run(samples)?;
    let peak_private_bytes = samples
        .iter()
        .map(|s| s.app_private_bytes)
        .max()
        .ok_or(BudgetError::InvalidSamples)?;
    let peak_cpu_percent = stats.cpu_percent.maximum;
    Ok(MonitorEvaluation {
        samples: samples.len(),
        peak_private_bytes,
        peak_cpu_percent,
        passed: peak_private_bytes <= MONITOR_RAM_CAP_BYTES && peak_cpu_percent < 0.5,
    })
}
