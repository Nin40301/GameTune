//! Offline diagnostic projection. Export only a closed schema, never raw state/errors.
use crate::{Analysis, Desktop};
use ametista_windows_runtime::{SupportTool, WindowsSession, logging::Entry};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::Mutex,
    time::Instant,
};
type Result<T> = std::result::Result<T, String>;
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FilePreview {
    pub name: String,
    pub bytes: usize,
    pub content: String,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Preview {
    pub token: String,
    pub files: Vec<FilePreview>,
    pub unavailable: Vec<&'static str>,
}
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Action {
    Preview {},
    Save { token: String, confirm: bool },
    Open { tool: SupportTool },
}
struct Pending {
    preview: Preview,
    created: Instant,
}
pub struct Support {
    directory: PathBuf,
    pending: Mutex<Option<Pending>>,
    last_id: Mutex<Option<u64>>,
}
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct LastAnalysis {
    complete: bool,
    observations: Vec<Row>,
}
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Row {
    item_id: String,
    presence: ametista_core::Presence,
}
fn storage<T>(_: T) -> String {
    "diagnostic_storage".into()
}
impl Support {
    pub fn new(directory: PathBuf) -> Self {
        Self {
            directory,
            pending: Mutex::new(None),
            last_id: Mutex::new(None),
        }
    }
    pub fn remember(&self, analysis: &Analysis) -> Result<()> {
        if analysis.status != "complete" {
            return Ok(());
        }
        let mut last = self.last_id.lock().map_err(storage)?;
        if *last == Some(analysis.id) {
            return Ok(());
        }
        let catalog = ametista_catalog::load_embedded().map_err(storage)?;
        let safe = LastAnalysis {
            complete: true,
            observations: analysis
                .observations
                .iter()
                .filter(|row| catalog.get(&row.item_id).is_ok())
                .map(|row| Row {
                    item_id: row.item_id.clone(),
                    presence: row.presence,
                })
                .collect(),
        };
        std::fs::create_dir_all(&self.directory).map_err(storage)?;
        std::fs::write(
            self.directory.join("last-analysis.json"),
            serde_json::to_vec(&safe).map_err(storage)?,
        )
        .map_err(storage)?;
        *last = Some(analysis.id);
        Ok(())
    }
    pub fn preview(&self, desktop: &Desktop) -> Result<Preview> {
        self.remember(&desktop.analysis_state()?)?;
        let session = WindowsSession::connect().map_err(storage)?;
        let mut unavailable = vec![];
        let analysis = read_limited(&self.directory.join("last-analysis.json"), 128 * 1024)
            .and_then(|bytes| serde_json::from_slice::<LastAnalysis>(&bytes).ok())
            .map(|mut value| {
                value
                    .observations
                    .retain(|row| session.catalog().get(&row.item_id).is_ok());
                value
            });
        if analysis.is_none() {
            unavailable.push("analysis");
        }
        let journal = session.history_readonly().ok().map(|view| json!({"needsRecovery":view.needs_recovery,"entries":view.entries.iter().filter(|entry|session.catalog().get(&entry.item_id).is_ok()).map(|entry| json!({"itemId":entry.item_id,"status":entry.status})).collect::<Vec<_>>()}));
        if journal.is_none() {
            unavailable.push("journal");
        }
        let mut logs = vec![];
        let mut accessible = true;
        for index in 0..=4 {
            let name = if index == 0 {
                "events.jsonl".into()
            } else {
                format!("events.{index}.jsonl")
            };
            let path = session.context.logs_directory().join(name);
            match read_limited(&path, 256 * 1024 + 1024) {
                Some(bytes) => {
                    // Projection again, even if an administrator modified a log file.
                    logs.extend(
                        bytes
                            .split(|b| *b == b'\n')
                            .filter_map(|line| serde_json::from_slice::<Entry>(line).ok()),
                    );
                }
                None => accessible = false,
            }
        }
        if !accessible {
            unavailable.push("logs");
        }
        let data = json!({"schemaVersion":1,"product":"Ametista","version":env!("CARGO_PKG_VERSION"),"anonymized":true,"system":session.context.system_info().map_err(storage)?,"analysis":analysis,"journal":journal,"logs":logs,"unavailable":unavailable});
        let content = serde_json::to_string_pretty(&data).map_err(storage)?;
        let preview = Preview {
            token: ametista_ipc::nonce().map_err(storage)?,
            files: vec![FilePreview {
                name: "diagnostic.json".into(),
                bytes: content.len(),
                content,
            }],
            unavailable,
        };
        *self.pending.lock().map_err(storage)? = Some(Pending {
            preview: preview.clone(),
            created: Instant::now(),
        });
        Ok(preview)
    }
    /// Consume a single-use, short-lived preview BEFORE showing the native Save dialog.
    pub fn consume(&self, token: &str, confirm: bool) -> Result<Vec<FilePreview>> {
        let pending = self
            .pending
            .lock()
            .map_err(storage)?
            .take()
            .ok_or("diagnostic_preview_expired")?;
        if !confirm || pending.preview.token != token || pending.created.elapsed().as_secs() > 120 {
            return Err("diagnostic_preview_expired".into());
        }
        Ok(pending.preview.files)
    }
}
fn read_limited(path: &Path, limit: u64) -> Option<Vec<u8>> {
    let mut bytes = vec![];
    std::fs::File::open(path)
        .ok()?
        .take(limit + 1)
        .read_to_end(&mut bytes)
        .ok()?;
    (bytes.len() as u64 <= limit).then_some(bytes)
}
pub fn save_zip(path: &Path, files: Vec<FilePreview>) -> Result<()> {
    if path
        .extension()
        .is_none_or(|ext| !ext.eq_ignore_ascii_case("zip"))
        || files.len() != 1
        || files[0].name != "diagnostic.json"
    {
        return Err("diagnostic_storage".into());
    }
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(storage)?;
    let mut archive = zip::ZipWriter::new(file);
    for file in files {
        archive
            .start_file(
                file.name,
                zip::write::SimpleFileOptions::default()
                    .compression_method(zip::CompressionMethod::Stored),
            )
            .map_err(storage)?;
        archive
            .write_all(file.content.as_bytes())
            .map_err(storage)?;
    }
    archive
        .finish()
        .map_err(storage)?
        .sync_all()
        .map_err(storage)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preview_required_single_use_and_zip_matches_preview()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let state = Support::new(dir.path().into());
        let content = json!({"schemaVersion":1,"anonymized":true}).to_string();
        let files = vec![FilePreview {
            name: "diagnostic.json".into(),
            bytes: content.len(),
            content: content.clone(),
        }];
        assert!(state.consume("unknown", true).is_err());
        *state.pending.lock().map_err(storage)? = Some(Pending {
            preview: Preview {
                token: "one".into(),
                files,
                unavailable: vec![],
            },
            created: Instant::now(),
        });
        assert!(state.consume("other", true).is_err());
        assert!(state.consume("one", true).is_err());
        let files = vec![FilePreview {
            name: "diagnostic.json".into(),
            bytes: content.len(),
            content: content.clone(),
        }];
        save_zip(&dir.path().join("support.zip"), files.clone())?;
        assert!(save_zip(&dir.path().join("support.zip"), files).is_err());
        let mut archive =
            zip::ZipArchive::new(std::fs::File::open(dir.path().join("support.zip"))?)?;
        let mut text = String::new();
        archive
            .by_name("diagnostic.json")?
            .read_to_string(&mut text)?;
        assert_eq!(text, content);
        assert!(
            serde_json::from_str::<LastAnalysis>(
                r#"{"complete":true,"observations":[],"user":"secret"}"#
            )
            .is_err()
        );
        Ok(())
    }
}
