//! Audited Win32 boundary: own token/session, known folders, and SCM services only.
//! Never opens a process handle (including explorer/games). No commands supplied by UI.
use crate::{Identity, failed, io_error};
use ametista_core::{Edition, ExecError, ExecResult, Snapshot};
use std::{
    os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle},
    ptr,
};
use windows_sys::Win32::{
    Foundation::*,
    Security::{Authorization::*, *},
    System::{Com::CoTaskMemFree, RemoteDesktop::*, Services::*, Threading::*},
    UI::Shell::*,
};
use winreg::{RegKey, enums::*};
fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain([0]).collect()
}
fn check(ok: i32) -> ExecResult<()> {
    if ok == 0 {
        Err(io_error(std::io::Error::last_os_error()))
    } else {
        Ok(())
    }
}
fn token_information(
    token: &OwnedHandle,
    class: TOKEN_INFORMATION_CLASS,
) -> ExecResult<Vec<usize>> {
    let mut size = 0;
    // SAFETY: own token, size query with no output buffer, then aligned bounded buffer.
    unsafe {
        GetTokenInformation(
            token.as_raw_handle().cast(),
            class,
            ptr::null_mut(),
            0,
            &mut size,
        );
    }
    if size == 0 || size > 65536 {
        return Err(ExecError::Protected);
    }
    let mut buf = vec![0usize; (size as usize).div_ceil(std::mem::size_of::<usize>())];
    unsafe {
        check(GetTokenInformation(
            token.as_raw_handle().cast(),
            class,
            buf.as_mut_ptr().cast(),
            size,
            &mut size,
        ))?;
    }
    Ok(buf)
}
fn token_sid(token: &OwnedHandle, class: TOKEN_INFORMATION_CLASS) -> ExecResult<String> {
    let info = token_information(token, class)?;
    // TOKEN_OWNER and TOKEN_USER both begin with a SID pointer. Buffer stays alive.
    let sid = unsafe { *info.as_ptr().cast::<PSID>() };
    let mut text = ptr::null_mut();
    unsafe {
        check(ConvertSidToStringSidW(sid, &mut text))?;
    }
    let mut len = 0;
    unsafe {
        while *text.add(len) != 0 && len < 184 {
            len += 1;
        }
    }
    let result = unsafe { String::from_utf16_lossy(std::slice::from_raw_parts(text, len)) };
    unsafe {
        LocalFree(text.cast());
    }
    if !crate::valid_sid(&result) {
        return Err(ExecError::Protected);
    }
    Ok(result)
}
fn session_text(session: u32, class: WTS_INFO_CLASS) -> ExecResult<String> {
    let mut text = ptr::null_mut();
    let mut size = 0;
    // SAFETY: only local current session; WTS allocation is copied then freed once.
    unsafe {
        check(WTSQuerySessionInformationW(
            ptr::null_mut(),
            session,
            class,
            &mut text,
            &mut size,
        ))?;
    }
    if text.is_null() || !(2..=65536).contains(&size) {
        if !text.is_null() {
            unsafe {
                WTSFreeMemory(text.cast());
            }
        }
        return Err(ExecError::Protected);
    }
    let words = unsafe { std::slice::from_raw_parts(text, size as usize / 2) };
    let len = words.iter().position(|c| *c == 0).unwrap_or(words.len());
    let result = String::from_utf16_lossy(&words[..len]);
    unsafe {
        WTSFreeMemory(text.cast());
    }
    Ok(result)
}
pub(crate) fn identity(test_current_user: bool) -> ExecResult<Identity> {
    let mut raw = ptr::null_mut();
    // SAFETY: pseudo-handle of this process only, never a game/process lookup.
    unsafe {
        check(OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut raw))?;
    }
    let token = unsafe { OwnedHandle::from_raw_handle(raw.cast()) };
    let mut session = 0;
    unsafe {
        check(ProcessIdToSessionId(GetCurrentProcessId(), &mut session))?;
    }
    if session == 0 && !test_current_user {
        return Err(ExecError::Failed(
            "interactive Windows session unavailable".into(),
        ));
    }
    let sid = if test_current_user {
        token_sid(&token, TokenUser)?
    } else {
        let user = session_text(session, WTSUserName)?;
        let domain = session_text(session, WTSDomainName)?;
        if user.is_empty() {
            return Err(ExecError::Protected);
        }
        // Resolve ONLY locally loaded hives. No account lookup/domain network request,
        // no WTSQueryUserToken (requires SYSTEM), no administrator HKCU fallback.
        let users = RegKey::predef(HKEY_USERS);
        let mut matches = Vec::new();
        for sid in users.enum_keys() {
            let sid = sid.map_err(io_error)?;
            if !crate::valid_sid(&sid) {
                continue;
            }
            let env = match users.open_subkey_with_flags(
                format!(r"{sid}\Volatile Environment"),
                KEY_READ | KEY_WOW64_64KEY,
            ) {
                Ok(key) => key,
                Err(_) => continue,
            };
            let name: String = env.get_value("USERNAME").unwrap_or_default();
            let account_domain: String = env.get_value("USERDOMAIN").unwrap_or_default();
            if name.eq_ignore_ascii_case(&user) && account_domain.eq_ignore_ascii_case(&domain) {
                matches.push(sid);
            }
        }
        if matches.len() != 1 {
            return Err(ExecError::Failed("interactive_identity_unavailable".into()));
        }
        matches.remove(0)
    };
    let machine = RegKey::predef(HKEY_LOCAL_MACHINE);
    let version = machine
        .open_subkey_with_flags(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
            KEY_READ | KEY_WOW64_64KEY,
        )
        .map_err(io_error)?;
    let sku: String = version.get_value("EditionID").map_err(io_error)?;
    let edition = if sku.starts_with("Core") {
        Edition::Home
    } else if sku.starts_with("Professional") {
        Edition::Pro
    } else if sku.starts_with("Enterprise") {
        Edition::Enterprise
    } else if sku.starts_with("Education") {
        Edition::Education
    } else {
        return Err(ExecError::Unavailable);
    };
    let build: String = version.get_value("CurrentBuildNumber").map_err(io_error)?;
    let guid: String = machine
        .open_subkey_with_flags(
            r"SOFTWARE\Microsoft\Cryptography",
            KEY_READ | KEY_WOW64_64KEY,
        )
        .and_then(|key| key.get_value("MachineGuid"))
        .map_err(io_error)?;
    let elevation = token_information(&token, TokenElevation)?;
    let elevated = unsafe { (*elevation.as_ptr().cast::<TOKEN_ELEVATION>()).TokenIsElevated != 0 };
    let mut program_data = ptr::null_mut();
    let hr = unsafe {
        SHGetKnownFolderPath(&FOLDERID_ProgramData, 0, ptr::null_mut(), &mut program_data)
    };
    if hr < 0 {
        return Err(ExecError::Unavailable);
    }
    let mut len = 0;
    unsafe {
        while *program_data.add(len) != 0 {
            len += 1;
        }
    }
    let folder = unsafe { String::from_utf16_lossy(std::slice::from_raw_parts(program_data, len)) };
    unsafe {
        CoTaskMemFree(program_data.cast());
    }
    Ok(Identity {
        sid,
        machine_guid: guid,
        session_id: session,
        elevated,
        file_owner_sid: token_sid(&token, TokenOwner)?,
        program_data: folder,
        build: build.parse().map_err(failed)?,
        edition,
    })
}
struct Service(SC_HANDLE);
impl Drop for Service {
    fn drop(&mut self) {
        unsafe {
            CloseServiceHandle(self.0);
        }
    }
}
fn service_handle(name: &str, write: bool) -> ExecResult<Service> {
    if name.is_empty()
        || name.len() > 160
        || !name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_.-".contains(&b))
    {
        return Err(ExecError::Protected);
    }
    // SAFETY: local SCM, only catalog-authorized service names, minimum rights.
    let manager = Service(unsafe { OpenSCManagerW(ptr::null(), ptr::null(), SC_MANAGER_CONNECT) });
    if manager.0.is_null() {
        return Err(io_error(std::io::Error::last_os_error()));
    }
    let rights = SERVICE_QUERY_CONFIG
        | SERVICE_QUERY_STATUS
        | if write {
            SERVICE_CHANGE_CONFIG | SERVICE_STOP | SERVICE_START | SERVICE_ENUMERATE_DEPENDENTS
        } else {
            0
        };
    let handle = unsafe { OpenServiceW(manager.0, wide(name).as_ptr(), rights) };
    if handle.is_null() {
        let error = std::io::Error::last_os_error();
        return Err(if error.raw_os_error() == Some(1060) {
            ExecError::NotFound
        } else {
            io_error(error)
        });
    }
    Ok(Service(handle))
}
fn service_status(service: &Service) -> ExecResult<u32> {
    let mut status = SERVICE_STATUS_PROCESS::default();
    let mut size = 0;
    // SAFETY: exact initialized OS struct and buffer length, live service handle.
    unsafe {
        check(QueryServiceStatusEx(
            service.0,
            SC_STATUS_PROCESS_INFO,
            (&mut status as *mut SERVICE_STATUS_PROCESS).cast(),
            std::mem::size_of::<SERVICE_STATUS_PROCESS>() as u32,
            &mut size,
        ))?;
    }
    if ![SERVICE_STOPPED, SERVICE_RUNNING].contains(&status.dwCurrentState) {
        return Err(ExecError::Conflict);
    }
    Ok(status.dwCurrentState)
}
fn config(service: &Service) -> ExecResult<Vec<usize>> {
    let mut buffer = vec![0usize; 8192 / std::mem::size_of::<usize>()];
    let mut size = 0;
    // SAFETY: API documents maximum 8KiB, aligned buffer retains embedded strings.
    unsafe {
        check(QueryServiceConfigW(
            service.0,
            buffer.as_mut_ptr().cast(),
            8192,
            &mut size,
        ))?;
    }
    Ok(buffer)
}
fn capture(name: &str, service: &Service) -> ExecResult<Snapshot> {
    let config = config(service)?;
    let config = unsafe { &*config.as_ptr().cast::<QUERY_SERVICE_CONFIGW>() };
    if config.dwServiceType & (SERVICE_WIN32_OWN_PROCESS | SERVICE_WIN32_SHARE_PROCESS) == 0
        || ![2, 3, 4].contains(&config.dwStartType)
    {
        return Err(ExecError::Protected);
    }
    let key = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags(
            format!(r"SYSTEM\CurrentControlSet\Services\{name}"),
            KEY_READ | KEY_WOW64_64KEY,
        )
        .map_err(io_error)?;
    let delayed = match key.get_raw_value("DelayedAutoStart") {
        Ok(value) if value.vtype == REG_DWORD && value.bytes.len() == 4 => {
            let n = u32::from_le_bytes(value.bytes.as_slice().try_into().map_err(failed)?);
            if n > 1 {
                return Err(ExecError::Protected);
            }
            Some(n == 1)
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => return Err(io_error(error)),
        _ => return Err(ExecError::Protected),
    };
    Ok(Snapshot::Service {
        start_mode: config.dwStartType,
        delayed_auto_start: delayed,
        running: service_status(service)? == SERVICE_RUNNING,
    })
}
fn set_mode(service: &Service, mode: u32) -> ExecResult<()> {
    if ![2, 3, 4].contains(&mode) {
        return Err(ExecError::Protected);
    }
    unsafe {
        check(ChangeServiceConfigW(
            service.0,
            SERVICE_NO_CHANGE,
            mode,
            SERVICE_NO_CHANGE,
            ptr::null(),
            ptr::null(),
            ptr::null_mut(),
            ptr::null(),
            ptr::null(),
            ptr::null(),
            ptr::null(),
        ))
    }
}
fn stop(service: &Service) -> ExecResult<()> {
    let mut bytes = 0;
    let mut count = 0;
    // Never cascade to dependencies: any active dependent refuses the change.
    let ok = unsafe {
        EnumDependentServicesW(
            service.0,
            SERVICE_ACTIVE,
            ptr::null_mut(),
            0,
            &mut bytes,
            &mut count,
        )
    };
    if ok == 0 || count > 0 {
        return Err(ExecError::Conflict);
    }
    let mut status = SERVICE_STATUS::default();
    unsafe {
        check(ControlService(service.0, SERVICE_CONTROL_STOP, &mut status))?;
    }
    wait(service, SERVICE_STOPPED)
}
fn wait(service: &Service, desired: u32) -> ExecResult<()> {
    let start = std::time::Instant::now();
    loop {
        let mut status = SERVICE_STATUS_PROCESS::default();
        let mut size = 0;
        unsafe {
            check(QueryServiceStatusEx(
                service.0,
                SC_STATUS_PROCESS_INFO,
                (&mut status as *mut SERVICE_STATUS_PROCESS).cast(),
                std::mem::size_of::<SERVICE_STATUS_PROCESS>() as u32,
                &mut size,
            ))?;
        }
        if status.dwCurrentState == desired {
            return Ok(());
        }
        if start.elapsed().as_secs() >= 20 {
            return Err(ExecError::Conflict);
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
fn start(service: &Service) -> ExecResult<()> {
    let config = config(service)?;
    let dependencies = unsafe { (*config.as_ptr().cast::<QUERY_SERVICE_CONFIGW>()).lpDependencies };
    // QueryServiceConfig's MULTI_SZ lives in config. Reject group dependencies;
    // only start when every explicit dependency is already running.
    if !dependencies.is_null() {
        let base = config.as_ptr() as usize;
        let end = base + config.len() * std::mem::size_of::<usize>();
        let mut pos = dependencies as usize;
        while pos + 2 <= end {
            let mut name = Vec::new();
            loop {
                if pos < base || pos + 2 > end {
                    return Err(ExecError::Protected);
                }
                let word = unsafe { *(pos as *const u16) };
                pos += 2;
                if word == 0 {
                    break;
                }
                name.push(word);
            }
            if name.is_empty() {
                break;
            }
            let name = String::from_utf16(&name).map_err(failed)?;
            if name.starts_with('+')
                || service_status(&service_handle(&name, false)?)? != SERVICE_RUNNING
            {
                return Err(ExecError::Conflict);
            }
        }
    }
    unsafe {
        check(StartServiceW(service.0, 0, ptr::null()))?;
    }
    wait(service, SERVICE_RUNNING)
}
pub(crate) fn service(request: &serde_json::Value) -> ExecResult<serde_json::Value> {
    let name = request["name"].as_str().ok_or(ExecError::Protected)?;
    let operation = request["operation"].as_str().ok_or(ExecError::Protected)?;
    let service = service_handle(name, operation != "capture")?;
    let snapshot = capture(name, &service)?;
    if operation == "capture" {
        return serde_json::to_value(snapshot).map_err(failed);
    }
    if let Some(expected) = request.get("expected").filter(|v| !v.is_null())
        && serde_json::from_value::<Snapshot>(expected.clone()).map_err(failed)? != snapshot
    {
        return Err(ExecError::Conflict);
    }
    let Snapshot::Service { running, .. } = snapshot else {
        return Err(ExecError::TypeMismatch);
    };
    match operation {
        "apply" | "pause" => {
            if running {
                stop(&service)?;
            }
            if operation == "apply" {
                set_mode(
                    &service,
                    match request["mode"].as_str() {
                        Some("manual") => 3,
                        Some("disabled") => 4,
                        _ => return Err(ExecError::Protected),
                    },
                )?;
            }
        }
        "restore" => {
            let original: Snapshot =
                serde_json::from_value(request["original"].clone()).map_err(failed)?;
            let Snapshot::Service {
                start_mode,
                delayed_auto_start,
                running: desired,
            } = original
            else {
                return Err(ExecError::TypeMismatch);
            };
            set_mode(&service, start_mode)?;
            let key = RegKey::predef(HKEY_LOCAL_MACHINE)
                .open_subkey_with_flags(
                    format!(r"SYSTEM\CurrentControlSet\Services\{name}"),
                    KEY_SET_VALUE | KEY_WOW64_64KEY,
                )
                .map_err(io_error)?;
            match delayed_auto_start {
                Some(value) => key
                    .set_value("DelayedAutoStart", &u32::from(value))
                    .map_err(io_error)?,
                None => match key.delete_value("DelayedAutoStart") {
                    Ok(()) => (),
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
                    Err(e) => return Err(io_error(e)),
                },
            }
            if desired && !running {
                if start_mode == 4 {
                    set_mode(&service, 3)?;
                }
                let result = start(&service);
                if start_mode == 4 {
                    set_mode(&service, 4)?;
                }
                result?;
            } else if !desired && running {
                stop(&service)?;
            }
        }
        _ => return Err(ExecError::Protected),
    }
    Ok(serde_json::json!(true))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn current_session_identity_and_missing_service_are_read_only()
    -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            service_handle("AmetistaTestMissingService_8675309", false).err(),
            Some(ExecError::NotFound)
        );
        assert!(service_handle("invalid;name", false).is_err());
        Ok(())
    }
}
