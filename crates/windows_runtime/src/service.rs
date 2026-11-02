use ametista_core::ExecResult;
use serde::de::DeserializeOwned;
pub(crate) fn run<T: DeserializeOwned>(request: &serde_json::Value) -> ExecResult<T> {
    #[cfg(windows)]
    {
        serde_json::from_value(crate::native::service(request)?).map_err(crate::failed)
    }
    #[cfg(not(windows))]
    {
        let _ = request;
        Err(ametista_core::ExecError::Unavailable)
    }
}

pub(crate) fn write(request: &serde_json::Value) -> ExecResult<()> {
    let success: bool = run(request)?;
    if success {
        Ok(())
    } else {
        Err(ametista_core::ExecError::Conflict)
    }
}
