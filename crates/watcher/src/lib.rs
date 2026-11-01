#![deny(unsafe_code)]
pub mod discovery;
#[cfg(windows)]
#[allow(unsafe_code)]
mod fs_native;
pub mod storage;
#[cfg(windows)]
#[allow(unsafe_code)]
pub mod wmi;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("games_invalid")]
    Invalid,
    #[error("games_storage")]
    Storage,
    #[error("games_busy")]
    Busy,
}
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Game {
    pub id: String,
    pub name: String,
    pub exe: String,
    pub action_ids: Vec<String>,
    pub consent: bool,
    pub enabled: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Library {
    pub version: u8,
    pub games: Vec<Game>,
    pub language: String,
}
impl Default for Library {
    fn default() -> Self {
        Self {
            version: 1,
            games: vec![],
            language: "pt-BR".into(),
        }
    }
}
/// Pure lexical normalization: no process or filesystem access, no UNC/ADS/wildcards.
pub fn normalize_exe(path: &str) -> Result<String> {
    let path = path
        .strip_prefix(r"\\?\")
        .unwrap_or(path)
        .replace('/', "\\");
    let bytes = path.as_bytes();
    if bytes.len() < 7
        || bytes.len() > 2048
        || !bytes[0].is_ascii_alphabetic()
        || bytes.get(1..3) != Some(b":\\")
        || !path.to_ascii_lowercase().ends_with(".exe")
        || path[2..]
            .chars()
            .any(|c| c.is_control() || "*:<>?\"|".contains(c))
        || path[3..]
            .split('\\')
            .any(|s| s.is_empty() || s == "." || s == ".." || s.ends_with([' ', '.']))
    {
        return Err(Error::Invalid);
    }
    Ok(path.to_lowercase())
}
pub fn game_id(exe: &str) -> Result<String> {
    Ok(format!(
        "{:x}",
        Sha256::digest(normalize_exe(exe)?.as_bytes())
    ))
}
impl Game {
    pub fn manual(exe: String) -> Result<Self> {
        let id = game_id(&exe)?;
        let name = exe
            .replace('/', "\\")
            .rsplit('\\')
            .next()
            .ok_or(Error::Invalid)?
            .to_string();
        Ok(Self {
            id,
            name,
            exe,
            action_ids: vec![],
            consent: false,
            enabled: false,
        })
    }
}
impl Library {
    pub fn validate(&self) -> Result<()> {
        if self.version != 1
            || self.games.len() > 128
            || !["pt-BR", "en"].contains(&self.language.as_str())
        {
            return Err(Error::Invalid);
        }
        let catalog = gametune_catalog::load_embedded().map_err(|_| Error::Invalid)?;
        let actions = gametune_catalog::game_actions(&catalog).map_err(|_| Error::Invalid)?;
        let mut ids = BTreeSet::new();
        for game in &self.games {
            if game.id != game_id(&game.exe)?
                || !ids.insert(&game.id)
                || game.name.trim().is_empty()
                || game.name.len() > 160
                || game.name.chars().any(char::is_control)
                || (game.enabled && !game.consent)
                || game.action_ids.len() > actions.len()
                || game.action_ids.iter().collect::<BTreeSet<_>>().len() != game.action_ids.len()
                || game
                    .action_ids
                    .iter()
                    .any(|id| !actions.iter().any(|a| &a.id == id))
            {
                return Err(Error::Invalid);
            }
        }
        Ok(())
    }
    pub fn allowed_ids(&self) -> Vec<String> {
        self.games
            .iter()
            .filter(|g| g.enabled && g.consent)
            .flat_map(|g| g.action_ids.iter().cloned())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessKey {
    pub pid: u32,
    pub generation: String,
}
#[derive(Clone, Debug)]
pub struct Event {
    pub started: bool,
    pub key: ProcessKey,
    pub path: Option<String>,
}
pub struct Detector {
    library: Library,
    active: BTreeMap<ProcessKey, String>,
    ended: BTreeSet<ProcessKey>,
}
impl Detector {
    pub fn new(library: Library) -> Result<Self> {
        library.validate()?;
        Ok(Self {
            library,
            active: BTreeMap::new(),
            ended: BTreeSet::new(),
        })
    }
    pub fn observe(&mut self, event: Event) -> Result<bool> {
        if self.ended.len() > 2048 || self.active.len() > 128 {
            return Err(Error::Invalid);
        }
        let initial = self.desired();
        if event.started && self.ended.contains(&event.key) {
            return Ok(false);
        }
        if event.started && !event.key.generation.is_empty() {
            let retired: Vec<_> = self
                .active
                .keys()
                .filter(|key| key.pid == event.key.pid && key.generation != event.key.generation)
                .cloned()
                .collect();
            self.ended.extend(retired);
            self.active.retain(|key, _| {
                key.pid != event.key.pid || key.generation == event.key.generation
            });
        }
        if !event.started && self.active.keys().any(|key| key.pid == event.key.pid) {
            if event.key.generation.is_empty() {
                return Err(Error::Invalid);
            }
            let before = self.desired();
            self.active.remove(&event.key);
            self.ended.insert(event.key);
            return Ok(before != self.desired());
        }
        let Some(path) = event.path.as_deref().and_then(|p| normalize_exe(p).ok()) else {
            return Ok(initial != self.desired());
        };
        let Some(game) =
            self.library.games.iter().find(|g| {
                g.enabled && g.consent && normalize_exe(&g.exe).ok().as_ref() == Some(&path)
            })
        else {
            return Ok(initial != self.desired());
        };
        if event.key.pid == 0 || event.key.generation.is_empty() || event.key.generation.len() > 64
        {
            return Err(Error::Invalid);
        }
        if event.started {
            if self.ended.contains(&event.key) {
                return Ok(false);
            }
            self.active
                .retain(|k, _| k.pid != event.key.pid || k == &event.key);
            self.active.insert(event.key, game.id.clone());
        } else {
            self.active.remove(&event.key);
            self.ended.insert(event.key);
            if self.ended.len() > 2048 {
                return Err(Error::Invalid);
            }
        }
        Ok(initial != self.desired())
    }
    pub fn active_games(&self) -> Vec<String> {
        self.active
            .values()
            .cloned()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }
    pub fn desired(&self) -> Vec<String> {
        let active = self.active_games();
        self.library
            .games
            .iter()
            .filter(|g| active.contains(&g.id))
            .flat_map(|g| g.action_ids.iter().cloned())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }
}
