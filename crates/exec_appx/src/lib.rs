#![forbid(unsafe_code)]
//! Appx executor, backed by validated Windows operations or an explicit mock.
use gametune_core::{ActionType, ExecResult, Executor, Item, Snapshot};
use gametune_exec_mock::MockHost;

pub struct MockExecutor(gametune_exec_mock::MockExecutor);
impl MockExecutor {
    pub fn new(host: MockHost) -> Self {
        Self(gametune_exec_mock::MockExecutor::new(
            ActionType::Appx,
            host,
        ))
    }
}
impl Executor for MockExecutor {
    fn action_type(&self) -> ActionType {
        ActionType::Appx
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

/// Real Windows adapter with validated catalog targets and interactive-user scope.
pub struct WindowsExecutor(gametune_windows_runtime::WindowsExecutor);
impl WindowsExecutor {
    pub fn new(context: gametune_windows_runtime::WindowsContext) -> Self {
        Self(gametune_windows_runtime::WindowsExecutor::new(
            ActionType::Appx,
            context,
        ))
    }
}
impl Executor for WindowsExecutor {
    fn action_type(&self) -> ActionType {
        ActionType::Appx
    }
    fn execution_scope(&self) -> Option<&str> {
        self.0.execution_scope()
    }
    fn ensure_write_ready(&self) -> ExecResult<()> {
        self.0.ensure_write_ready()
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
