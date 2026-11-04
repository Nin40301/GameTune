#![forbid(unsafe_code)]
// Read only the icon resource of our own build artifact. No game is launched.
#[cfg(windows)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use base64::Engine;
    let probe = std::env::current_exe()?;
    let release = probe
        .parent()
        .and_then(|p| p.parent())
        .ok_or("missing release directory")?;
    let own_artifact = release.join("ametista.exe");
    let uri = ametista_windows_runtime::icon::executable_icon(&own_artifact)
        .ok_or("icon resource unavailable")?;
    let bytes = base64::engine::general_purpose::STANDARD.decode(
        uri.strip_prefix("data:image/png;base64,")
            .ok_or("unexpected icon MIME")?,
    )?;
    let reader = png::Decoder::new(std::io::Cursor::new(&bytes)).read_info()?;
    if reader.info().width != 32 || reader.info().height != 32 {
        return Err("unexpected icon dimensions".into());
    }
    let output =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../work/desktop-verification");
    std::fs::create_dir_all(&output)?;
    std::fs::write(output.join("executable-icon.png"), bytes)?;
    println!("Own executable icon: valid 32x32 PNG; no process opened");
    Ok(())
}
#[cfg(not(windows))]
fn main() {}
