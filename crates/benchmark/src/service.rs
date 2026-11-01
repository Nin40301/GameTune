//! One read-only sampler shared by dashboard and benchmark. No elevated operations.
use crate::{
    Conditions, Error, Phase, Protocol, Result, presentmon,
    stats::{self, Comparison},
    store::Store,
};
use gametune_metrics::{Collector, Sample};
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
struct Active {
    started: Instant,
    conditions: Conditions,
    boot: String,
    samples: Vec<Sample>,
}
struct Inner {
    store: Store,
    protocol: Protocol,
    latest: Option<Sample>,
    monitor: bool,
    active: Option<Active>,
    error: Option<String>,
    presentmon: Option<PathBuf>,
    mutating: bool,
}
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub protocol: Protocol,
    pub latest: Option<Sample>,
    pub monitor: bool,
    pub elapsed_seconds: u64,
    pub error: Option<String>,
    pub comparisons: Vec<Comparison>,
    pub presentmon_available: bool,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Action {
    Status {},
    Monitor { enabled: bool },
    Start { conditions: Conditions },
    Cancel {},
    Reset { confirm: bool },
}
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum FileAction {
    ChoosePresentmon {},
    ImportCapture {},
    Export {
        format: crate::report::Format,
        anonymize: bool,
        english: bool,
    },
}
pub struct Lab {
    inner: Arc<Mutex<Inner>>,
}
pub struct MutationGuard {
    inner: Arc<Mutex<Inner>>,
}
impl Drop for MutationGuard {
    fn drop(&mut self) {
        if let Ok(mut i) = self.inner.lock() {
            i.mutating = false;
        }
    }
}
impl Lab {
    pub fn open(directory: &Path, monitor: bool) -> Result<Self> {
        let store = Store::open(directory)?;
        let mut protocol = store.load()?;
        if protocol.pending_run {
            protocol.resume();
            store.save(&protocol)?;
        }
        let inner = Arc::new(Mutex::new(Inner {
            store,
            protocol,
            latest: None,
            monitor,
            active: None,
            error: None,
            presentmon: None,
            mutating: false,
        }));
        let weak = Arc::downgrade(&inner);
        std::thread::spawn(move || {
            let mut collector = Collector::default();
            let mut tick = 0;
            loop {
                let Some(inner) = weak.upgrade() else {
                    break;
                };
                let (needed, disabled) = match inner.lock() {
                    Ok(i) => (
                        i.active.is_some() || (i.monitor && tick % 2 == 0),
                        i.active.is_none() && !i.monitor,
                    ),
                    Err(_) => break,
                };
                if disabled {
                    collector = Collector::default();
                }
                if needed {
                    let sample = collector.sample();
                    if let Ok(mut i) = inner.lock() {
                        match sample {
                            Ok(sample) => {
                                if i.error
                                    .as_deref()
                                    .is_some_and(|e| e.starts_with("metrics_"))
                                {
                                    i.error = None;
                                }
                                i.latest = Some(sample.clone());
                                let plan = i.protocol.power_plan.clone();
                                if let Some(active) = &mut i.active {
                                    let invalid = sample.boot != active.boot
                                        || Some(&sample.power_plan) != plan.as_ref()
                                        || active.samples.last().is_some_and(|s| {
                                            sample.timestamp_ms.saturating_sub(s.timestamp_ms)
                                                > 2000
                                        });
                                    if invalid {
                                        i.active = None;
                                        i.protocol.cancel();
                                        i.error = Some("benchmark_invalid_run".into());
                                        if i.store.save(&i.protocol).is_err() {
                                            i.error = Some("benchmark_storage".into());
                                        }
                                    } else if sample.cpu_percent.is_some() {
                                        active.samples.push(sample);
                                        if active.started.elapsed() >= Duration::from_secs(60) {
                                            let run = i.active.take();
                                            if let Some(run) = run {
                                                let mut p = i.protocol.clone();
                                                let result = p
                                                    .finish(
                                                        run.samples,
                                                        run.started.elapsed().as_millis() as u64,
                                                        run.conditions,
                                                    )
                                                    .and_then(|()| i.store.save(&p));
                                                match result {
                                                    Ok(()) => i.protocol = p,
                                                    Err(e) => {
                                                        i.protocol.cancel();
                                                        i.error = Some(e.to_string());
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                i.latest = None;
                                i.error = Some(e.to_string());
                                if i.active.take().is_some() {
                                    i.protocol.cancel();
                                    let _ = i.store.save(&i.protocol);
                                }
                            }
                        }
                    }
                }
                drop(inner);
                tick += 1;
                std::thread::sleep(Duration::from_secs(1));
            }
        });
        Ok(Self { inner })
    }
    pub fn status(&self) -> Result<Status> {
        let i = self.inner.lock().map_err(|_| Error::State)?;
        let protocol = i.protocol.summary();
        Ok(Status {
            protocol,
            latest: i.latest.clone(),
            monitor: i.monitor,
            elapsed_seconds: i
                .active
                .as_ref()
                .map_or(0, |a| a.started.elapsed().as_secs().min(60)),
            error: i.error.clone(),
            comparisons: stats::comparisons(&i.protocol),
            presentmon_available: i
                .presentmon
                .as_ref()
                .is_some_and(|p| presentmon::installed(p)),
        })
    }
    pub fn protocol(&self) -> Result<Protocol> {
        Ok(self
            .inner
            .lock()
            .map_err(|_| Error::State)?
            .protocol
            .clone())
    }
    pub fn action(&self, action: Action) -> Result<Status> {
        {
            let mut i = self.inner.lock().map_err(|_| Error::State)?;
            match action {
                Action::Status {} => {}
                Action::Monitor { enabled } => {
                    if !enabled && i.active.is_some() {
                        return Err(Error::State);
                    }
                    if enabled && !i.monitor {
                        i.latest = None;
                    }
                    i.monitor = enabled;
                }
                Action::Start { conditions } => {
                    if i.mutating {
                        return Err(Error::State);
                    }
                    let sample = i.latest.clone().ok_or(Error::InvalidRun)?;
                    if sample.cpu_percent.is_none() {
                        return Err(Error::InvalidRun);
                    }
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map_err(|_| Error::State)?
                        .as_millis() as u64;
                    if now.saturating_sub(sample.timestamp_ms) > 3000 {
                        return Err(Error::InvalidRun);
                    }
                    let mut p = i.protocol.clone();
                    p.begin(&sample, &conditions)?;
                    i.store.save(&p)?;
                    i.protocol = p;
                    i.error = None;
                    i.active = Some(Active {
                        started: Instant::now(),
                        conditions,
                        boot: sample.boot,
                        samples: Vec::new(),
                    });
                }
                Action::Cancel {} => {
                    i.active = None;
                    let mut p = i.protocol.clone();
                    p.cancel();
                    i.store.save(&p)?;
                    i.protocol = p;
                }
                Action::Reset { confirm } => {
                    if !confirm || i.active.is_some() {
                        return Err(Error::State);
                    }
                    let p = Protocol::new();
                    i.store.save(&p)?;
                    i.protocol = p;
                    i.error = None;
                }
            }
        }
        self.status()
    }
    /// Prevent concurrent system changes while sampling. The elevated helper remains
    /// responsible for catalog validation; this guard only protects the experiment.
    pub fn guard_mutation(&self) -> Result<MutationGuard> {
        let mut i = self.inner.lock().map_err(|_| Error::State)?;
        if i.active.is_some() || i.mutating {
            return Err(Error::State);
        }
        i.mutating = true;
        Ok(MutationGuard {
            inner: self.inner.clone(),
        })
    }
    pub fn record_applied(&self, ids: Vec<String>) -> Result<()> {
        let mut i = self.inner.lock().map_err(|_| Error::State)?;
        if i.protocol.phase == Phase::AwaitApply {
            let mut p = i.protocol.clone();
            let sample = i.latest.as_ref().ok_or(Error::InvalidRun)?;
            p.applied(ids, sample.uptime_ms)?;
            // Use boot of the actual application, even if Before spanned a reboot.
            p.baseline_boot = i
                .latest
                .as_ref()
                .map(|s| s.boot.clone())
                .or(p.baseline_boot);
            if let Err(e) = i.store.save(&p) {
                i.error = Some(e.to_string());
                return Err(e);
            }
            i.protocol = p;
        }
        Ok(())
    }
    pub fn choose_presentmon(&self, path: PathBuf) -> Result<()> {
        if !presentmon::installed(&path) {
            return Err(Error::PresentMon);
        }
        self.inner.lock().map_err(|_| Error::State)?.presentmon = Some(path);
        Ok(())
    }
    pub fn import_capture(&self, path: &Path) -> Result<()> {
        let mut i = self.inner.lock().map_err(|_| Error::State)?;
        if !i
            .presentmon
            .as_ref()
            .is_some_and(|p| presentmon::installed(p))
            || i.protocol.presentmon.len() >= 10
        {
            return Err(Error::PresentMon);
        }
        let file = std::fs::File::open(path).map_err(|_| Error::PresentMon)?;
        let capture = presentmon::parse(file)?;
        let mut p = i.protocol.clone();
        p.presentmon.push(capture);
        i.store.save(&p)?;
        i.protocol = p;
        Ok(())
    }
}
