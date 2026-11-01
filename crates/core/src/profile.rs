use crate::{Catalog, Error, Machine, Result, Revert, Risk};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy)]
pub enum Preset {
    Safe,
    Balanced,
    Aggressive,
}

impl Preset {
    pub fn select(self, catalog: &Catalog, machine: Machine) -> Vec<String> {
        let level = match self {
            Self::Safe => 1,
            Self::Balanced => 2,
            Self::Aggressive => 3,
        };
        catalog
            .items()
            .filter(|i| {
                i.automatic
                    && !crate::protection::is_protected(i)
                    && i.verified
                    && i.level > 0
                    && i.level <= level
                    && i.risk != Risk::High
                    && i.revert == Revert::Auto
                    && machine.supports(i)
                    && (!i.requires_ssd || machine.has_ssd())
            })
            .map(|i| i.id.clone())
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Profile {
    version: u8,
    pub name: String,
    item_ids: Vec<String>,
}

impl Profile {
    pub fn import(json: &str, catalog: &Catalog) -> Result<Self> {
        if json.len() > 64 * 1024 {
            return Err(Error::InvalidProfile("size limit exceeded".into()));
        }
        let profile: Self = serde_json::from_str(json)?;
        if profile.version != 1
            || profile.name.trim().is_empty()
            || profile.name.len() > 120
            || profile.item_ids.len() > 512
        {
            return Err(Error::InvalidProfile(
                "invalid version, name or item count".into(),
            ));
        }
        let mut seen = BTreeSet::new();
        for id in &profile.item_ids {
            catalog.get(id)?;
            if !seen.insert(id) {
                return Err(Error::InvalidProfile("duplicate id".into()));
            }
        }
        Ok(profile)
    }
    pub fn item_ids(&self) -> &[String] {
        &self.item_ids
    }
    pub fn export(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}
