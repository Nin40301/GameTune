#![deny(unsafe_code)]
//! System-wide read-only snapshots. Never opens an individual process.
#[cfg(windows)]
#[allow(unsafe_code)]
mod native;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("metrics_unavailable")]
    Unavailable,
    #[error("metrics_layout")]
    Layout,
}
#[derive(Clone, Debug)]
pub struct Process {
    pub pid: u32,
    pub parent: u32,
    pub created: u64,
    pub name: String,
    pub working_set: u64,
    pub private_working_set: u64,
    pub cpu_ticks: u64,
}
#[derive(Clone, Debug)]
pub struct Raw {
    pub uptime_ms: u64,
    pub total: u64,
    pub available: u64,
    pub idle: u64,
    pub total_ticks: u64,
    pub processes: Vec<Process>,
    pub boot: String,
    pub power_plan: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub name: String,
    pub count: usize,
    pub memory_bytes: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sample {
    pub uptime_ms: u64,
    pub timestamp_ms: u64,
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub cpu_percent: Option<f64>,
    pub process_count: usize,
    pub top: Vec<Group>,
    pub app_private_bytes: u64,
    pub app_cpu_percent: Option<f64>,
    pub app_processes: usize,
    pub boot: String,
    pub power_plan: String,
}
#[derive(Default)]
pub struct Collector {
    previous: Option<Raw>,
    root: Option<(u32, u64)>,
}
fn family(raw: &Raw, root: (u32, u64)) -> BTreeSet<u32> {
    let mut pids = BTreeSet::new();
    if !raw.processes.iter().any(|p| (p.pid, p.created) == root) {
        return pids;
    }
    let mut children = BTreeMap::<u32, Vec<&Process>>::new();
    for process in &raw.processes {
        children.entry(process.parent).or_default().push(process);
    }
    let mut queue = vec![root];
    while let Some((pid, created)) = queue.pop() {
        if !pids.insert(pid) {
            continue;
        }
        if let Some(children) = children.get(&pid) {
            // Creation time prevents attributing an old process to a reused parent.
            queue.extend(
                children
                    .iter()
                    .filter(|p| p.created >= created)
                    .map(|p| (p.pid, p.created)),
            );
        }
    }
    pids
}
impl Collector {
    /// Diagnostic breakdown of the same app tree used by the resource gate.
    pub fn app_memory(&self) -> Vec<Group> {
        let (Some(raw), Some(root)) = (&self.previous, self.root) else {
            return Vec::new();
        };
        let pids = family(raw, root);
        raw.processes
            .iter()
            .filter(|p| pids.contains(&p.pid))
            .map(|p| Group {
                name: p.name.clone(),
                count: 1,
                memory_bytes: p.private_working_set,
            })
            .collect()
    }
    pub fn sample(&mut self) -> Result<Sample, Error> {
        self.sample_pid(std::process::id())
    }
    pub fn sample_pid(&mut self, root_pid: u32) -> Result<Sample, Error> {
        #[cfg(windows)]
        {
            self.ingest(native::read()?, root_pid)
        }
        #[cfg(not(windows))]
        {
            let _ = root_pid;
            Err(Error::Unavailable)
        }
    }
    pub fn ingest(&mut self, raw: Raw, root_pid: u32) -> Result<Sample, Error> {
        if self.root.is_none() {
            self.root = raw
                .processes
                .iter()
                .find(|p| p.pid == root_pid)
                .map(|p| (p.pid, p.created));
        }
        let root = self.root.ok_or(Error::Unavailable)?;
        let pids = family(&raw, root);
        if pids.is_empty() {
            return Err(Error::Unavailable);
        }
        let mut groups = BTreeMap::<String, Group>::new();
        let mut app_bytes = 0;
        for p in &raw.processes {
            let key = p.name.to_lowercase();
            let group = groups.entry(key).or_insert_with(|| Group {
                name: p.name.clone(),
                count: 0,
                memory_bytes: 0,
            });
            group.count += 1;
            group.memory_bytes += p.working_set;
            if pids.contains(&p.pid) {
                app_bytes += p.private_working_set;
            }
        }
        let mut top: Vec<_> = groups.into_values().collect();
        top.sort_by(|a, b| {
            b.memory_bytes
                .cmp(&a.memory_bytes)
                .then(a.name.cmp(&b.name))
        });
        top.truncate(12);
        let mut cpu = None;
        let mut app_cpu = None;
        if let Some(old) = &self.previous {
            let total = raw.total_ticks.saturating_sub(old.total_ticks);
            if old.boot == raw.boot && total > 0 {
                cpu = Some(
                    100.0 * total.saturating_sub(raw.idle.saturating_sub(old.idle)) as f64
                        / total as f64,
                );
                let ticks: u64 = raw
                    .processes
                    .iter()
                    .filter(|p| pids.contains(&p.pid))
                    .map(|p| {
                        old.processes
                            .iter()
                            .find(|o| (o.pid, o.created) == (p.pid, p.created))
                            // A child born since the previous complete snapshot
                            // has no previous counter: count its lifetime CPU.
                            .map_or(p.cpu_ticks, |o| p.cpu_ticks.saturating_sub(o.cpu_ticks))
                    })
                    .sum();
                app_cpu = Some((100.0 * ticks as f64 / total as f64).clamp(0.0, 100.0));
            }
        }
        let result = Sample {
            uptime_ms: raw.uptime_ms,
            timestamp_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|_| Error::Unavailable)?
                .as_millis() as u64,
            total_bytes: raw.total,
            available_bytes: raw.available,
            cpu_percent: cpu,
            process_count: raw.processes.len(),
            top,
            app_private_bytes: app_bytes,
            app_cpu_percent: app_cpu,
            app_processes: pids.len(),
            boot: raw.boot.clone(),
            power_plan: raw.power_plan.clone(),
        };
        self.previous = Some(raw);
        Ok(result)
    }
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Budget {
    pub samples: usize,
    pub peak_private_bytes: u64,
    pub peak_cpu_percent: f64,
    pub passed: bool,
}
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BudgetProfile {
    Interface,
    GameMonitor,
}
impl BudgetProfile {
    pub fn limits(self) -> (u64, f64) {
        match self {
            Self::Interface => (150 * 1024 * 1024, 1.0),
            Self::GameMonitor => (15 * 1024 * 1024, 0.5),
        }
    }
}
pub fn budget(samples: &[Sample]) -> Budget {
    budget_for(samples, BudgetProfile::Interface)
}
pub fn budget_for(samples: &[Sample], profile: BudgetProfile) -> Budget {
    let (ram_limit, cpu_limit) = profile.limits();
    let peak_private_bytes = samples
        .iter()
        .map(|s| s.app_private_bytes)
        .max()
        .unwrap_or(u64::MAX);
    let peak_cpu_percent = samples
        .iter()
        .filter_map(|s| s.app_cpu_percent)
        .fold(0.0, f64::max);
    Budget {
        samples: samples.len(),
        peak_private_bytes,
        peak_cpu_percent,
        passed: samples.len() >= 60
            && samples.windows(2).all(|p| {
                (500..=2000).contains(&p[1].timestamp_ms.saturating_sub(p[0].timestamp_ms))
            })
            && samples
                .first()
                .zip(samples.last())
                .is_some_and(|(a, b)| b.timestamp_ms.saturating_sub(a.timestamp_ms) >= 58500)
            && samples.iter().all(|s| {
                s.app_cpu_percent
                    .is_some_and(|v| v.is_finite() && (0.0..100.0).contains(&v))
            })
            && peak_private_bytes <= ram_limit
            && peak_cpu_percent < cpu_limit,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn raw(t: u64) -> Raw {
        Raw {
            uptime_ms: t * 1000,
            total: 1000,
            available: 400,
            idle: t * 8,
            total_ticks: t * 10,
            boot: "boot".into(),
            power_plan: "plan".into(),
            processes: vec![
                Process {
                    pid: 1,
                    parent: 0,
                    created: 1,
                    name: "GameTune.exe".into(),
                    working_set: 40,
                    private_working_set: 20,
                    cpu_ticks: t,
                },
                Process {
                    pid: 2,
                    parent: 1,
                    created: 2,
                    name: "WebView.exe".into(),
                    working_set: 60,
                    private_working_set: 30,
                    cpu_ticks: t,
                },
                Process {
                    pid: 3,
                    parent: 0,
                    created: 3,
                    name: "webview.EXE".into(),
                    working_set: 10,
                    private_working_set: 10,
                    cpu_ticks: 0,
                },
            ],
        }
    }
    #[test]
    fn first_cpu_is_unknown_and_groups_include_children() -> Result<(), Error> {
        let mut c = Collector::default();
        let first = c.ingest(raw(1), 1)?;
        assert!(first.cpu_percent.is_none());
        assert_eq!(first.app_private_bytes, 50);
        assert_eq!(first.top[0].count, 2);
        assert_eq!(first.app_processes, 2);
        let next = c.ingest(raw(2), 1)?;
        assert_eq!(next.cpu_percent, Some(20.0));
        assert_eq!(next.app_cpu_percent, Some(20.0));
        Ok(())
    }
    #[test]
    fn pid_reuse_does_not_count_as_same_root() -> Result<(), Error> {
        let mut c = Collector::default();
        c.ingest(raw(1), 1)?;
        let mut reused = raw(2);
        reused.processes[0].created = 90;
        assert!(c.ingest(reused, 1).is_err());
        Ok(())
    }
    #[test]
    fn newborn_child_cpu_is_counted_in_its_first_interval() -> Result<(), Error> {
        let mut c = Collector::default();
        c.ingest(raw(1), 1)?;
        let mut next = raw(2);
        next.processes.push(Process {
            pid: 4,
            parent: 1,
            created: 4,
            name: "child.exe".into(),
            working_set: 1,
            private_working_set: 1,
            cpu_ticks: 2,
        });
        assert_eq!(c.ingest(next, 1)?.app_cpu_percent, Some(40.0));
        Ok(())
    }
    #[test]
    fn budget_fails_memory_cpu_and_insufficient_samples() -> Result<(), Error> {
        let mut c = Collector::default();
        c.ingest(raw(1), 1)?;
        let mut s = c.ingest(raw(2), 1)?;
        s.app_cpu_percent = Some(0.2);
        let mut samples: Vec<_> = (0..60)
            .map(|i| {
                let mut sample = s.clone();
                sample.timestamp_ms = i * 1000;
                sample
            })
            .collect();
        assert!(budget(&samples).passed);
        samples[4].app_private_bytes = 150 * 1024 * 1024 + 1;
        assert!(!budget(&samples).passed);
        samples[4].app_private_bytes = 50;
        samples[5].app_cpu_percent = Some(1.1);
        assert!(!budget(&samples).passed);
        assert!(!budget(&[]).passed);
        Ok(())
    }
    #[test]
    fn independent_resource_profiles_have_closed_inclusive_ram_and_strict_cpu_limits()
    -> Result<(), Error> {
        let mut c = Collector::default();
        c.ingest(raw(1), 1)?;
        let mut sample = c.ingest(raw(2), 1)?;
        sample.app_cpu_percent = Some(0.49);
        sample.app_private_bytes = 15 * 1024 * 1024;
        let mut samples: Vec<_> = (0..60)
            .map(|i| {
                let mut s = sample.clone();
                s.timestamp_ms = i * 1000;
                s
            })
            .collect();
        assert!(budget_for(&samples, BudgetProfile::GameMonitor).passed);
        samples[0].app_cpu_percent = Some(0.5);
        assert!(!budget_for(&samples, BudgetProfile::GameMonitor).passed);
        samples[0].app_cpu_percent = Some(0.49);
        samples[0].app_private_bytes += 1;
        assert!(!budget_for(&samples, BudgetProfile::GameMonitor).passed);
        for s in &mut samples {
            s.app_private_bytes = 150 * 1024 * 1024;
            s.app_cpu_percent = Some(0.99);
        }
        assert!(budget_for(&samples, BudgetProfile::Interface).passed);
        samples[0].app_private_bytes += 1;
        assert!(!budget_for(&samples, BudgetProfile::Interface).passed);
        Ok(())
    }
}
