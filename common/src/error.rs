//! Common error types.

use thiserror::Error;

/// Common errors shared by multiple crates.
#[derive(Debug, Error)]
pub enum CommonError {
    #[error("Operation is not supported")]
    Unsupported,

    #[error("Invalid version")]
    InvalidVersion,

    #[error("Unknown error")]
    Unknown,
}