//! The only FFI boundary: global kernel snapshots, no OpenProcess/ReadProcessMemory.
use super::{Error, Process, Raw};
use std::{
    mem::{size_of, zeroed},
    ptr,
};
use windows_sys::{
    Wdk::System::SystemInformation::NtQuerySystemInformation,
    Win32::{
        Foundation::{FILETIME, LocalFree},
        System::{
            Power::PowerGetActiveScheme,
            SystemInformation::{GetTickCount64, GlobalMemoryStatusEx, MEMORYSTATUSEX},
            Threading::GetSystemTimes,
            WindowsProgramming::SYSTEM_PROCESS_INFORMATION,
        },
    },
};
fn query(class: i32) -> Result<Vec<u64>, Error> {
    let mut length = if class == 3 { 48 } else { 1024 * 1024 };
    for _ in 0..8 {
        if length > 32 * 1024 * 1024 {
            return Err(Error::Layout);
        }
        let mut data = vec![0u64; (length as usize).div_ceil(8)];
        let mut needed = 0;
        // SAFETY: u64 buffer is aligned for NT structures, writable for length bytes;
        // the kernel returns only into that buffer and reports the needed size.
        let status = unsafe {
            NtQuerySystemInformation(class, data.as_mut_ptr().cast(), length, &mut needed)
        };
        if status == 0 {
            if needed == 0 || needed > length {
                return Err(Error::Layout);
            }
            data.truncate((needed as usize).div_ceil(8));
            return Ok(data);
        }
        if status != 0xc0000004u32 as i32 {
            return Err(Error::Unavailable);
        }
        length = needed.max(length * 2);
    }
    Err(Error::Unavailable)
}
fn ticks(t: FILETIME) -> u64 {
    ((t.dwHighDateTime as u64) << 32) | t.dwLowDateTime as u64
}
fn field(bytes: &[u8], offset: usize) -> Result<u64, Error> {
    let array = bytes
        .get(offset..offset + 8)
        .ok_or(Error::Layout)?
        .try_into()
        .map_err(|_| Error::Layout)?;
    Ok(u64::from_le_bytes(array))
}
pub fn read() -> Result<Raw, Error> {
    // SAFETY: these Win32 output structures accept zero initialization; lengths are set.
    let mut memory: MEMORYSTATUSEX = unsafe { zeroed() };
    memory.dwLength = size_of::<MEMORYSTATUSEX>() as u32;
    let mut idle = FILETIME::default();
    let mut kernel = FILETIME::default();
    let mut user = FILETIME::default();
    // SAFETY: all pointers reference writable outputs. These APIs only read global counters.
    if unsafe { GlobalMemoryStatusEx(&mut memory) } == 0
        || unsafe { GetSystemTimes(&mut idle, &mut kernel, &mut user) } == 0
    {
        return Err(Error::Unavailable);
    }
    let data = query(5)?; // SystemProcessInformation, supported on Windows 10/11.
    let base = data.as_ptr() as usize;
    let end = base + data.len() * 8;
    let mut offset = 0usize;
    let mut processes = Vec::new();
    loop {
        if offset + size_of::<SYSTEM_PROCESS_INFORMATION>() > data.len() * 8
            || !offset.is_multiple_of(8)
        {
            return Err(Error::Layout);
        }
        // SAFETY: aligned bounds-checked NT record. read copies it; no references into
        // a potentially changing foreign object escape the owned snapshot buffer.
        let entry = unsafe { ptr::read((base + offset) as *const SYSTEM_PROCESS_INFORMATION) };
        let name_ptr = entry.ImageName.Buffer as usize;
        let name_len = entry.ImageName.Length as usize;
        let name = if name_len == 0 {
            "System".to_string()
        } else {
            if !name_len.is_multiple_of(2)
                || !name_ptr.is_multiple_of(2)
                || name_ptr < base
                || name_ptr.checked_add(name_len).is_none_or(|p| p > end)
            {
                return Err(Error::Layout);
            }
            // SAFETY: UTF-16 span is wholly inside the owned kernel output buffer.
            String::from_utf16_lossy(unsafe {
                std::slice::from_raw_parts(entry.ImageName.Buffer, name_len / 2)
            })
        };
        // SDK reserves these fields; the Win10/11 NT layout is documented in our
        // architecture notes. Reject unexpected sizes and fail closed, never fall
        // back to opening process handles. 8=WorkingSetPrivateSize, 32=CreateTime.
        processes.push(Process {
            pid: entry.UniqueProcessId as usize as u32,
            parent: entry.Reserved2 as usize as u32,
            created: field(&entry.Reserved1, 24)?,
            name,
            working_set: entry.WorkingSetSize as u64,
            private_working_set: field(&entry.Reserved1, 0)?,
            cpu_ticks: field(&entry.Reserved1, 32)?.saturating_add(field(&entry.Reserved1, 40)?),
        });
        if entry.NextEntryOffset == 0 {
            break;
        }
        let next = entry.NextEntryOffset as usize;
        if next < size_of::<SYSTEM_PROCESS_INFORMATION>() {
            return Err(Error::Layout);
        }
        offset = offset.checked_add(next).ok_or(Error::Layout)?;
    }
    let boot = query(3)?; // SystemTimeOfDayInformation.BootTime, stable across app restarts.
    let mut guid = ptr::null_mut();
    // SAFETY: API allocates a GUID with LocalAlloc; copy before releasing exactly once.
    if unsafe { PowerGetActiveScheme(ptr::null_mut(), &mut guid) } != 0 || guid.is_null() {
        return Err(Error::Unavailable);
    }
    let value = unsafe { *guid };
    unsafe {
        LocalFree(guid.cast());
    }
    let power_plan = format!(
        "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        value.data1,
        value.data2,
        value.data3,
        value.data4[0],
        value.data4[1],
        value.data4[2],
        value.data4[3],
        value.data4[4],
        value.data4[5],
        value.data4[6],
        value.data4[7]
    );
    Ok(Raw {
        // SAFETY: global monotonic uptime, no pointers or process access.
        uptime_ms: unsafe { GetTickCount64() },
        total: memory.ullTotalPhys,
        available: memory.ullAvailPhys,
        idle: ticks(idle),
        total_ticks: ticks(kernel).saturating_add(ticks(user)),
        processes,
        boot: format!("{:016x}", boot[0]),
        power_plan,
    })
}
