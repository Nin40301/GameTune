#![forbid(unsafe_code)]
#[cfg(feature = "real-profile-tests")]
fn main() -> std::process::ExitCode {
    use gametune_windows_runtime::real_profiles::{self, Stage};
    let stage = match std::env::args().nth(1).as_deref() {
        Some("step") => Stage::Step,
        Some("verify") => Stage::Verify,
        Some("recover") => Stage::Recover,
        _ => {
            eprintln!("step | verify | recover");
            return std::process::ExitCode::from(2);
        }
    };
    match real_profiles::run(stage) {
        Ok(result) => {
            println!("{}: {}", result.status, result.evidence);
            std::process::ExitCode::from(if result.status == "reboot_required" {
                10
            } else {
                0
            })
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::ExitCode::from(7)
        }
    }
}
#[cfg(not(feature = "real-profile-tests"))]
fn main() {
    eprintln!("Requires real-profile-tests and explicit VM opt-in");
}
