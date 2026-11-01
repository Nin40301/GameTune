//! Run medians are the experimental units; adjacent 1-second samples are not replicates.
use crate::{Protocol, Run};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Distribution {
    pub runs: usize,
    pub median: f64,
    pub p5: f64,
    pub p95: f64,
    pub minimum: f64,
    pub maximum: f64,
    pub mad: f64,
    pub run_medians: Vec<f64>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comparison {
    pub metric: String,
    pub before: Distribution,
    pub after: Distribution,
    pub median_difference: f64,
    pub noise_band: f64,
    pub conclusion: String,
}
pub fn percentile(values: &[f64], quantile: f64) -> Option<f64> {
    if values.is_empty()
        || values.iter().any(|v| !v.is_finite())
        || !(0.0..=1.0).contains(&quantile)
    {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let position = quantile * (sorted.len() - 1) as f64;
    let low = position.floor() as usize;
    let high = position.ceil() as usize;
    Some(sorted[low] + (sorted[high] - sorted[low]) * (position - low as f64))
}
pub fn distribution(values: &[f64]) -> Option<Distribution> {
    let median = percentile(values, 0.5)?;
    let deviations: Vec<_> = values.iter().map(|x| (x - median).abs()).collect();
    Some(Distribution {
        runs: values.len(),
        median,
        p5: percentile(values, 0.05)?,
        p95: percentile(values, 0.95)?,
        minimum: percentile(values, 0.0)?,
        maximum: percentile(values, 1.0)?,
        mad: percentile(&deviations, 0.5)?,
        run_medians: values.to_vec(),
    })
}
pub fn compare(metric: &str, before: &[f64], after: &[f64], floor: f64) -> Option<Comparison> {
    if before.len() < 4 || after.len() < 4 {
        return None;
    }
    let before = distribution(before)?;
    let after = distribution(after)?;
    let delta = after.median - before.median;
    // Conservative descriptive rule, not a significance test or causal estimate.
    // Require separation of p5/p95 ranges AND a difference larger than noise/floor.
    let noise = (before.p95 - before.p5)
        .max(after.p95 - after.p5)
        .max(floor);
    let overlap = before.p5 <= after.p95 && after.p5 <= before.p95;
    let conclusion = if overlap || delta.abs() <= noise {
        "inconclusive"
    } else {
        "difference_observed"
    };
    Some(Comparison {
        metric: metric.into(),
        before,
        after,
        median_difference: delta,
        noise_band: noise,
        conclusion: conclusion.into(),
    })
}
fn medians(runs: &[Run], metric: &str) -> Vec<f64> {
    runs.iter()
        .filter(|r| !r.warmup)
        .filter_map(|r| {
            let values: Vec<_> = r
                .samples
                .iter()
                .filter_map(|s| match metric {
                    "available_memory" => Some(s.available_bytes as f64 / 1048576.0),
                    "cpu" => s.cpu_percent,
                    "processes" => Some(s.process_count as f64),
                    _ => None,
                })
                .collect();
            percentile(&values, 0.5)
        })
        .collect()
}
pub fn comparisons(p: &Protocol) -> Vec<Comparison> {
    [("available_memory", 16.0), ("cpu", 0.5), ("processes", 1.0)]
        .into_iter()
        .filter_map(|(m, f)| compare(m, &medians(&p.before, m), &medians(&p.after, m), f))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quantiles_and_variation() {
        assert!(percentile(&[0., 10., 20., 30.], 0.05).is_some_and(|p| (p - 1.5).abs() < 1e-12));
        assert!(percentile(&[0., 10., 20., 30.], 0.95).is_some_and(|p| (p - 28.5).abs() < 1e-12));
        let d = distribution(&[1., 2., 3., 4.]);
        assert_eq!(d.map(|d| d.mad), Some(1.0));
        assert!(percentile(&[f64::NAN], 0.5).is_none());
    }
    #[test]
    fn overlapping_runs_are_inconclusive() {
        let c = compare("cpu", &[10., 12., 9., 11.], &[10.5, 9.5, 11., 10.], 0.5);
        assert_eq!(c.map(|c| c.conclusion), Some("inconclusive".into()));
    }
    #[test]
    fn floor_prevents_false_precision_and_requires_replicates() {
        assert_eq!(
            compare("ram", &[100.; 4], &[101.; 4], 16.).map(|c| c.conclusion),
            Some("inconclusive".into())
        );
        assert!(compare("cpu", &[1.; 3], &[2.; 4], 0.5).is_none());
        assert_eq!(
            compare("cpu", &[1., 1.1, 0.9, 1.], &[3., 3.1, 3., 2.9], 0.5).map(|c| c.conclusion),
            Some("difference_observed".into())
        );
    }
}
