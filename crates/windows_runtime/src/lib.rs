#![forbid(unsafe_code)]
//! Fixed Windows operations. This crate never accepts command or script text from callers.
#[cfg(feature = "vm-tests")]
pub mod fixture;
#[cfg(test)]
mod native_tests;
mod platform;
#[cfg(feature = "real-profile-tests")]
pub mod real_profiles;
mod registry;
mod scan;
pub use scan::*;
mod startup;
mod storage;
mod transport;
use ametista_core::{ExecError, ExecResult};
pub use platform::*;
use serde::{Deserialize, Serialize};
pub use storage::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Identity {
    sid: String,
    machine_guid: String,
    session_id: u32,
    elevated: bool,
    file_owner_sid: String,
    program_data: String,
    build: u32,
    edition: ametista_core::Edition,
}

#[derive(Debug, Clone)]
pub struct WindowsContext {
    identity: Identity,
    scope: String,
}
impl WindowsContext {
    pub(crate) fn state_directory(&self) -> std::path::PathBuf {
        std::path::Path::new(&self.identity.program_data)
            .join("Ametista")
            .join(self.interactive_sid())
    }
    #[cfg(all(windows, test))]
    fn current_test_user() -> ExecResult<Self> {
        let identity: Identity = transport::run(
            transport::Script::Identity,
            &serde_json::json!({"testCurrentUser":true}),
        )?;
        let scope = format!("test:{}:{}", identity.machine_guid, identity.sid);
        Ok(Self { identity, scope })
    }
    pub fn detect() -> ExecResult<Self> {
        let identity: Identity =
            transport::run(transport::Script::Identity, &serde_json::json!({}))?;
        if !valid_sid(&identity.sid)
            || identity.session_id == 0
            || !valid_guid(&identity.machine_guid)
        {
            return Err(ExecError::Failed(
                "interactive Windows identity is unavailable".into(),
            ));
        }
        let scope = format!(
            "{}:{}",
            identity.machine_guid.to_ascii_lowercase(),
            identity.sid
        );
        Ok(Self { identity, scope })
    }
    pub fn scope(&self) -> &str {
        &self.scope
    }
    pub fn interactive_sid(&self) -> &str {
        &self.identity.sid
    }
    pub fn elevated(&self) -> bool {
        self.identity.elevated
    }
    pub fn machine(&self) -> ametista_core::Machine {
        ametista_core::Machine {
            build: self.identity.build,
            edition: self.identity.edition,
            disk: ametista_core::DiskKind::Unknown,
        }
    }
    pub fn ensure_elevated(&self) -> ExecResult<()> {
        refuse_legacy_data(std::path::Path::new(&self.identity.program_data))?;
        self.ensure_privileged_identity()
    }
    /// Authenticate the helper before opening its pipe, without starting any write.
    pub fn ensure_privileged_identity(&self) -> ExecResult<()> {
        if self.elevated()
            && ["S-1-5-32-544", "S-1-5-18"].contains(&self.identity.file_owner_sid.as_str())
        {
            Ok(())
        } else {
            Err(ExecError::AccessDenied)
        }
    }
    pub fn create_restore_point(&self) -> ExecResult<RestorePointStatus> {
        self.ensure_elevated()?;
        transport::run(transport::Script::RestorePoint, &serde_json::json!({}))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestorePointStatus {
    Created,
    Recent,
}

pub(crate) fn valid_guid(s: &str) -> bool {
    s.len() == 36
        && s.bytes().enumerate().all(|(i, b)| {
            if [8, 13, 18, 23].contains(&i) {
                b == b'-'
            } else {
                b.is_ascii_hexdigit()
            }
        })
}
pub(crate) fn valid_sid(s: &str) -> bool {
    s.starts_with("S-1-5-")
        && s.len() < 184
        && s[6..].bytes().all(|b| b.is_ascii_digit() || b == b'-')
}
pub(crate) fn failed(e: impl std::fmt::Display) -> ExecError {
    ExecError::Failed(e.to_string())
}
pub(crate) fn io_error(e: std::io::Error) -> ExecError {
    match e.kind() {
        std::io::ErrorKind::NotFound => ExecError::NotFound,
        std::io::ErrorKind::PermissionDenied => ExecError::AccessDenied,
        _ => failed(e),
    }
}

// Exact pre-rename directory name, intentionally separate from the current brand.
fn refuse_legacy_data(program_data: &std::path::Path) -> ExecResult<()> {
    let legacy = program_data.join(concat!("Game", "Tune"));
    match std::fs::symlink_metadata(&legacy) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Ok(_) => Err(ExecError::Failed("legacy_data".into())),
        Err(error) => Err(io_error(error)),
    }
}
#[cfg(test)]
mod rename_tests {
    use super::*;
    #[test]
    fn legacy_data_blocks_writes_without_moving_or_deleting()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = tempfile::tempdir()?;
        refuse_legacy_data(root.path())?;
        let old = root.path().join(concat!("Game", "Tune"));
        std::fs::create_dir(&old)?;
        std::fs::write(old.join("journal.wal"), b"original")?;
        assert_eq!(
            refuse_legacy_data(root.path()),
            Err(ExecError::Failed("legacy_data".into()))
        );
        assert_eq!(std::fs::read(old.join("journal.wal"))?, b"original");
        assert!(!root.path().join("Ametista").exists());
        Ok(())
    }
}
