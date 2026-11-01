#![deny(unsafe_code)]
//! One-use protocol. Only the native module contains documented Win32 FFI.
mod protocol;
pub use protocol::*;
mod game_protocol;
pub use game_protocol::*;
mod transaction;
pub use transaction::*;
#[cfg(windows)]
pub mod game_pipe;
#[cfg(windows)]
#[allow(unsafe_code)]
pub mod native;
#[cfg(windows)]
pub mod pipe;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("catalog_mismatch")]
    CatalogMismatch,
    #[error("invalid_payload")]
    InvalidPayload,
    #[error("legacy_data")]
    LegacyData,
    #[error("identity_mismatch")]
    Identity,
    #[error("replay")]
    Replay,
    #[error("confirmation_required")]
    Confirmation,
    #[error("invalid_item")]
    InvalidItem,
    #[error("timeout")]
    Timeout,
    #[error("windows_required")]
    Windows,
    #[error("uac_cancelled")]
    UacCancelled,
    #[error("operation_failed")]
    Operation,
    #[error("io")]
    Io(#[from] std::io::Error),
    #[error("json")]
    Json(#[from] serde_json::Error),
}
pub type Result<T> = std::result::Result<T, Error>;
pub fn nonce() -> Result<String> {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes).map_err(|_| Error::Operation)?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}
pub fn valid_nonce(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
