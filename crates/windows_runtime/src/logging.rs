//! Small, typed events only. No error text, registry values, paths, SIDs or user names.
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Write,
    path::Path,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Event {
    Apply,
    Undo,
    RemoveApp,
    GameSession,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Entry {
    pub at: u64,
    pub event: Event,
    pub succeeded: bool,
}
const SIZE: u64 = 256 * 1024;
const AGE: Duration = Duration::from_secs(14 * 24 * 60 * 60);
/// Caller supplies an already protected directory; never creates a ProgramData root.
pub(crate) fn append(directory: &Path, entry: &Entry) -> std::io::Result<()> {
    let current = directory.join("events.jsonl");
    for index in 0..=4 {
        let file = if index == 0 {
            current.clone()
        } else {
            directory.join(format!("events.{index}.jsonl"))
        };
        match fs::symlink_metadata(&file) {
            Ok(meta) => {
                #[cfg(windows)]
                {
                    use std::os::windows::fs::MetadataExt;
                    if meta.file_attributes() & 0x400 != 0 {
                        return Err(std::io::ErrorKind::PermissionDenied.into());
                    }
                }
                if !meta.is_file() || meta.is_symlink() {
                    return Err(std::io::ErrorKind::PermissionDenied.into());
                }
                if SystemTime::now()
                    .duration_since(meta.modified()?)
                    .unwrap_or_default()
                    > AGE
                {
                    fs::remove_file(file)?;
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
            Err(e) => return Err(e),
        }
    }
    if fs::metadata(&current).is_ok_and(|m| m.len() >= SIZE) {
        let oldest = directory.join("events.4.jsonl");
        if oldest.exists() {
            fs::remove_file(oldest)?;
        }
        for index in (1..4).rev() {
            let source = directory.join(format!("events.{index}.jsonl"));
            if source.exists() {
                fs::rename(
                    source,
                    directory.join(format!("events.{}.jsonl", index + 1)),
                )?;
            }
        }
        fs::rename(&current, directory.join("events.1.jsonl"))?;
    }
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(current)?;
    serde_json::to_writer(&mut file, entry)?;
    file.write_all(b"\n")?;
    file.sync_all()
}
impl crate::ProtectedStore {
    pub fn log(&self, event: Event, succeeded: bool) -> std::io::Result<()> {
        let directory = self
            .directory()
            .parent()
            .ok_or(std::io::ErrorKind::PermissionDenied)?
            .join("logs");
        append(
            &directory,
            &Entry {
                at: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
                event,
                succeeded,
            },
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn size_age_rotation_and_typed_privacy() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        fs::write(dir.path().join("events.jsonl"), vec![b' '; SIZE as usize])?;
        for _ in 0..6 {
            append(
                dir.path(),
                &Entry {
                    at: 0,
                    event: Event::Apply,
                    succeeded: true,
                },
            )?;
            fs::write(dir.path().join("events.jsonl"), vec![b' '; SIZE as usize])?;
        }
        assert_eq!(fs::read_dir(dir.path())?.count(), 5);
        let old = fs::OpenOptions::new()
            .write(true)
            .open(dir.path().join("events.4.jsonl"))?;
        old.set_times(
            fs::FileTimes::new().set_modified(SystemTime::now() - AGE - Duration::from_secs(1)),
        )?;
        append(
            dir.path(),
            &Entry {
                at: 1,
                event: Event::Undo,
                succeeded: false,
            },
        )?;
        assert!(
            serde_json::from_str::<Entry>(
                r#"{"at":1,"event":"undo","succeeded":true,"username":"private"}"#
            )
            .is_err()
        );
        Ok(())
    }
}
