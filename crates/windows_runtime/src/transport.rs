use ametista_core::{ExecError, ExecResult};
use serde::{Serialize, de::DeserializeOwned};
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone, Copy)]
pub(crate) enum Script {
    #[cfg(all(test, windows))]
    Parse,
    #[cfg(all(test, windows))]
    CancelProbe,
    Identity,
    Service,
    Task,
    Power,
    Appx,
    AppxInventory,
    Storage,
    RestorePoint,
    Store,
    #[cfg(feature = "vm-tests")]
    Fixture,
    #[cfg(feature = "real-profile-tests")]
    RealProfiles,
}
impl Script {
    fn text(self) -> &'static str {
        match self {
            #[cfg(all(test, windows))]
            Self::Parse => include_str!("../scripts/parse.ps1"),
            #[cfg(all(test, windows))]
            Self::CancelProbe => "Start-Sleep -Seconds 30; Emit $true",
            Self::Identity => include_str!("../scripts/identity.ps1"),
            Self::Service => include_str!("../scripts/service.ps1"),
            Self::Task => include_str!("../scripts/task.ps1"),
            Self::Power => include_str!("../scripts/power.ps1"),
            Self::Appx => include_str!("../scripts/appx.ps1"),
            Self::AppxInventory => include_str!("../scripts/appx_inventory.ps1"),
            Self::Storage => include_str!("../scripts/storage.ps1"),
            Self::RestorePoint => include_str!("../scripts/restore_point.ps1"),
            Self::Store => include_str!("../scripts/store.ps1"),
            #[cfg(feature = "vm-tests")]
            Self::Fixture => include_str!("../scripts/fixture.ps1"),
            #[cfg(feature = "real-profile-tests")]
            Self::RealProfiles => include_str!("../scripts/real_profiles.ps1"),
        }
    }
    fn timeout(self) -> std::time::Duration {
        std::time::Duration::from_secs(match self {
            Self::Appx | Self::AppxInventory | Self::RestorePoint => 180,
            #[cfg(feature = "real-profile-tests")]
            Self::RealProfiles => 1800,
            _ => 45,
        })
    }
}

#[derive(serde::Deserialize)]
struct Envelope<T> {
    ok: bool,
    value: Option<T>,
    error: Option<ExecError>,
}

pub(crate) fn run<T: DeserializeOwned>(script: Script, params: &impl Serialize) -> ExecResult<T> {
    run_controlled(script, params, None)
}
pub(crate) fn run_controlled<T: DeserializeOwned>(
    script: Script,
    params: &impl Serialize,
    cancel: Option<&AtomicBool>,
) -> ExecResult<T> {
    if cancel.is_some_and(|flag| flag.load(Ordering::Acquire)) {
        return Err(ExecError::Conflict);
    }
    let bytes = run_bytes(
        script,
        &serde_json::to_vec(params).map_err(crate::failed)?,
        cancel,
    )?;
    let envelope: Envelope<T> = serde_json::from_slice(&bytes)
        .map_err(|_| ExecError::Failed("invalid fixed-script response".into()))?;
    if envelope.ok {
        envelope
            .value
            .ok_or_else(|| ExecError::Failed("missing fixed-script result".into()))
    } else {
        Err(envelope
            .error
            .unwrap_or_else(|| ExecError::Failed("fixed Windows operation failed".into())))
    }
}

#[cfg(not(windows))]
fn run_bytes(script: Script, _: &[u8], _: Option<&AtomicBool>) -> ExecResult<Vec<u8>> {
    let _ = (script.text(), script.timeout());
    Err(ExecError::Unavailable)
}

