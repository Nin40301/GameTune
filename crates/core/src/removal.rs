//! Explicitly irreversible app removal, separate from the reversible engine/journal.
use crate::{ActionType, Engine, Error, ExecError, Item, Result, Risk, Snapshot, Target};
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug, Default)]
pub struct AppRemovalConsent {
    pub item_id: String,
    pub acknowledge_no_undo: bool,
    pub acknowledge_gaming_dependencies: bool,
    pub acknowledge_unverified: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemovalStatus {
    Prepared,
    Removed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemovedApp {
    pub item_id: String,
    pub name: String,
    pub package_family_names: Vec<String>,
    pub original: Snapshot,
    pub attempted_at: String,
    pub status: RemovalStatus,
    pub error: Option<ExecError>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Document {
    version: u8,
    scope: Option<String>,
    entries: Vec<RemovedApp>,
}

/// Lock and replace-in-place on the same volume. Parent must already be protected.
pub struct RemovedApps {
    path: PathBuf,
    document: Document,
    _lock: File,
    poisoned: bool,
}

impl RemovedApps {
    pub fn open(path: impl AsRef<Path>, scope: Option<&str>) -> Result<Self> {
        let path = path.as_ref().to_owned();
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path.with_extension("lock"))?;
        lock.try_lock()
            .map_err(|e| Error::JournalLocked(e.to_string()))?;
        let document = match std::fs::read(&path) {
            Ok(bytes) => serde_json::from_slice::<Document>(&bytes)?,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Document {
                version: 1,
                scope: scope.map(str::to_owned),
                entries: vec![],
            },
            Err(e) => return Err(e.into()),
        };
        if document.version != 1 || document.scope.as_deref() != scope {
            return Err(Error::CorruptJournal(
                "removed-apps scope or version mismatch".into(),
            ));
        }
        Ok(Self {
            path,
            document,
            _lock: lock,
            poisoned: false,
        })
    }
    pub fn entries(&self) -> &[RemovedApp] {
        &self.document.entries
    }
    pub fn scope(&self) -> Option<&str> {
        self.document.scope.as_deref()
    }
    pub fn pending(&self) -> bool {
        self.document
            .entries
            .iter()
            .any(|e| e.status == RemovalStatus::Prepared)
    }

    fn save(&mut self) -> Result<()> {
        if self.poisoned {
            return Err(Error::JournalPoisoned);
        }
        let result = (|| -> Result<()> {
            let parent = self
                .path
                .parent()
                .ok_or_else(|| Error::CorruptJournal("missing parent".into()))?;
            let mut temp = tempfile::NamedTempFile::new_in(parent)?;
            temp.write_all(&serde_json::to_vec_pretty(&self.document)?)?;
            temp.as_file().sync_all()?;
            let file = temp.persist(&self.path).map_err(|e| e.error)?;
            file.sync_all()?;
            #[cfg(unix)]
            File::open(parent)?.sync_all()?;
            Ok(())
        })();
        if result.is_err() {
            self.poisoned = true;
        }
        result
    }
    fn prepare(&mut self, item: &Item, original: Snapshot) -> Result<usize> {
        if self.pending() {
            return Err(Error::RecoveryRequired);
        }
        let Target::Appx { name } = &item.target else {
            return Err(Error::InvalidCatalog("not Appx".into()));
        };
        let Snapshot::Appx { families, .. } = &original else {
            return Err(Error::InvalidCatalog("not Appx state".into()));
        };
        for family in families {
            validate_family(name, family)?;
        }
        if families.is_empty() {
            return Err(Error::InvalidCatalog(
                "Appx inventory has no family name".into(),
            ));
        }
        self.document.entries.push(RemovedApp {
            item_id: item.id.clone(),
            name: name.clone(),
            package_family_names: families.clone(),
            original,
            attempted_at: time::OffsetDateTime::now_utc()
                .format(&time::format_description::well_known::Rfc3339)?,
            status: RemovalStatus::Prepared,
            error: None,
        });
        self.save()?;
        Ok(self.document.entries.len() - 1)
    }
    fn finish(&mut self, index: usize, result: std::result::Result<(), ExecError>) -> Result<()> {
        let entry = self
            .document
            .entries
            .get_mut(index)
            .ok_or_else(|| Error::CorruptJournal("missing app attempt".into()))?;
        entry.status = if result.is_ok() {
            RemovalStatus::Removed
        } else {
            RemovalStatus::Failed
        };
        entry.error = result.err();
        self.save()
    }

    pub fn reinstall_actions(
        &self,
        catalog: &crate::Catalog,
        item_id: &str,
    ) -> Result<Vec<ReinstallAction>> {
        let item = catalog.get(item_id)?;
        let Target::Appx { name } = &item.target else {
            return Err(Error::UnknownItem(item_id.into()));
        };
        let entry = self
            .document
            .entries
            .iter()
            .rev()
            .find(|e| e.item_id == item_id)
            .ok_or_else(|| Error::UnknownItem(item_id.into()))?;
        if entry.name != *name {
            return Err(Error::CorruptJournal("Appx name mismatch".into()));
        }
        entry
            .package_family_names
            .iter()
            .map(|family| {
                validate_family(name, family)?;
                Ok(ReinstallAction {
                    item_id: item_id.into(),
                    family: family.clone(),
                    uri: format!("ms-windows-store://pdp/?PFN={family}"),
                })
            })
            .collect()
    }

    /// Offline report with actual Reinstall buttons (Store protocol links, no JavaScript).
    /// Opening a link is an explicit user action; exporting does not contact the Store.
    pub fn export_html(&self, catalog: &crate::Catalog, english: bool) -> Result<String> {
        let source = if english {
            include_str!("../../../locales/en.json")
        } else {
            include_str!("../../../locales/pt-BR.json")
        };
        let labels: serde_json::Value = serde_json::from_str(source)?;
        let label = |key: &str| -> Result<String> {
            labels[key]
                .as_str()
                .map(escape)
                .ok_or_else(|| Error::InvalidCatalog("missing locale".into()))
        };
        let mut html = format!(
            "<!doctype html><html lang=\"{}\"><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width\"><meta http-equiv=\"Content-Security-Policy\" content=\"default-src 'none'; style-src 'unsafe-inline'; base-uri 'none'; form-action 'none'\"><title>{}</title><style>body{{font:1rem system-ui;background:#111827;color:#f9fafb;max-width:64rem;margin:2rem auto;padding:1rem}}a{{display:inline-block;background:#c7f9cc;color:#102316;padding:.7rem 1rem;border-radius:.4rem}}a:focus-visible{{outline:3px solid #fff;outline-offset:4px}}li{{margin:1rem 0}}p{{line-height:1.6}}</style><h1>{}</h1><p>{}</p><ul>",
            if english { "en" } else { "pt-BR" },
            label("appx.history")?,
            label("appx.history")?,
            label("appx.noUndo")?
        );
        for entry in &self.document.entries {
            let item = catalog.get(&entry.item_id)?;
            let status_key = match entry.status {
                RemovalStatus::Prepared => "appx.status.prepared",
                RemovalStatus::Removed => "appx.status.removed",
                RemovalStatus::Failed => "appx.status.failed",
            };
            html.push_str(&format!(
                "<li><h2>{}</h2><p>{}</p>",
                escape(if english {
                    &item.name.en
                } else {
                    &item.name.pt_br
                }),
                escape(&entry.attempted_at)
            ));
            html.push_str(&format!("<p>{}</p>", label(status_key)?));
            for action in self.reinstall_actions(catalog, &entry.item_id)? {
                html.push_str(&format!(
                    "<p>{}</p><a href=\"{}\">{}</a>",
                    escape(action.family()),
                    escape(action.uri()),
                    label("appx.reinstall")?
                ));
            }
            html.push_str("</li>");
        }
        html.push_str("</ul></html>");
        Ok(html)
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReinstallAction {
    item_id: String,
    family: String,
    uri: String,
}
impl ReinstallAction {
    pub fn item_id(&self) -> &str {
        &self.item_id
    }
    pub fn family(&self) -> &str {
        &self.family
    }
    pub fn uri(&self) -> &str {
        &self.uri
    }
}

fn validate_family(name: &str, family: &str) -> Result<()> {
    let prefix = format!("{name}_");
    if !family
        .get(..prefix.len())
        .is_some_and(|p| p.eq_ignore_ascii_case(&prefix))
        || !name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b".-".contains(&b))
        || family.len() != prefix.len() + 13
        || !family[prefix.len()..]
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
    {
        return Err(Error::CorruptJournal(
            "invalid captured PackageFamilyName".into(),
        ));
    }
    Ok(())
}
fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

impl Engine<'_> {
    pub fn preview_app_removal(
        &self,
        id: &str,
    ) -> Result<std::result::Result<Snapshot, ExecError>> {
        let item = self.catalog.get(id)?;
        if item.action_type != ActionType::Appx {
            return Err(Error::UnknownItem(id.into()));
        }
        Ok(self
            .executors
            .get(ActionType::Appx)
            .and_then(|e| e.capture(item)))
    }

    pub fn remove_app(
        &mut self,
        consent: &AppRemovalConsent,
        store: &mut RemovedApps,
    ) -> Result<std::result::Result<(), ExecError>> {
        let item = self.catalog.get(&consent.item_id)?;
        if item.action_type != ActionType::Appx {
            return Err(Error::UnknownItem(item.id.clone()));
        }
        if !self.machine.supports(item) {
            return Err(Error::Incompatible(item.id.clone()));
        }
        if !consent.acknowledge_no_undo
            || (item.risk == Risk::High && !consent.acknowledge_gaming_dependencies)
            || (!item.verified && !consent.acknowledge_unverified)
        {
            return Err(Error::ConfirmationRequired(item.id.clone()));
        }
        let executor = match self.executors.get(ActionType::Appx) {
            Ok(e) => e,
            Err(e) => return Ok(Err(e)),
        };
        if executor.execution_scope() != store.scope() {
            return Err(Error::CorruptJournal("Appx scope mismatch".into()));
        }
        if let Err(e) = executor.ensure_write_ready() {
            return Ok(Err(e));
        }
        let original = match executor.capture(item) {
            Ok(state) => state,
            Err(e) => return Ok(Err(e)),
        };
        let index = store.prepare(item, original)?; // persistent name/PFN BEFORE uninstall
        let result = self
            .executors
            .get_mut(ActionType::Appx)
            .and_then(|e| e.apply(item));
        store.finish(index, result.clone())?;
        Ok(result)
    }

    pub fn reconcile_app_removal(&self, id: &str, store: &mut RemovedApps) -> Result<()> {
        let item = self.catalog.get(id)?;
        let executor = self
            .executors
            .get(ActionType::Appx)
            .map_err(|e| Error::InvalidCatalog(e.to_string()))?;
        if item.action_type != ActionType::Appx || executor.execution_scope() != store.scope() {
            return Err(Error::CorruptJournal("Appx scope mismatch".into()));
        }
        let index = store
            .document
            .entries
            .iter()
            .rposition(|e| e.item_id == id && e.status == RemovalStatus::Prepared)
            .ok_or_else(|| Error::UnknownItem(id.into()))?;
        let result = match executor.capture(item) {
            Err(ExecError::NotFound) => Ok(()),
            Err(e) => Err(e),
            Ok(_) => Err(ExecError::Failed(
                "removal incomplete; review installed and provisioned packages before retrying"
                    .into(),
            )),
        };
        store.finish(index, result)
    }
}
