//! Read a registered executable's icon resource, never a process or executable code.
//! The caller resolves IDs from the library and rejects network/reparse paths first.
#[cfg(windows)]
#[allow(unsafe_code)]
mod native;

pub fn executable_icon(path: &std::path::Path) -> Option<String> {
    let text = path.to_str()?;
    let local = text.strip_prefix(r"\\?\").unwrap_or(text);
    let bytes = local.as_bytes();
    if !path.is_absolute()
        || bytes.len() < 7
        || !bytes[0].is_ascii_alphabetic()
        || bytes.get(1..3) != Some(b":\\")
        || local[2..].contains(':')
        || !path.extension()?.to_str()?.eq_ignore_ascii_case("exe")
        || text.contains('\0')
    {
        return None;
    }
    #[cfg(windows)]
    return native::extract(path);
    #[cfg(not(windows))]
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn paths_from_the_ui_cannot_be_used_as_icon_resources() {
        for path in [
            "game.exe",
            r"\\server\share\game.exe",
            "/tmp/resource.dll",
            "https://invalid/game.exe",
        ] {
            assert!(executable_icon(std::path::Path::new(path)).is_none());
        }
    }
}
