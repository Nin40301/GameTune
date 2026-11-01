#![forbid(unsafe_code)]
#![cfg_attr(windows, windows_subsystem = "windows")]
fn main() -> std::process::ExitCode {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 4
        || args[0] != "--nonce"
        || !ametista_ipc::valid_nonce(&args[1])
        || args[2] != "--parent"
    {
        return std::process::ExitCode::from(2);
    }
    let Ok(parent) = args[3].parse::<u32>() else {
        return std::process::ExitCode::from(2);
    };
    if parent == 0 {
        return std::process::ExitCode::from(2);
    }
    #[cfg(windows)]
    {
        let runtime = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(r) => r,
            Err(_) => return std::process::ExitCode::from(7),
        };
        std::process::ExitCode::from(
            if runtime
                .block_on(ametista_ipc::pipe::serve(args[1].clone(), parent))
                .is_ok()
            {
                0
            } else {
                7
            },
        )
    }
    #[cfg(not(windows))]
    std::process::ExitCode::from(7)
}
