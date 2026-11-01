//! Local configuration discovery; never launches a game or scans executable trees.
use crate::*;
use serde_json::Value;
use std::path::{Path, PathBuf};
fn local_path(path: &Path) -> Result<()> {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        let text = path.to_str().ok_or(Error::Invalid)?.replace('/', "\\");
        let bytes = text.as_bytes();
        if bytes.len() < 3
            || !bytes[0].is_ascii_alphabetic()
            || bytes.get(1..3) != Some(b":\\")
            || text[2..].contains(':')
            || text.split('\\').any(|p| p == "..")
        {
            return Err(Error::Invalid);
        }
        if !crate::fs_native::local_drive(bytes[0]) {
            return Err(Error::Invalid);
        }
        // Reject junctions/symlinks before following descendants, including links
        // to network shares. Configuration cannot cause remote file access.
        for ancestor in path.ancestors().collect::<Vec<_>>().into_iter().rev() {
            let metadata = std::fs::symlink_metadata(ancestor).map_err(|_| Error::Storage)?;
            if metadata.file_attributes() & 0x400 != 0 {
                return Err(Error::Invalid);
            }
        }
    }
    let _ = path;
    Ok(())
}
fn local_bytes(path: &Path) -> Result<Vec<u8>> {
    local_path(path)?;
    storage::bounded_read(path, 1024 * 1024)
}
pub fn local_executable(path: &Path) -> Result<PathBuf> {
    local_path(path)?;
    if !path.is_file() {
        return Err(Error::Invalid);
    }
    let canonical = path.canonicalize().map_err(|_| Error::Invalid)?;
    normalize_exe(canonical.to_str().ok_or(Error::Invalid)?)?;
    Ok(canonical)
}
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub launcher: String,
    pub name: String,
    pub install_dir: String,
    pub exe: Option<String>,
}
fn string<'a>(v: &'a Value, key: &str) -> Option<&'a str> {
    v.get(key)?
        .as_str()
        .filter(|s| !s.is_empty() && s.len() <= 2048)
}
pub fn json_candidates(launcher: &str, bytes: &[u8]) -> Result<Vec<Candidate>> {
    if bytes.len() > 1024 * 1024 {
        return Err(Error::Invalid);
    }
    let v: Value = serde_json::from_slice(bytes).map_err(|_| Error::Invalid)?;
    let mut output = vec![];
    match launcher {
        "epic" => {
            let directory = string(&v, "InstallLocation").ok_or(Error::Invalid)?;
            let name = string(&v, "DisplayName").ok_or(Error::Invalid)?;
            let relative = string(&v, "LaunchExecutable")
                .unwrap_or_default()
                .replace('/', "\\");
            let path = format!("{}\\{}", directory.trim_end_matches(['\\', '/']), relative);
            output.push(Candidate {
                launcher: launcher.into(),
                name: name.into(),
                install_dir: directory.into(),
                exe: normalize_exe(&path).ok().map(|_| path),
            });
        }
        "riot" => {
            if let Some(map) = v.get("associated_client").and_then(Value::as_object) {
                for path in map.keys().take(128) {
                    if normalize_exe(path).is_ok() {
                        output.push(Candidate {
                            launcher: launcher.into(),
                            name: path
                                .replace('/', "\\")
                                .rsplit('\\')
                                .next()
                                .unwrap_or("Riot")
                                .into(),
                            install_dir: path.rsplit_once(['\\', '/']).map_or("", |p| p.0).into(),
                            exe: Some(path.clone()),
                        });
                    }
                }
            }
        }
        "minecraft" => {
            if let Some(map) = v.get("profiles").and_then(Value::as_object) {
                for item in map.values().take(128) {
                    output.push(Candidate {
                        launcher: launcher.into(),
                        name: string(item, "name").unwrap_or("Minecraft").into(),
                        install_dir: string(item, "gameDir").unwrap_or_default().into(),
                        exe: string(item, "javaDir")
                            .filter(|p| normalize_exe(p).is_ok())
                            .map(str::to_string),
                    });
                }
            }
        }
        "battlenet" => {
            if let Some(map) = v.get("Games").and_then(Value::as_object) {
                for (name, item) in map.iter().take(128) {
                    if let Some(path) = string(item, "InstallPath") {
                        output.push(Candidate {
                            launcher: launcher.into(),
                            name: name.clone(),
                            install_dir: path.into(),
                            exe: None,
                        });
                    }
                }
            }
        }
        _ => return Err(Error::Invalid),
    }
    if output.iter().any(|c| c.name.len() > 160) {
        return Err(Error::Invalid);
    }
    Ok(output)
}
/// Bounded KeyValues lexer. We read quoted scalar metadata only; scripts are data.
pub fn vdf_pairs(text: &str) -> Result<Vec<(String, String)>> {
    if text.len() > 1024 * 1024 {
        return Err(Error::Invalid);
    }
    let mut tokens = vec![];
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c != '"' {
            continue;
        }
        let mut token = String::new();
        let mut closed = false;
        while let Some(c) = chars.next() {
            match c {
                '"' => {
                    closed = true;
                    break;
                }
                '\\' => {
                    let next = chars.next().ok_or(Error::Invalid)?;
                    token.push(next);
                }
                _ => token.push(c),
            }
            if token.len() > 4096 {
                return Err(Error::Invalid);
            }
        }
        if !closed || tokens.len() > 8192 {
            return Err(Error::Invalid);
        }
        tokens.push(token);
    }
    Ok(tokens
        .windows(2)
        .map(|p| (p[0].clone(), p[1].clone()))
        .collect())
}
pub fn steam_manifest(text: &str, steamapps: &Path) -> Result<Candidate> {
    let fields = vdf_pairs(text)?;
    let find = |key: &str| {
        fields
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.clone())
            .ok_or(Error::Invalid)
    };
    let name = find("name")?;
    let dir = find("installdir")?;
    if dir.contains(['\\', '/', ':']) || dir == ".." || dir.is_empty() {
        return Err(Error::Invalid);
    }
    Ok(Candidate {
        launcher: "steam".into(),
        name,
        install_dir: steamapps.join("common").join(dir).to_string_lossy().into(),
        exe: None,
    })
}
pub struct Locations {
    pub steam: Vec<PathBuf>,
    pub program_data: PathBuf,
    pub roaming: PathBuf,
}
pub fn discover(locations: &Locations) -> Vec<Candidate> {
    let mut output = vec![];
    for root in &locations.steam {
        let folders = root.join("steamapps/libraryfolders.vdf");
        let mut libraries = vec![root.clone()];
        if let Ok(bytes) = local_bytes(&folders)
            && let Ok(pairs) = vdf_pairs(&String::from_utf8_lossy(&bytes))
        {
            libraries.extend(
                pairs
                    .into_iter()
                    .filter(|(k, _)| k == "path")
                    .take(32)
                    .map(|(_, p)| PathBuf::from(p)),
            );
        }
        libraries.sort();
        libraries.dedup();
        for library in libraries {
            let apps = library.join("steamapps");
            if local_path(&apps).is_ok()
                && let Ok(entries) = std::fs::read_dir(&apps)
            {
                for entry in entries.take(512).flatten() {
                    let path = entry.path();
                    let name = entry.file_name().to_string_lossy().into_owned();
                    if name.starts_with("appmanifest_")
                        && name.ends_with(".acf")
                        && let Ok(bytes) = local_bytes(&path)
                        && let Ok(candidate) =
                            steam_manifest(&String::from_utf8_lossy(&bytes), &apps)
                    {
                        output.push(candidate);
                    }
                }
            }
        }
    }
    let epic = locations
        .program_data
        .join("Epic/EpicGamesLauncher/Data/Manifests");
    if local_path(&epic).is_ok()
        && let Ok(entries) = std::fs::read_dir(epic)
    {
        for entry in entries.take(512).flatten() {
            if entry.path().extension().is_some_and(|e| e == "item")
                && let Ok(bytes) = local_bytes(&entry.path())
                && let Ok(c) = json_candidates("epic", &bytes)
            {
                output.extend(c);
            }
        }
    }
    for (launcher, path) in [
        (
            "riot",
            locations
                .program_data
                .join("Riot Games/RiotClientInstalls.json"),
        ),
        (
            "battlenet",
            locations.roaming.join("Battle.net/Battle.net.config"),
        ),
        (
            "minecraft",
            locations.roaming.join(".minecraft/launcher_profiles.json"),
        ),
        (
            "minecraft",
            locations
                .roaming
                .join(".minecraft/launcher_profiles_microsoft_store.json"),
        ),
    ] {
        if let Ok(bytes) = local_bytes(&path)
            && let Ok(c) = json_candidates(launcher, &bytes)
        {
            output.extend(c);
        }
    }
    output.truncate(256);
    output
}
