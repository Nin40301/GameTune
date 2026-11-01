use crate::*;
use ametista_core::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const MAX_FRAME: usize = 65536;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AppConsent {
    pub item_id: String,
    pub no_undo: bool,
    pub gaming_risk: bool,
    pub unverified: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Command {
    TemporarySession {
        ids: Vec<String>,
        confirm: bool,
    },
    Apply {
        ids: Vec<String>,
        confirm: bool,
        high_risk_ids: BTreeSet<String>,
        unverified_ids: BTreeSet<String>,
    },
    Undo {
        ids: Vec<String>,
        confirm: bool,
    },
    UndoAll {
        confirm: bool,
    },
    RemoveApps {
        apps: Vec<AppConsent>,
        confirm: bool,
    },
    History,
    Reinstall {
        item_id: String,
        confirm: bool,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Envelope {
    pub version: u8,
    pub catalog_hash: String,
    pub nonce: String,
    pub sequence: u8,
    pub command: Command,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Reply {
    pub ok: bool,
    pub result: serde_json::Value,
    pub error: Option<String>,
}
impl Command {
    pub fn ids(&self) -> Vec<String> {
        match self {
            Self::Apply { ids, .. }
            | Self::Undo { ids, .. }
            | Self::TemporarySession { ids, .. } => ids.clone(),
            Self::RemoveApps { apps, .. } => apps.iter().map(|a| a.item_id.clone()).collect(),
            Self::Reinstall { item_id, .. } => vec![item_id.clone()],
            _ => vec![],
        }
    }
    /// Transport structure only. Domain authorization remains in the shared engine.
    pub fn validate_shape(&self, catalog: &Catalog) -> crate::Result<()> {
        let ids = self.ids();
        let unique: BTreeSet<_> = ids.iter().collect();
        if ids.len() > 512
            || unique.len() != ids.len()
            || ids
                .iter()
                .any(|id| id.len() > 160 || catalog.get(id).is_err())
        {
            return Err(crate::Error::InvalidItem);
        }
        if !matches!(
            self,
            Self::UndoAll { .. } | Self::History | Self::TemporarySession { .. }
        ) && ids.is_empty()
        {
            return Err(crate::Error::InvalidPayload);
        }
        match self {
            Self::TemporarySession { confirm, .. } => {
                if !confirm {
                    return Err(crate::Error::Confirmation);
                }
                let actions = ametista_catalog::game_actions(catalog)
                    .map_err(|_| crate::Error::InvalidItem)?;
                if ids.iter().any(|id| !actions.iter().any(|a| &a.id == id)) {
                    return Err(crate::Error::InvalidItem);
                }
            }
            Self::Apply {
                confirm,
                high_risk_ids,
                unverified_ids,
                ..
            } => {
                if !confirm {
                    return Err(crate::Error::Confirmation);
                }
                if high_risk_ids
                    .iter()
                    .chain(unverified_ids)
                    .any(|id| !ids.contains(id))
                {
                    return Err(crate::Error::InvalidItem);
                }
            }
            Self::Undo { confirm, .. }
            | Self::UndoAll { confirm }
            | Self::Reinstall { confirm, .. } => {
                if !confirm {
                    return Err(crate::Error::Confirmation);
                }
            }
            Self::RemoveApps { apps, confirm } => {
                if !confirm || apps.iter().any(|a| !a.no_undo) {
                    return Err(crate::Error::Confirmation);
                }
                for app in apps {
                    let item = catalog
                        .get(&app.item_id)
                        .map_err(|_| crate::Error::InvalidItem)?;
                    if item.action_type != ActionType::Appx {
                        return Err(crate::Error::InvalidItem);
                    }
                    if item.risk == Risk::High && !app.gaming_risk
                        || !item.verified && !app.unverified
                    {
                        return Err(crate::Error::Confirmation);
                    }
                }
            }
            Self::History => {}
        }
        Ok(())
    }
}
/// Peer facts come from the OS, never from the envelope or WebView.
pub struct Gate {
    sid: String,
    pid: u32,
    nonce: String,
    used: bool,
}
impl Gate {
    pub fn new(sid: String, pid: u32, nonce: String) -> crate::Result<Self> {
        if !valid_nonce(&nonce) || pid == 0 {
            return Err(crate::Error::InvalidPayload);
        }
        Ok(Self {
            sid,
            pid,
            nonce,
            used: false,
        })
    }
    pub fn accept(
        &mut self,
        bytes: &[u8],
        peer_sid: &str,
        peer_pid: u32,
        catalog: &Catalog,
    ) -> crate::Result<Command> {
        if self.used {
            return Err(crate::Error::Replay);
        }
        self.used = true; // Even malformed/unauthorized attempts consume this session.
        if peer_sid != self.sid || peer_pid != self.pid {
            return Err(crate::Error::Identity);
        }
        if bytes.len() > MAX_FRAME {
            return Err(crate::Error::InvalidPayload);
        }
        let envelope: Envelope =
            serde_json::from_slice(bytes).map_err(|_| crate::Error::InvalidPayload)?;
        if envelope.version != 1 || envelope.sequence != 1 || envelope.nonce != self.nonce {
            return Err(crate::Error::Replay);
        }
        if envelope.catalog_hash != ametista_catalog::embedded_hash() {
            return Err(crate::Error::CatalogMismatch);
        }
        envelope.command.validate_shape(catalog)?;
        Ok(envelope.command)
    }
}
