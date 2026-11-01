//! Reject mapped network drives before reading launcher files. No process access.
use windows::{
    Win32::{
        Storage::FileSystem::GetDriveTypeW,
        System::WindowsProgramming::{DRIVE_FIXED, DRIVE_RAMDISK, DRIVE_REMOVABLE},
    },
    core::PCWSTR,
};
pub fn local_drive(letter: u8) -> bool {
    if !letter.is_ascii_alphabetic() {
        return false;
    }
    let root = [letter as u16, ':' as u16, '\\' as u16, 0];
    // SAFETY: fixed valid drive-root string lives through the call; GetDriveType
    // queries the mapping type without reading files from the mapped location.
    matches!(
        unsafe { GetDriveTypeW(PCWSTR(root.as_ptr())) },
        DRIVE_FIXED | DRIVE_REMOVABLE | DRIVE_RAMDISK
    )
}
