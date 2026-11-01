#![deny(unsafe_code)]
#![cfg_attr(windows, windows_subsystem = "windows")]
#[cfg(windows)]
#[allow(unsafe_code)]
mod tray;
#[cfg(windows)]
mod worker;
fn main() -> std::process::ExitCode {
    #[cfg(windows)]
    {
        if worker::run().is_ok() {
            std::process::ExitCode::SUCCESS
        } else {
            std::process::ExitCode::from(7)
        }
    }
    #[cfg(not(windows))]
    {
        std::process::ExitCode::from(7)
    }
}
