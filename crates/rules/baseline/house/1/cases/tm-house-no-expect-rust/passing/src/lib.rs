//! Where the user's settings live.

use std::env::VarError;
use std::path::PathBuf;

/// The user's home folder, from `HOME`.
pub fn home() -> Result<PathBuf, VarError> {
    std::env::var("HOME").map(PathBuf::from)
}
