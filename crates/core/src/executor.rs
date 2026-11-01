use crate::{ActionType, Item};
use serde::{Deserialize, Serialize};

/// Raw registry bytes and kind preserve REG_SZ, REG_BINARY, QWORD and missing values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegistryValue {
    pub kind: u32,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Snapshot {
    Registry {
        value: Option<RegistryValue>,
        created_keys: Vec<String>,
    },
    Service {
        start_mode: u32,
        delayed_auto_start: Option<bool>,
        running: bool,
    },
    ScheduledTask {
        enabled: bool,
        #[serde(default)]
        definition: Option<String>,
    },
    Appx {
        packages: Vec<String>,
        provisioned: Vec<String>,
        families: Vec<String>,
    },
    Startup {
        approved: Option<RegistryValue>,
        created_keys: Vec<String>,
    },
    Power {
        active_guid: String,
        existing_guids: Vec<String>,
    },
}

impl Snapshot {
    pub fn action_type(&self) -> ActionType {
        match self {
            Self::Registry { .. } => ActionType::Registry,
            Self::Service { .. } => ActionType::Service,
            Self::ScheduledTask { .. } => ActionType::ScheduledTask,
            Self::Appx { .. } => ActionType::Appx,
            Self::Startup { .. } => ActionType::Startup,
            Self::Power { .. } => ActionType::Power,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
#[serde(tag = "code", content = "detail", rename_all = "snake_case")]
pub enum ExecError {
    #[error("resource not found")]
    NotFound,
    #[error("access denied")]
    AccessDenied,
    #[error("executor type mismatch")]
    TypeMismatch,
    #[error("executor is unavailable on this platform")]
    Unavailable,
    #[error("resource changed after analysis; analyze again before applying")]
    Conflict,
    #[error("operation timed out; inspect the recovery journal before retrying")]
    Timeout,
    #[error("explicit per-app removal acknowledgement is required")]
    RemovalConfirmationRequired,
    #[error("protected resource; catalog inclusion does not authorize execution")]
    Protected,
    #[error("manual reinstall required")]
    ManualReinstallRequired,
    #[error("operation failed: {0}")]
    Failed(String),
}

pub type ExecResult<T> = std::result::Result<T, ExecError>;

/// Implementations capture state without mutation; restore must be idempotent.
/// A failure from apply may follow a partial change: the original must remain recoverable.
pub trait Executor {
    fn execution_scope(&self) -> Option<&str> {
        None
    }
    fn ensure_write_ready(&self) -> ExecResult<()> {
        Ok(())
    }
    fn action_type(&self) -> ActionType;
    fn capture(&self, item: &Item) -> ExecResult<Snapshot>;
    fn apply(&mut self, item: &Item) -> ExecResult<()>;
    /// Service pauses must be implemented explicitly, never fall back to Disabled.
    fn apply_temporary(&mut self, item: &Item) -> ExecResult<()> {
        if item.action_type == ActionType::Service {
            Err(ExecError::Unavailable)
        } else {
            self.apply(item)
        }
    }
    fn restore(&mut self, item: &Item, original: &Snapshot) -> ExecResult<()>;
}

#[derive(Default)]
pub struct Executors {
    executors: std::collections::BTreeMap<ActionType, Box<dyn Executor>>,
}

impl Executors {
    pub fn insert(&mut self, executor: impl Executor + 'static) {
        self.executors
            .insert(executor.action_type(), Box::new(executor));
    }
    pub fn get(&self, kind: ActionType) -> ExecResult<&dyn Executor> {
        self.executors
            .get(&kind)
            .map(|e| e.as_ref())
            .ok_or(ExecError::Unavailable)
    }
    pub fn get_mut(&mut self, kind: ActionType) -> ExecResult<&mut (dyn Executor + '_)> {
        match self.executors.get_mut(&kind) {
            Some(executor) => Ok(executor.as_mut()),
            None => Err(ExecError::Unavailable),
        }
    }
}
