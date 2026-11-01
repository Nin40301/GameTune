#![forbid(unsafe_code)]

pub mod engine;
pub mod executor;
pub mod journal;
pub mod model;
pub mod profile;
pub mod protection;

pub use engine::*;
pub use executor::*;
pub use journal::*;
pub use model::*;
pub use profile::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid catalog: {0}")]
    InvalidCatalog(String),
    #[error("unknown item: {0}")]
    UnknownItem(String),
    #[error("item is incompatible with this machine: {0}")]
    Incompatible(String),
    #[error("explicit acknowledgement required: {0}")]
    ConfirmationRequired(String),
    #[error("journal recovery required before applying changes")]
    RecoveryRequired,
    #[error("journal is locked or unavailable: {0}")]
    JournalLocked(String),
    #[error("corrupt journal: {0}")]
    CorruptJournal(String),
    #[error("journal writer failed; reopen it before continuing")]
    JournalPoisoned,
    #[error("profile is invalid: {0}")]
    InvalidProfile(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("clock formatting failed: {0}")]
    Clock(#[from] time::error::Format),
}

pub type Result<T> = std::result::Result<T, Error>;
