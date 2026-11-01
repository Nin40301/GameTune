use crate::*;
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
};
pub fn bounded_read(path: &Path, limit: u64) -> Result<Vec<u8>> {
    let file = File::open(path).map_err(|_| Error::Storage)?;
    let mut bytes = vec![];
    file.take(limit + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| Error::Storage)?;
    if bytes.len() as u64 > limit {
        return Err(Error::Storage);
    }
    Ok(bytes)
}
pub fn save<T: Serialize>(path: &Path, data: &T) -> Result<()> {
    let parent = path.parent().ok_or(Error::Storage)?;
    fs::create_dir_all(parent).map_err(|_| Error::Storage)?;
    let mut file = tempfile::NamedTempFile::new_in(parent).map_err(|_| Error::Storage)?;
    serde_json::to_writer(&mut file, data).map_err(|_| Error::Storage)?;
    file.flush().map_err(|_| Error::Storage)?;
    file.as_file().sync_all().map_err(|_| Error::Storage)?;
    file.persist(path).map_err(|_| Error::Storage)?;
    Ok(())
}
pub fn load(directory: &Path) -> Result<Library> {
    let path = directory.join("games.json");
    if !path.exists() {
        return Ok(Library::default());
    }
    let library: Library =
        serde_json::from_slice(&bounded_read(&path, 256 * 1024)?).map_err(|_| Error::Invalid)?;
    library.validate()?;
    Ok(library)
}
pub fn load_for_monitor(directory: &Path) -> Result<Library> {
    match load(directory) {
        Ok(library) => Ok(library),
        Err(_) if debt(directory) => Ok(Library::default()),
        Err(e) => Err(e),
    }
}
pub fn lock(directory: &Path) -> Result<File> {
    fs::create_dir_all(directory).map_err(|_| Error::Storage)?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(directory.join("monitor.lock"))
        .map_err(|_| Error::Storage)?;
    file.try_lock().map_err(|_| Error::Busy)?;
    Ok(file)
}
pub fn save_library(directory: &Path, library: &Library) -> Result<()> {
    let _lock = lock(directory)?;
    library.validate()?;
    save(&directory.join("games.json"), library)
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MonitorStatus {
    pub running: bool,
    pub pid: u32,
    #[serde(default)]
    pub instance: String,
    #[serde(default)]
    pub helper_pid: Option<u32>,
    pub active_games: Vec<String>,
    pub recovery_pending: bool,
    pub error: Option<String>,
    pub readonly: bool,
}
pub fn status(directory: &Path) -> MonitorStatus {
    let mut status: MonitorStatus = bounded_read(&directory.join("monitor-status.json"), 16384)
        .ok()
        .and_then(|b| serde_json::from_slice(&b).ok())
        .unwrap_or_default();
    if let Ok(_lock) = lock(directory) {
        status.running = false;
    }
    status.recovery_pending = debt(directory);
    status
}
pub fn debt(directory: &Path) -> bool {
    let path = directory.join("recovery-pending.json");
    path.exists()
        && bounded_read(&path, 1024)
            .ok()
            .and_then(|b| serde_json::from_slice::<bool>(&b).ok())
            != Some(false)
}
