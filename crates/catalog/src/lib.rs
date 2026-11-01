#![forbid(unsafe_code)]

use gametune_core::{Catalog, Error, Item, Result};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub const EMBEDDED_ITEMS: &str = include_str!("../../../catalog/items.json");
pub const CATALOG_SCHEMA: &str = include_str!("../../../catalog/schema.json");
pub const PROFILE_SCHEMA: &str = include_str!("../../../catalog/profile.schema.json");
const HASHES: &str = include_str!("../../../catalog/sha256.json");
const GAME_ACTIONS: &str = include_str!("../../../catalog/game-actions.json");
const GAME_SCHEMA: &str = include_str!("../../../catalog/game-actions.schema.json");
pub fn embedded_hash() -> String {
    let mut hash = Sha256::new();
    hash.update(EMBEDDED_ITEMS);
    hash.update(GAME_ACTIONS);
    format!("{hash:x}", hash = hash.finalize())
}
pub fn game_actions(catalog: &Catalog) -> Result<Vec<gametune_core::temporary::GameAction>> {
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Actions {
        version: u8,
        actions: Vec<gametune_core::temporary::GameAction>,
    }
    let hashes: BTreeMap<String, String> = serde_json::from_str(HASHES)?;
    verify_hash(
        GAME_SCHEMA.as_bytes(),
        hashes
            .get("game-actions.schema.json")
            .ok_or_else(|| Error::InvalidCatalog("missing game schema hash".into()))?,
    )?;
    validate(GAME_SCHEMA, GAME_ACTIONS)?;
    verify_hash(
        GAME_ACTIONS.as_bytes(),
        hashes
            .get("game-actions.json")
            .ok_or_else(|| Error::InvalidCatalog("missing game action hash".into()))?,
    )?;
    let document: Actions = serde_json::from_str(GAME_ACTIONS)?;
    if document.version != 1 {
        return Err(Error::InvalidCatalog("game action version".into()));
    }
    gametune_core::temporary::validate_actions(catalog, &document.actions)?;
    Ok(document.actions)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Document {
    version: u8,
    items: Vec<Item>,
}

pub fn verify_hash(bytes: &[u8], expected: &str) -> Result<()> {
    if format!("{:x}", Sha256::digest(bytes)) != expected {
        return Err(Error::InvalidCatalog("embedded SHA-256 mismatch".into()));
    }
    Ok(())
}

fn validate(schema: &str, json: &str) -> Result<()> {
    let schema = serde_json::from_str(schema)?;
    let value = serde_json::from_str(json)?;
    let validator =
        jsonschema::validator_for(&schema).map_err(|e| Error::InvalidCatalog(e.to_string()))?;
    validator
        .validate(&value)
        .map_err(|e| Error::InvalidCatalog(e.to_string()))
}

/// Validation for contributor tooling; does not authorize a replacement catalog.
pub fn validate_catalog_json(json: &str) -> Result<()> {
    validate(CATALOG_SCHEMA, json)
}

pub fn validate_profile_json(json: &str) -> Result<()> {
    validate(PROFILE_SCHEMA, json)
}

/// Only embedded data becomes the runtime catalog. Profiles cannot replace it.
pub fn load_embedded() -> Result<Catalog> {
    let hashes: BTreeMap<String, String> = serde_json::from_str(HASHES)?;
    for (name, document) in [
        ("items.json", EMBEDDED_ITEMS),
        ("schema.json", CATALOG_SCHEMA),
        ("profile.schema.json", PROFILE_SCHEMA),
    ] {
        let expected = hashes
            .get(name)
            .ok_or_else(|| Error::InvalidCatalog("missing hash".into()))?;
        verify_hash(document.as_bytes(), expected)?;
    }
    validate_catalog_json(EMBEDDED_ITEMS)?;
    let document: Document = serde_json::from_str(EMBEDDED_ITEMS)?;
    if document.version != 1 {
        return Err(Error::InvalidCatalog("unsupported version".into()));
    }
    let catalog = Catalog::try_new(document.items)?;
    game_actions(&catalog)?;
    Ok(catalog)
}
