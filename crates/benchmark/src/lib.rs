#![forbid(unsafe_code)]
pub mod presentmon;
pub mod report;
pub mod service;
#[cfg(test)]
mod service_tests;
pub mod stats;
pub mod store;
use ametista_metrics::Sample;
use serde::{Deserialize, Serialize};

pub const RUNS: usize = 5;
pub const SECONDS: u64 = 60;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("benchmark_state")]
    State,
    #[error("benchmark_conditions")]
    Conditions,
    #[error("benchmark_power")]
    Power,
    #[error("benchmark_reboot")]
    Reboot,
    #[error("benchmark_invalid_run")]
    InvalidRun,
    #[error("benchmark_storage")]
    Storage,
    #[error("presentmon_invalid")]
    PresentMon,
}
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    #[default]
    Before,
    AwaitApply,
    AwaitReboot,
    After,
    Complete,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Conditions {
    pub apps_closed: bool,
    pub same_workload: bool,
    pub cooled: bool,
    pub same_power: bool,
}
impl Conditions {
    pub fn accepted(&self) -> bool {
        self.apps_closed && self.same_workload && self.cooled && self.same_power
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Run {
    pub warmup: bool,
    pub duration_ms: u64,
    pub conditions: Conditions,
    pub samples: Vec<Sample>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Protocol {
    pub version: u32,
    pub phase: Phase,
    pub baseline_boot: Option<String>,
    pub applied_uptime_ms: Option<u64>,
    pub power_plan: Option<String>,
    pub before: Vec<Run>,
    pub after: Vec<Run>,
    pub interrupted: bool,
    pub pending_run: bool,
    pub applied_ids: Vec<String>,
    pub presentmon: Vec<presentmon::Capture>,
}
impl Protocol {
    /// Avoid cloning hundreds of process lists on every UI progress poll.
    pub fn summary(&self) -> Self {
        let runs = |runs: &[Run]| {
            runs.iter()
                .map(|r| Run {
                    warmup: r.warmup,
                    duration_ms: r.duration_ms,
                    conditions: r.conditions.clone(),
                    samples: Vec::new(),
                })
                .collect()
        };
        Self {
            version: self.version,
            phase: self.phase.clone(),
            baseline_boot: self.baseline_boot.clone(),
            applied_uptime_ms: self.applied_uptime_ms,
            power_plan: self.power_plan.clone(),
            before: runs(&self.before),
            after: runs(&self.after),
            interrupted: self.interrupted,
            pending_run: self.pending_run,
            applied_ids: self.applied_ids.clone(),
            presentmon: self.presentmon.clone(),
        }
    }
    pub fn new() -> Self {
        Self {
            version: 1,
            ..Self::default()
        }
    }
    pub fn resume(&mut self) {
        if self.pending_run {
            self.interrupted = true;
            self.pending_run = false;
        }
    }
    pub fn begin(&mut self, sample: &Sample, conditions: &Conditions) -> Result<()> {
        if !conditions.accepted() {
            return Err(Error::Conditions);
        }
        if self.pending_run {
            return Err(Error::State);
        }
        if let Some(plan) = &self.power_plan
            && plan != &sample.power_plan
        {
            return Err(Error::Power);
        }
        if self.phase == Phase::AwaitReboot {
            // Fail closed: a wall-clock adjustment alone must not count as reboot.
            // If delayed too long after reboot, restart again and begin sooner.
            if self.baseline_boot.as_ref() == Some(&sample.boot)
                || self
                    .applied_uptime_ms
                    .is_none_or(|uptime| sample.uptime_ms >= uptime)
            {
                return Err(Error::Reboot);
            }
            self.phase = Phase::After;
        }
        if !matches!(self.phase, Phase::Before | Phase::After) {
            return Err(Error::State);
        }
        if self.baseline_boot.is_none() {
            self.baseline_boot = Some(sample.boot.clone());
            self.power_plan = Some(sample.power_plan.clone());
        }
        self.pending_run = true;
        self.interrupted = false;
        Ok(())
    }
    pub fn cancel(&mut self) {
        self.pending_run = false;
        self.interrupted = true;
    }
    pub fn finish(
        &mut self,
        samples: Vec<Sample>,
        duration_ms: u64,
        conditions: Conditions,
    ) -> Result<()> {
        if !self.pending_run {
            return Err(Error::State);
        }
        let valid = (SECONDS * 1000..=(SECONDS + 2) * 1000).contains(&duration_ms)
            && samples.len() >= SECONDS as usize
            && samples.len() <= 62
            && samples
                .first()
                .zip(samples.last())
                .is_some_and(|(a, b)| b.timestamp_ms.saturating_sub(a.timestamp_ms) >= 58500)
            && conditions.accepted()
            && samples.iter().all(|s| {
                s.cpu_percent
                    .is_some_and(|v| v.is_finite() && (0.0..=100.0).contains(&v))
                    && Some(&s.power_plan) == self.power_plan.as_ref()
            })
            && samples.windows(2).all(|s| {
                s[0].boot == s[1].boot
                    && (500..=2000).contains(&s[1].timestamp_ms.saturating_sub(s[0].timestamp_ms))
            });
        if !valid {
            self.cancel();
            return Err(Error::InvalidRun);
        }
        let runs = match self.phase {
            Phase::Before => &mut self.before,
            Phase::After => &mut self.after,
            _ => return Err(Error::State),
        };
        if runs.len() >= RUNS {
            return Err(Error::State);
        }
        runs.push(Run {
            warmup: runs.is_empty(),
            duration_ms,
            conditions,
            samples,
        });
        self.pending_run = false;
        if runs.len() == RUNS {
            self.phase = if self.phase == Phase::Before {
                Phase::AwaitApply
            } else {
                Phase::Complete
            };
        }
        Ok(())
    }
    /// Called by the native bridge only after a successful elevated Apply response.
    pub fn applied(&mut self, ids: Vec<String>, uptime_ms: u64) -> Result<()> {
        if self.phase != Phase::AwaitApply || ids.is_empty() {
            return Err(Error::State);
        }
        self.applied_ids = ids;
        self.applied_uptime_ms = Some(uptime_ms);
        self.phase = Phase::AwaitReboot;
        Ok(())
    }
}
