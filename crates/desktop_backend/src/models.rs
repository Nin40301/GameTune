use gametune_core::*;
use gametune_ipc::AppConsent;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bootstrap {
    pub items: Vec<Item>,
    pub presets: BTreeMap<String, Vec<String>>,
    pub compatible: BTreeMap<String, bool>,
    pub blocked_ids: Vec<String>,
    pub history: History,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct History {
    pub available: bool,
    pub journal: Option<JournalReadView>,
    pub apps: Vec<RemovedApp>,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Analysis {
    pub id: u64,
    pub status: String,
    pub completed: usize,
    pub total: usize,
    pub item_id: String,
    pub observations: Vec<Observation>,
    pub error: Option<String>,
}
impl Default for Analysis {
    fn default() -> Self {
        Self {
            id: 0,
            status: "idle".into(),
            completed: 0,
            total: 0,
            item_id: String::new(),
            observations: vec![],
            error: None,
        }
    }
}
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewKind {
    Apply,
    RemoveApps,
    Undo,
    UndoAll,
    History,
    Reinstall,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Selection {
    pub kind: ReviewKind,
    pub ids: Vec<String>,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Review {
    pub ticket: String,
    pub kind: ReviewKind,
    pub ids: Vec<String>,
    pub counts: BTreeMap<String, usize>,
    pub items: Vec<Item>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Approval {
    pub ticket: String,
    pub confirm: bool,
    pub high_risk_ids: BTreeSet<String>,
    pub unverified_ids: BTreeSet<String>,
    pub apps: Vec<AppConsent>,
}
