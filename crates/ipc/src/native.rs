//! Audited FFI boundary: pipe ACLs/peer tokens and launching our own helper only.
//! No process enumeration, game handles, command interpreter or free-form launch.
use crate::{Error, Result, valid_nonce};
use std::{
    ffi::c_void,
    os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle},
    path::Path,
    ptr,
};
use tokio::net::windows::named_pipe::{NamedPipeClient, NamedPipeServer, ServerOptions};
use windows_sys::Win32::{
    Foundation::*,
    Security::{Authorization::*, *},
    Storage::FileSystem::*,
    System::{Pipes::*, Threading::*},
    UI::{Shell::*, WindowsAndMessaging::SW_HIDE},
};
fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain([0]).collect()
}
fn check(ok: i32) -> Result<()> {
    if ok == 0 {
        Err(std::io::Error::last_os_error().into())
    } else {
        Ok(())
    }
}
fn valid_sid(s: &str) -> bool {
    s.starts_with("S-1-5-")
        && s.len() < 184
        && s[6..].bytes().all(|b| b.is_ascii_digit() || b == b'-')
}
pub fn pipe_name(nonce: &str) -> Result<String> {
    if !valid_nonce(nonce) {
        return Err(Error::InvalidPayload);
    }
    Ok(format!(r"\\.\pipe\GameTune.v1.{nonce}"))
}
pub fn server(nonce: &str, sid: &str) -> Result<NamedPipeServer> {
    let name = pipe_name(nonce)?;
    if !valid_sid(sid) {
        return Err(Error::Identity);
    }
    // Exclude FILE_CREATE_PIPE_INSTANCE (0x4). Medium-integrity UI can write to
    // a pipe created by the elevated helper. No administrator/group-wide ACE.
    let sddl = wide(&format!("D:P(A;;0x12019b;;;{sid})S:(ML;;NW;;;ME)"));
    let mut descriptor = ptr::null_mut();
    // SAFETY: valid NUL-terminated SDDL; OS allocates descriptor, freed after create.
    unsafe {
        check(ConvertStringSecurityDescriptorToSecurityDescriptorW(
            sddl.as_ptr(),
            1,
            &mut descriptor,
            ptr::null_mut(),
        ))?;
    }
    let mut attributes = SECURITY_ATTRIBUTES {
        nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
        lpSecurityDescriptor: descriptor,
        bInheritHandle: 0,
    };
    let created = unsafe {
        ServerOptions::new()
            .first_pipe_instance(true)
            .max_instances(1)
            .reject_remote_clients(true)
            .in_buffer_size(65536)
            .out_buffer_size(65536)
            .create_with_security_attributes_raw(
                name,
                (&mut attributes as *mut SECURITY_ATTRIBUTES).cast::<c_void>(),
            )
    };
    // SAFETY: descriptor was allocated by the conversion call and is no longer borrowed.
    unsafe {
        LocalFree(descriptor);
    }
    Ok(created?)
}
pub fn client(nonce: &str) -> Result<NamedPipeClient> {
    let name = wide(&pipe_name(nonce)?);
    // Specific rights match the ACL without granting another server instance.
    // Identification prevents the server from using this client's credentials.
    let handle = unsafe {
        CreateFileW(
            name.as_ptr(),
            0x12019b,
            0,
            ptr::null(),
            OPEN_EXISTING,
            FILE_FLAG_OVERLAPPED | SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION,
            ptr::null_mut(),
        )
    };
    if handle == INVALID_HANDLE_VALUE {
        return Err(std::io::Error::last_os_error().into());
    }
    // SAFETY: Tokio takes sole ownership of a fresh valid overlapped pipe handle.
    unsafe { NamedPipeClient::from_raw_handle(handle.cast()).map_err(Error::from) }
}
pub fn server_pid(pipe: &NamedPipeClient) -> Result<u32> {
    let mut pid = 0;
    unsafe {
        check(GetNamedPipeServerProcessId(
            pipe.as_raw_handle().cast(),
            &mut pid,
        ))?;
    }
    Ok(pid)
}
pub fn client_pid(pipe: &NamedPipeServer) -> Result<u32> {
    let mut pid = 0;
    unsafe {
        check(GetNamedPipeClientProcessId(
            pipe.as_raw_handle().cast(),
            &mut pid,
        ))?;
    }
    Ok(pid)
}
pub fn client_sid(pipe: &NamedPipeServer) -> Result<String> {
    // SAFETY: synchronous impersonation on this OS thread, never across await.
    unsafe {
        check(ImpersonateNamedPipeClient(pipe.as_raw_handle().cast()))?;
    }
    struct Revert;
    impl Drop for Revert {
        fn drop(&mut self) {
            if unsafe { RevertToSelf() } == 0 {
                std::process::abort();
            }
        }
    }
    let _revert = Revert;
    let mut token = ptr::null_mut();
    unsafe {
        check(OpenThreadToken(
            GetCurrentThread(),
            TOKEN_QUERY,
            1,
            &mut token,
        ))?;
    }
    let token = unsafe { OwnedHandle::from_raw_handle(token.cast()) };
    token_sid(token)
}
/// Inspect only our own token, used by pipe ACL tests and UI bootstrap checks.
pub fn current_sid() -> Result<String> {
    let mut token = ptr::null_mut();
    // SAFETY: pseudo-handle of this process, minimum TOKEN_QUERY access.
    unsafe {
        check(OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_QUERY,
            &mut token,
        ))?;
    }
    token_sid(unsafe { OwnedHandle::from_raw_handle(token.cast()) })
}
fn token_sid(token: OwnedHandle) -> Result<String> {
    let mut size = 0;
    unsafe {
        GetTokenInformation(
            token.as_raw_handle().cast(),
            TokenUser,
            ptr::null_mut(),
            0,
            &mut size,
        );
    }
    if size == 0 || size > 65536 {
        return Err(Error::Identity);
    }
    // Pointer-aligned storage for TOKEN_USER followed by the variable SID.
    let mut buffer = vec![0usize; (size as usize).div_ceil(std::mem::size_of::<usize>())];
    unsafe {
        check(GetTokenInformation(
            token.as_raw_handle().cast(),
            TokenUser,
            buffer.as_mut_ptr().cast(),
            size,
            &mut size,
        ))?;
    }
    let user = unsafe { &*buffer.as_ptr().cast::<TOKEN_USER>() };
    let mut string = ptr::null_mut();
    unsafe {
        check(ConvertSidToStringSidW(user.User.Sid, &mut string))?;
    }
    let mut length = 0;
    unsafe {
        while *string.add(length) != 0 {
            length += 1;
        }
    }
    let sid = unsafe { String::from_utf16_lossy(std::slice::from_raw_parts(string, length)) };
    unsafe {
        LocalFree(string.cast());
    }
    Ok(sid)
}
/// The returned handle belongs only to the helper launched by this application.
pub struct Helper {
    pub pid: u32,
    _handle: OwnedHandle,
}
pub fn launch_helper(nonce: &str) -> Result<Helper> {
    if !valid_nonce(nonce) {
        return Err(Error::InvalidPayload);
    }
    let own = std::env::current_exe()?.canonicalize()?;
    let parent = own.parent().ok_or(Error::Operation)?;
    let helper = parent.join("gametune-helper.exe").canonicalize()?;
    if helper.parent() != Some(parent)
        || helper.file_name().and_then(|s| s.to_str()) != Some("gametune-helper.exe")
    {
        return Err(Error::Identity);
    }
    launch(&helper, nonce)
}
fn launch(exe: &Path, nonce: &str) -> Result<Helper> {
    let path = exe.to_str().ok_or(Error::Identity)?;
    let path = path.strip_prefix(r"\\?\").unwrap_or(path);
    if path.starts_with(r"\\") {
        return Err(Error::Identity);
    }
    let file = wide(path);
    let verb = wide("runas");
    let args = wide(&format!("--nonce {nonce} --parent {}", std::process::id()));
    let mut info: SHELLEXECUTEINFOW = unsafe { std::mem::zeroed() };
    info.cbSize = std::mem::size_of::<SHELLEXECUTEINFOW>() as u32;
    info.fMask = SEE_MASK_NOCLOSEPROCESS | SEE_MASK_NOASYNC;
    info.lpVerb = verb.as_ptr();
    info.lpFile = file.as_ptr();
    info.lpParameters = args.as_ptr();
    info.nShow = SW_HIDE;
    // SAFETY: pointers remain valid through ShellExecuteExW; fixed helper and typed args.
    if unsafe { ShellExecuteExW(&mut info) } == 0 {
        let error = std::io::Error::last_os_error();
        return Err(if error.raw_os_error() == Some(1223) {
            Error::UacCancelled
        } else {
            Error::Io(error)
        });
    }
    if info.hProcess.is_null() {
        return Err(Error::Operation);
    }
    let handle = unsafe { OwnedHandle::from_raw_handle(info.hProcess.cast()) };
    let pid = unsafe { GetProcessId(info.hProcess) };
    if pid == 0 {
        return Err(Error::Identity);
    }
    Ok(Helper {
        pid,
        _handle: handle,
    })
}
