//! Pure interpretation of read-only observations. Unknown is never treated as absent.
use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Presence {
    Exists,
    Missing,
    AlreadyApplied,
    Unknown,
    Discovery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Observation {
    pub item_id: String,
    pub presence: Presence,
    pub exists: Option<bool>,
    pub key_exists: Option<bool>,
    pub already_applied: Option<bool>,
    pub current: Option<Snapshot>,
    pub current_dword: Option<u32>,
    pub error: Option<ExecError>,
    pub scope: ReadScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadScope {
    Machine,
    InteractiveUser,
    CurrentUserOnly,
    AllUsers,
    Discovery,
}

pub fn observe(item: &Item, result: ExecResult<Snapshot>, scope: ReadScope) -> Observation {
    let mut row = Observation {
        item_id: item.id.clone(),
        presence: Presence::Unknown,
        exists: None,
        key_exists: None,
        already_applied: None,
        current: None,
        current_dword: None,
        error: None,
        scope,
    };
    match result {
        Err(ExecError::NotFound) => {
            row.presence = Presence::Missing;
            row.exists = Some(false);
            row.already_applied = Some(false);
        }
        Err(error) => row.error = Some(error),
        Ok(snapshot) => {
            let (exists, applied) = match (&item.target, &snapshot) {
                (
                    Target::Registry { value: desired, .. },
                    Snapshot::Registry {
                        value,
                        created_keys,
                    },
                ) => {
                    row.key_exists = Some(created_keys.is_empty());
                    row.current_dword = value
                        .as_ref()
                        .filter(|v| v.kind == 4)
                        .and_then(|v| v.bytes.as_slice().try_into().ok())
                        .map(u32::from_le_bytes);
                    (value.is_some(), row.current_dword == Some(*desired))
                }
                (
                    Target::Service { mode, .. },
                    Snapshot::Service {
                        start_mode,
                        running,
                        ..
                    },
                ) => (
                    true,
                    *start_mode == (if *mode == ServiceMode::Manual { 3 } else { 4 }) && !*running,
                ),
                (Target::ScheduledTask { .. }, Snapshot::ScheduledTask { enabled, .. }) => {
                    (true, !*enabled)
                }
                (
                    Target::Power { guid },
                    Snapshot::Power {
                        active_guid,
                        existing_guids,
                    },
                ) => (
                    existing_guids.contains(guid),
                    active_guid.eq_ignore_ascii_case(guid),
                ),
                (
                    Target::Appx { .. },
                    Snapshot::Appx {
                        packages,
                        provisioned,
                        ..
                    },
                ) => (!packages.is_empty() || !provisioned.is_empty(), false),
                (
                    Target::Startup { entry: Some(_), .. },
                    Snapshot::Startup {
                        approved,
                        created_keys,
                    },
                ) => {
                    row.key_exists = Some(created_keys.is_empty());
                    (
                        true,
                        approved
                            .as_ref()
                            .is_some_and(|v| v.kind == 3 && v.bytes.first() == Some(&3)),
                    )
                }
                _ => {
                    row.error = Some(ExecError::TypeMismatch);
                    return row;
                }
            };
            row.exists = Some(exists);
            row.already_applied = Some(applied);
            row.presence = if applied {
                Presence::AlreadyApplied
            } else if exists {
                Presence::Exists
            } else {
                Presence::Missing
            };
            row.current = Some(snapshot);
        }
    }
    row
}
