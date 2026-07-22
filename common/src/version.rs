//! Version information shared across the pApO project.

use serde::{Deserialize, Serialize};

/// Represents a semantic version.
///
/// Example:
/// - 1.0.0
/// - 1.2.4
/// - 2.0.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Version {
    /// Major version.
    pub major: u16,

    /// Minor version.
    pub minor: u16,

    /// Patch version.
    pub patch: u16,
}