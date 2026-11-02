//! Read-only support metadata. Never infers enabled protection from old restore points.
use crate::{ExecResult, WindowsContext};
use serde::Serialize;
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfo {
    pub windows_build: u32,
    pub edition: ametista_core::Edition,
    pub protection: &'static str,
    pub legacy_data: bool,
}
impl WindowsContext {
    pub fn system_info(&self) -> ExecResult<SystemInfo> {
        let legacy_data =
            match crate::refuse_legacy_data(std::path::Path::new(&self.identity.program_data)) {
                Ok(()) => false,
                Err(ametista_core::ExecError::Failed(ref text)) if text == "legacy_data" => true,
                Err(error) => return Err(error),
            };
        #[cfg(windows)]
        let protection = {
            use winreg::{RegKey, enums::*};
            let disabled = RegKey::predef(HKEY_LOCAL_MACHINE)
                .open_subkey_with_flags(
                    r"SOFTWARE\Policies\Microsoft\Windows NT\SystemRestore",
                    KEY_READ | KEY_WOW64_64KEY,
                )
                .and_then(|key| key.get_value::<u32, _>("DisableSR"));
            if matches!(disabled, Ok(1)) {
                "disabled_by_policy"
            } else {
                "unknown"
            }
        };
        #[cfg(not(windows))]
        let protection = "unknown";
        Ok(SystemInfo {
            windows_build: self.identity.build,
            edition: self.identity.edition,
            protection,
            legacy_data,
        })
    }
    pub fn logs_directory(&self) -> std::path::PathBuf {
        std::path::Path::new(&self.identity.program_data).join("Ametista/logs")
    }
    pub fn open_support_tool(&self, tool: SupportTool) -> ExecResult<()> {
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            let system = std::path::PathBuf::from(crate::registry::system_root()?);
            let mut command = match tool {
                SupportTool::Protection => std::process::Command::new(
                    system.join("System32/SystemPropertiesProtection.exe"),
                ),
                SupportTool::Logs => {
                    let mut command = std::process::Command::new(system.join("explorer.exe"));
                    command.arg(self.logs_directory());
                    command
                }
                SupportTool::RestartDialog => {
                    let opened: bool = crate::transport::run(
                        crate::transport::Script::RestartDialog,
                        &serde_json::json!({"expectedSid":self.interactive_sid()}),
                    )?;
                    return if opened {
                        Ok(())
                    } else {
                        Err(ametista_core::ExecError::Unavailable)
                    };
                }
            };
            command
                .creation_flags(0x0800_0000)
                .spawn()
                .map_err(crate::io_error)?;
            Ok(())
        }
        #[cfg(not(windows))]
        {
            let _ = tool;
            Err(ametista_core::ExecError::Unavailable)
        }
    }
}
#[derive(Clone, Copy, Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportTool {
    Protection,
    Logs,
    RestartDialog,
}
