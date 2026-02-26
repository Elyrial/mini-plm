use thiserror::Error;
use crate::core::Lifecycle;

#[derive(Debug, Error)]
pub enum PlmError {
    #[error("part not found: {0}")]
    NotFound(String),

    #[error("part already exists: {0}")]
    AlreadyExists(String),

    #[error("invalid lifecycle transition: {from:?} -> {to:?} ({msg})")]
    InvalidTransition {
        from: Lifecycle,
        to: Lifecycle,
        msg: String,
    },

    #[error("storage error: {0}")]
    Storage(String),
}

