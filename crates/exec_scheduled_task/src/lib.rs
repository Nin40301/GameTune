#![forbid(unsafe_code)]
//! ScheduledTask executor boundary. Real Windows implementation is a phase 2 deliverable.
use gametune_core::{ActionType, ExecError, ExecResult, Executor, Item, Snapshot};
use gametune_exec_mock::MockHost;

pub struct MockExecutor(gametune_exec_mock::MockExecutor);
impl MockExecutor {
    pub fn new(host: MockHost) -> Self {
        Self(gametune_exec_mock::MockExecutor::new(
            ActionType::ScheduledTask,
            host,
        ))
    }
}
impl Executor for MockExecutor {
    fn action_type(&self) -> ActionType {
        ActionType::ScheduledTask
    }
    fn capture(&self, item: &Item) -> ExecResult<Snapshot> {
        self.0.capture(item)
    }
    fn apply(&mut self, item: &Item) -> ExecResult<()> {
        self.0.apply(item)
    }
    fn restore(&mut self, item: &Item, original: &Snapshot) -> ExecResult<()> {
        self.0.restore(item, original)
    }
}

/// Fails closed on every OS, including Windows. Never silently substitutes a mock.
pub struct WindowsExecutor;
impl Executor for WindowsExecutor {
    fn action_type(&self) -> ActionType {
        ActionType::ScheduledTask
    }
    fn capture(&self, _item: &Item) -> ExecResult<Snapshot> {
        Err(ExecError::Unavailable)
    }
    fn apply(&mut self, _item: &Item) -> ExecResult<()> {
        Err(ExecError::Unavailable)
    }
    fn restore(&mut self, _item: &Item, _original: &Snapshot) -> ExecResult<()> {
        Err(ExecError::Unavailable)
    }
}
