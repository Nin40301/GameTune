use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum GameStep {
    Set { ids: Vec<String> },
    Ping {},
    Finish {},
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GameFrame {
    pub nonce: String,
    pub sequence: u64,
    pub step: GameStep,
}
pub struct GameGate {
    nonce: String,
    next: u64,
    allowed: BTreeSet<String>,
}
impl GameGate {
    pub fn new(nonce: String, allowed: Vec<String>) -> Result<Self> {
        if !valid_nonce(&nonce) {
            return Err(Error::InvalidPayload);
        }
        Ok(Self {
            nonce,
            next: 2,
            allowed: allowed.into_iter().collect(),
        })
    }
    pub fn accept(&mut self, bytes: &[u8]) -> Result<GameStep> {
        if bytes.len() > MAX_FRAME {
            return Err(Error::InvalidPayload);
        }
        let frame: GameFrame = serde_json::from_slice(bytes).map_err(|_| Error::InvalidPayload)?;
        if frame.nonce != self.nonce || frame.sequence != self.next || self.next > 1_000_000 {
            return Err(Error::Replay);
        }
        self.next += 1;
        if let GameStep::Set { ids } = &frame.step
            && (ids.len() > self.allowed.len()
                || ids.iter().collect::<BTreeSet<_>>().len() != ids.len()
                || ids.iter().any(|id| !self.allowed.contains(id)))
        {
            return Err(Error::InvalidItem);
        }
        Ok(frame.step)
    }
}