#[cfg(windows)]
fn run_bytes(script: Script, params: &[u8], cancel: Option<&AtomicBool>) -> ExecResult<Vec<u8>> {
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    use std::{
        io::{Read, Write},
        os::windows::process::CommandExt,
        process::{Command, Stdio},
        time::Instant,
    };
    // SystemRoot comes from protected HKLM, not user-controlled PATH/environment.
    let system = crate::registry::system_root()?;
    let executable =
        std::path::Path::new(&system).join("System32/WindowsPowerShell/v1.0/powershell.exe");
    // Only this private enum can select executable source. User values stay JSON data.
    // The constant bootstrap avoids CreateProcess's 32K command-line limit for fixtures.
    let code = "$ErrorActionPreference='Stop';[Console]::InputEncoding=[Text.UTF8Encoding]::new($false);[Console]::OutputEncoding=[Text.UTF8Encoding]::new($false);$packet=[Console]::In.ReadToEnd()|ConvertFrom-Json;$p=$packet.parameters;& ([scriptblock]::Create([string]$packet.fixedSource))";
    let fixed_source = format!(
        "{}\n{}\ntry {{\n{}\n}} catch {{\n{}\n}}",
        include_str!("../scripts/prelude.ps1")
            .replace("$p = [Console]::In.ReadToEnd() | ConvertFrom-Json", ""),
        include_str!("../scripts/desktop.ps1"),
        script.text(),
        include_str!("../scripts/catch.ps1")
    );
    let packet=serde_json::to_vec(&serde_json::json!({"parameters":serde_json::from_slice::<serde_json::Value>(params).map_err(crate::failed)?,"fixedSource":fixed_source})).map_err(crate::failed)?;
    let utf16: Vec<u8> = code.encode_utf16().flat_map(u16::to_le_bytes).collect();
    let mut child = Command::new(executable)
        .env(
            "PSModulePath",
            std::path::Path::new(&system).join("System32/WindowsPowerShell/v1.0/Modules"),
        )
        .env("PATH", std::path::Path::new(&system).join("System32"))
        .args([
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-EncodedCommand",
            &STANDARD.encode(utf16),
        ])
        .creation_flags(0x0800_0000)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(crate::io_error)?;
    let mut input = child
        .stdin
        .take()
        .ok_or_else(|| ExecError::Failed("missing stdin".into()))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| ExecError::Failed("missing stdout".into()))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| ExecError::Failed("missing stderr".into()))?;
    // Drain bounded output concurrently; never wait with a full pipe.
    let read = |stream: Box<dyn Read + Send>| {
        std::thread::spawn(move || {
            let mut bytes = Vec::new();
            stream
                .take(4 * 1024 * 1024 + 1)
                .read_to_end(&mut bytes)
                .map(|_| bytes)
        })
    };
    let out = read(Box::new(stdout));
    let err = read(Box::new(stderr));
    if let Err(e) = input.write_all(&packet) {
        let _ = child.kill();
        let _ = child.wait();
        let _ = out.join();
        let _ = err.join();
        return Err(crate::io_error(e));
    }
    drop(input);
    let start = Instant::now();
    let status = loop {
        if cancel.is_some_and(|flag| flag.load(Ordering::Acquire)) {
            // Only our own read-only PowerShell child is terminated. No process lookup.
            let _ = child.kill();
            let _ = child.wait();
            let _ = out.join();
            let _ = err.join();
            return Err(ExecError::Conflict);
        }
        if let Some(status) = child.try_wait().map_err(crate::io_error)? {
            break status;
        }
        if start.elapsed() > script.timeout() {
            let _ = child.kill();
            let _ = child.wait();
            return Err(ExecError::Timeout);
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    };
    let stdout = out
        .join()
        .map_err(|_| ExecError::Failed("stdout reader failed".into()))?
        .map_err(crate::io_error)?;
    let _stderr = err
        .join()
        .map_err(|_| ExecError::Failed("stderr reader failed".into()))?
        .map_err(crate::io_error)?;
    if !status.success() || stdout.len() > 4 * 1024 * 1024 {
        return Err(ExecError::Failed(
            "fixed Windows operation exited unexpectedly or exceeded output limit".into(),
        ));
    }
    Ok(stdout)
}
#[cfg(all(test, windows))]
mod tests {
    use super::*;
    #[test]
    fn cancellation_stops_our_read_only_child_before_its_deadline()
    -> Result<(), Box<dyn std::error::Error>> {
        let cancel = AtomicBool::new(false);
        let started = std::time::Instant::now();
        std::thread::scope(|scope| {
            scope.spawn(|| {
                std::thread::sleep(std::time::Duration::from_millis(300));
                cancel.store(true, Ordering::Release);
            });
            let result: ExecResult<bool> =
                run_controlled(Script::CancelProbe, &serde_json::json!({}), Some(&cancel));
            assert!(matches!(result, Err(ExecError::Conflict)));
        });
        assert!(started.elapsed() < std::time::Duration::from_secs(5));
        Ok(())
    }
    #[test]
    fn embedded_powershell_sources_parse() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let sources = [
            include_str!("../scripts/prelude.ps1"),
            include_str!("../scripts/catch.ps1"),
            include_str!("../scripts/desktop.ps1"),
            Script::Identity.text(),
            Script::Service.text(),
            Script::Task.text(),
            Script::Power.text(),
            Script::Appx.text(),
            Script::AppxInventory.text(),
            Script::Storage.text(),
            Script::RestorePoint.text(),
            Script::Store.text(),
            include_str!("../scripts/fixture.ps1"),
            include_str!("../scripts/real_profiles.ps1"),
        ];
        let fixture = include_str!("../scripts/fixture.ps1");
        let csharp = fixture
            .split("Add-Type -TypeDefinition @'\n")
            .nth(1)
            .and_then(|s| s.split("\n'@").next())
            .ok_or("missing fixed service-helper source")?;
        let valid: bool = run(
            Script::Parse,
            &serde_json::json!({"sources":sources,"serviceHelper":csharp,"helpers":[
                include_str!("../scripts/desktop.ps1").split("Add-Type -TypeDefinition @'\n").nth(1).and_then(|s|s.split("\n'@").next()).ok_or("desktop FFI source")?,
                include_str!("../scripts/real_profiles.ps1").split("Add-Type -TypeDefinition @'\n").nth(1).and_then(|s|s.split("\n'@").next()).ok_or("VM window FFI source")?
            ]}),
        )?;
        assert!(valid);
        Ok(())
    }
}
