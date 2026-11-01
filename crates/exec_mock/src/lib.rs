#![forbid(unsafe_code)]
//! Deterministic fake Windows state, shared by all executor kinds. No OS mutations.

use gametune_core::{
    ActionType, ExecError, ExecResult, Executor, Item, RegistryValue, ServiceMode, Snapshot, Target,
};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex, MutexGuard},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stage {
    Capture,
    Apply,
    Restore,
    AfterApply,
}

#[derive(Default)]
struct State {
    resources: BTreeMap<String, Snapshot>,
    failures: BTreeMap<(String, Stage), ExecError>,
    writes: usize,
}

#[derive(Clone, Default)]
pub struct MockHost(Arc<Mutex<State>>);

impl MockHost {
    fn lock(&self) -> ExecResult<MutexGuard<'_, State>> {
        self.0
            .lock()
            .map_err(|_| ExecError::Failed("mock state poisoned".into()))
    }
    pub fn seed(&self, item: &Item, state: Snapshot) -> ExecResult<()> {
        if item.action_type != state.action_type() {
            return Err(ExecError::TypeMismatch);
        }
        self.lock()?.resources.insert(item.target.key(), state);
        Ok(())
    }
    pub fn snapshot(&self) -> ExecResult<BTreeMap<String, Snapshot>> {
        Ok(self.lock()?.resources.clone())
    }
    pub fn writes(&self) -> ExecResult<usize> {
        Ok(self.lock()?.writes)
    }
    pub fn fail(&self, item: &Item, stage: Stage, error: ExecError) -> ExecResult<()> {
        self.lock()?
            .failures
            .insert((item.target.key(), stage), error);
        Ok(())
    }
    pub fn clear_failures(&self) -> ExecResult<()> {
        self.lock()?.failures.clear();
        Ok(())
    }
}

pub struct MockExecutor {
    kind: ActionType,
    host: MockHost,
}

impl MockExecutor {
    pub fn new(kind: ActionType, host: MockHost) -> Self {
        Self { kind, host }
    }
    fn check(&self, item: &Item) -> ExecResult<()> {
        if item.action_type != self.kind {
            return Err(ExecError::TypeMismatch);
        }
        Ok(())
    }
}

impl Executor for MockExecutor {
    fn action_type(&self) -> ActionType {
        self.kind
    }
    fn capture(&self, item: &Item) -> ExecResult<Snapshot> {
        self.check(item)?;
        let state = self.host.lock()?;
        if let Some(error) = state.failures.get(&(item.target.key(), Stage::Capture)) {
            return Err(error.clone());
        }
        let snapshot = state
            .resources
            .get(&item.target.key())
            .cloned()
            .ok_or(ExecError::NotFound)?;
        if matches!(&snapshot,Snapshot::Appx{packages,provisioned,..} if packages.is_empty()&&provisioned.is_empty())
        {
            return Err(ExecError::NotFound);
        }
        Ok(snapshot)
    }
    fn apply(&mut self, item: &Item) -> ExecResult<()> {
        self.check(item)?;
        let mut state = self.host.lock()?;
        let key = item.target.key();
        if let Some(error) = state.failures.get(&(key.clone(), Stage::Apply)) {
            return Err(error.clone());
        }
        let current = state
            .resources
            .get(&key)
            .cloned()
            .ok_or(ExecError::NotFound)?;
        let changed = match (&item.target, current) {
            (Target::Registry { value, .. }, Snapshot::Registry { created_keys, .. }) => {
                Snapshot::Registry {
                    value: Some(RegistryValue {
                        kind: 4,
                        bytes: value.to_le_bytes().to_vec(),
                    }),
                    created_keys,
                }
            }
            (
                Target::Service { mode, .. },
                Snapshot::Service {
                    delayed_auto_start, ..
                },
            ) => Snapshot::Service {
                start_mode: if *mode == ServiceMode::Disabled { 4 } else { 3 },
                delayed_auto_start,
                running: false,
            },
            (Target::ScheduledTask { .. }, Snapshot::ScheduledTask { definition, .. }) => {
                Snapshot::ScheduledTask {
                    enabled: false,
                    definition,
                }
            }
            (Target::Appx { .. }, Snapshot::Appx { .. }) => Snapshot::Appx {
                packages: Vec::new(),
                provisioned: Vec::new(),
                families: Vec::new(),
            },
            (Target::Startup { .. }, Snapshot::Startup { created_keys, .. }) => Snapshot::Startup {
                approved: Some(RegistryValue {
                    kind: 3,
                    bytes: vec![3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                }),
                created_keys,
            },
            (
                Target::Power { guid },
                Snapshot::Power {
                    mut existing_guids, ..
                },
            ) => {
                if !existing_guids.contains(guid) {
                    existing_guids.push(guid.clone());
                }
                Snapshot::Power {
                    active_guid: guid.clone(),
                    existing_guids,
                }
            }
            _ => return Err(ExecError::TypeMismatch),
        };
        state.resources.insert(key.clone(), changed);
        state.writes += 1;
        if let Some(error) = state.failures.get(&(key, Stage::AfterApply)) {
            return Err(error.clone());
        }
        Ok(())
    }
    fn restore(&mut self, item: &Item, original: &Snapshot) -> ExecResult<()> {
        self.check(item)?;
        if original.action_type() != self.kind {
            return Err(ExecError::TypeMismatch);
        }
        let mut state = self.host.lock()?;
        let key = item.target.key();
        if let Some(error) = state.failures.get(&(key.clone(), Stage::Restore)) {
            return Err(error.clone());
        }
        if self.kind == ActionType::Appx {
            return Err(ExecError::ManualReinstallRequired);
        }
        state.resources.insert(key, original.clone());
        state.writes += 1;
        Ok(())
    }
}
