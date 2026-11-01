use crate::{Error, Result, protection};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    Registry,
    Service,
    ScheduledTask,
    Appx,
    Startup,
    Power,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Risk {
    None,
    Low,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Revert {
    Auto,
    Manual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Hive {
    HKCU,
    HKLM,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Edition {
    Home,
    Pro,
    Enterprise,
    Education,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceMode {
    Disabled,
    Manual,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StartupTarget {
    pub name: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Target {
    Registry {
        hive: Hive,
        path: String,
        name: String,
        value: u32,
    },
    Service {
        name: String,
        mode: ServiceMode,
    },
    ScheduledTask {
        path: String,
        name: String,
    },
    Appx {
        name: String,
    },
    Startup {
        hive: Hive,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        entry: Option<StartupTarget>,
    },
    Power {
        guid: String,
    },
}

impl Target {
    pub fn action_type(&self) -> ActionType {
        match self {
            Self::Registry { .. } => ActionType::Registry,
            Self::Service { .. } => ActionType::Service,
            Self::ScheduledTask { .. } => ActionType::ScheduledTask,
            Self::Appx { .. } => ActionType::Appx,
            Self::Startup { .. } => ActionType::Startup,
            Self::Power { .. } => ActionType::Power,
        }
    }

    /// Logical key. Windows executors must additionally bind HKCU to the interactive SID.
    pub fn key(&self) -> String {
        match self {
            Self::Registry {
                hive, path, name, ..
            } => format!("registry:{hive:?}\\{path}|{name}"),
            Self::Service { name, .. } => format!("service:{name}"),
            Self::ScheduledTask { path, name } => format!("scheduled_task:{path}{name}"),
            Self::Appx { name } => format!("appx:{name}"),
            Self::Startup { hive, entry } => format!(
                "startup:{hive:?}:{}",
                entry
                    .as_ref()
                    .map_or("discovery", |e| e.fingerprint.as_str())
            ),
            Self::Power { .. } => "power:active".into(),
        }
        .to_ascii_lowercase()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Localized {
    #[serde(rename = "pt-BR")]
    pub pt_br: String,
    pub en: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Item {
    #[serde(default = "default_automatic")]
    pub automatic: bool,
    pub id: String,
    #[serde(rename = "type")]
    pub action_type: ActionType,
    pub category: String,
    pub level: u8,
    pub risk: Risk,
    pub risk_note: Localized,
    pub name: Localized,
    pub description: Localized,
    pub min_build: u32,
    pub max_build: Option<u32>,
    pub target: Target,
    pub revert: Revert,
    pub requires_reboot: bool,
    pub verified: bool,
    pub verification_note: String,
    pub sources: Vec<String>,
    pub supported_editions: Vec<Edition>,
    pub requires_ssd: bool,
}

fn default_automatic() -> bool {
    true
}

#[derive(Debug, Clone)]
pub struct Catalog {
    items: BTreeMap<String, Item>,
}

impl Catalog {
    /// Schema validation belongs to the loader; safety is also enforced here.
    pub fn try_new(items: Vec<Item>) -> Result<Self> {
        let mut by_id = BTreeMap::new();
        let mut keys = BTreeSet::new();
        for item in items {
            protection::validate(&item)?;
            if !keys.insert(item.target.key()) || by_id.contains_key(&item.id) {
                return Err(Error::InvalidCatalog(format!(
                    "duplicate id or target: {}",
                    item.id
                )));
            }
            by_id.insert(item.id.clone(), item);
        }
        if by_id.is_empty() {
            return Err(Error::InvalidCatalog("empty catalog".into()));
        }
        Ok(Self { items: by_id })
    }

    pub fn get(&self, id: &str) -> Result<&Item> {
        self.items
            .get(id)
            .ok_or_else(|| Error::UnknownItem(id.into()))
    }

    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.items.values()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiskKind {
    Unknown,
    Hdd,
    Ssd,
    Nvme,
}

#[derive(Debug, Clone, Copy)]
pub struct Machine {
    pub build: u32,
    pub edition: Edition,
    pub disk: DiskKind,
}

impl Machine {
    pub fn supports(&self, item: &Item) -> bool {
        self.build >= item.min_build
            && item.max_build.is_none_or(|max| self.build <= max)
            && item.supported_editions.contains(&self.edition)
    }
    pub fn has_ssd(&self) -> bool {
        matches!(self.disk, DiskKind::Ssd | DiskKind::Nvme)
    }
}
