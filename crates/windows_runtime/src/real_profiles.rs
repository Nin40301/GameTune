//! Extra opt-in for real catalog/reboot acceptance. Absent from production builds.
use crate::{
    WindowsContext,
    transport::{self, Script},
};
use gametune_core::{ExecError, ExecResult};
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Stage {
    Step,
    Verify,
    Recover,
}
#[derive(Debug, Deserialize)]
pub struct Evidence {
    pub status: String,
    pub evidence: String,
}
pub fn run(stage: Stage) -> ExecResult<Evidence> {
    if std::env::var("GAMETUNE_DISPOSABLE_VM").as_deref() != Ok("I_HAVE_A_SNAPSHOT")
        || std::env::var("GAMETUNE_REAL_PROFILES").as_deref()
            != Ok("I_ACCEPT_REAL_CATALOG_CHANGES_AND_REBOOTS")
    {
        return Err(ExecError::Protected);
    }
    let context = WindowsContext::detect()?;
    context.ensure_elevated()?;
    let exe = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/debug/gametune-cli.exe")
        .canonicalize()
        .map_err(crate::io_error)?;
    transport::run(
        Script::RealProfiles,
        &serde_json::json!({"operation":stage,"sid":context.interactive_sid(),"exe":exe}),
    )
}
