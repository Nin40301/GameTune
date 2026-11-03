//! Only our notification window/icon. No hooks or handles to game processes.
use std::{
    ptr,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use windows_sys::Win32::{
    Foundation::*,
    System::{Com::CoTaskMemFree, LibraryLoader::GetModuleHandleW},
    UI::{HiDpi::GetDpiForWindow, Shell::*, WindowsAndMessaging::*},
};
static STOP: AtomicBool = AtomicBool::new(false);
static ENGLISH: AtomicBool = AtomicBool::new(false);
static TOOLTIP: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
pub fn set_status(text: &str) {
    if let Ok(mut value) = TOOLTIP.lock() {
        *value = text.into();
    }
}
fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain([0]).collect()
}
pub fn data_directory() -> std::io::Result<std::path::PathBuf> {
    let mut path = ptr::null_mut();
    // SAFETY: known-folder allocation is copied and freed once. Never use an
    // environment-controlled executable path or a remote WMI namespace.
    if unsafe { SHGetKnownFolderPath(&FOLDERID_LocalAppData, 0, ptr::null_mut(), &mut path) } < 0 {
        return Err(std::io::Error::last_os_error());
    }
    let mut length = 0;
    unsafe {
        while *path.add(length) != 0 {
            length += 1;
        }
    }
    let value = unsafe { String::from_utf16_lossy(std::slice::from_raw_parts(path, length)) };
    unsafe {
        CoTaskMemFree(path.cast());
    }
    Ok(std::path::PathBuf::from(value)
        .join("io.github.nin40301.ametista")
        .join("games"))
}
pub fn stopped() -> bool {
    STOP.load(Ordering::Relaxed)
}
unsafe extern "system" fn callback(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // SAFETY: Win32 owns dispatch; only scalar menu IDs and our HWND are used.
    unsafe {
        match message {
            m if m == WM_APP + 1
                && [
                    WM_CONTEXTMENU,
                    WM_RBUTTONUP,
                    WM_LBUTTONUP,
                    NIN_SELECT,
                    NIN_SELECT | NINF_KEY,
                ]
                .contains(&((lparam as u32) & 0xffff)) =>
            {
                let menu = CreatePopupMenu();
                let english = ENGLISH.load(Ordering::Relaxed);
                let labels = crate::worker::labels(english);
                AppendMenuW(
                    menu,
                    MF_STRING,
                    1,
                    wide(labels.get("open").map(String::as_str).unwrap_or("Ametista")).as_ptr(),
                );
                AppendMenuW(
                    menu,
                    MF_STRING,
                    2,
                    wide(labels.get("stop").map(String::as_str).unwrap_or("Ametista")).as_ptr(),
                );
                let mut point = POINT::default();
                GetCursorPos(&mut point);
                SetForegroundWindow(window);
                let command = TrackPopupMenu(
                    menu,
                    TPM_RETURNCMD | TPM_NONOTIFY,
                    point.x,
                    point.y,
                    0,
                    window,
                    ptr::null(),
                ) as u32;
                DestroyMenu(menu);
                if command == 1
                    && let Ok(path) = std::env::current_exe()
                    && let Some(parent) = path.parent()
                {
                    let _ = std::process::Command::new(parent.join("ametista.exe")).spawn();
                }
                if command == 2 {
                    STOP.store(true, Ordering::Relaxed);
                }
                0
            }
            WM_DPICHANGED => {
                let dpi = (wparam as u32 & 0xffff).max(96);
                let size = ((16 * dpi + 48) / 96) as i32;
                let image = LoadImageW(
                    GetModuleHandleW(ptr::null()),
                    std::ptr::without_provenance::<u16>(1),
                    IMAGE_ICON,
                    size,
                    size,
                    0,
                ) as HICON;
                if !image.is_null() {
                    let old = SetWindowLongPtrW(window, GWLP_USERDATA, image as isize) as HICON;
                    SendMessageW(window, WM_TIMER, 1, 0);
                    if !old.is_null() {
                        DestroyIcon(old);
                    }
                }
                0
            }
            WM_CLOSE => {
                STOP.store(true, Ordering::Relaxed);
                0
            }
            WM_TIMER => {
                let mut icon: NOTIFYICONDATAW = std::mem::zeroed();
                icon.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
                icon.hWnd = window;
                icon.uID = 1;
                icon.uFlags = NIF_TIP | NIF_SHOWTIP | NIF_MESSAGE | NIF_ICON;
                icon.uCallbackMessage = WM_APP + 1;
                icon.hIcon = GetWindowLongPtrW(window, GWLP_USERDATA) as HICON;
                if let Ok(value) = TOOLTIP.lock() {
                    for (out, c) in icon.szTip.iter_mut().take(127).zip(value.encode_utf16()) {
                        *out = c;
                    }
                }
                if Shell_NotifyIconW(NIM_MODIFY, &icon) == 0
                    && Shell_NotifyIconW(NIM_ADD, &icon) != 0
                {
                    icon.Anonymous.uVersion = NOTIFYICON_VERSION_4;
                    Shell_NotifyIconW(NIM_SETVERSION, &icon);
                }
                0
            }
            _ => DefWindowProcW(window, message, wparam, lparam),
        }
    }
}
pub fn run(done: Arc<AtomicBool>, title: &str, english: bool) -> std::io::Result<()> {
    ENGLISH.store(english, Ordering::Relaxed);
    set_status(title);
    // SAFETY: class, callback, icon, menu and hidden window live on this thread.
    // All allocations/handles created here are released before returning.
    unsafe {
        let instance = GetModuleHandleW(ptr::null());
        let class = wide("AmetistaMonitorWindow");
        let descriptor = WNDCLASSW {
            lpfnWndProc: Some(callback),
            hInstance: instance,
            lpszClassName: class.as_ptr(),
            ..std::mem::zeroed()
        };
        if RegisterClassW(&descriptor) == 0 {
            return Err(std::io::Error::last_os_error());
        }
        let window = CreateWindowExW(
            0,
            class.as_ptr(),
            wide("Ametista Monitor").as_ptr(),
            0,
            0,
            0,
            0,
            0,
            ptr::null_mut(),
            ptr::null_mut(),
            instance,
            ptr::null(),
        );
        if window.is_null() {
            return Err(std::io::Error::last_os_error());
        }
        let mut icon: NOTIFYICONDATAW = std::mem::zeroed();
        icon.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
        icon.hWnd = window;
        icon.uID = 1;
        icon.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP | NIF_SHOWTIP;
        icon.uCallbackMessage = WM_APP + 1;
        // MAKEINTRESOURCE(1) is a resource identifier, never a dereferenceable pointer.
        let dpi = GetDpiForWindow(window).max(96);
        let size = ((16 * dpi + 48) / 96) as i32;
        icon.hIcon = LoadImageW(
            instance,
            std::ptr::without_provenance::<u16>(1),
            IMAGE_ICON,
            size,
            size,
            0,
        ) as HICON;
        if icon.hIcon.is_null() {
            DestroyWindow(window);
            UnregisterClassW(class.as_ptr(), instance);
            return Err(std::io::Error::last_os_error());
        }
        SetWindowLongPtrW(window, GWLP_USERDATA, icon.hIcon as isize);
        for (out, c) in icon.szTip.iter_mut().take(127).zip(title.encode_utf16()) {
            *out = c;
        }
        if Shell_NotifyIconW(NIM_ADD, &icon) == 0 {
            DestroyIcon(icon.hIcon);
            DestroyWindow(window);
            UnregisterClassW(class.as_ptr(), instance);
            return Err(std::io::Error::last_os_error());
        }
        icon.Anonymous.uVersion = NOTIFYICON_VERSION_4;
        Shell_NotifyIconW(NIM_SETVERSION, &icon);
        SetTimer(window, 1, 1000, None);
        let mut message = MSG::default();
        while !done.load(Ordering::Relaxed) && GetMessageW(&mut message, ptr::null_mut(), 0, 0) > 0
        {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
        Shell_NotifyIconW(NIM_DELETE, &icon);
        KillTimer(window, 1);
        DestroyIcon(GetWindowLongPtrW(window, GWLP_USERDATA) as HICON);
        DestroyWindow(window);
        UnregisterClassW(class.as_ptr(), instance);
        Ok(())
    }
}
