//! Where the user's settings live.

use std::path::PathBuf;

/// The user's home folder, from `HOME`.
pub fn home() -> PathBuf {
    PathBuf::from(std::env::var("HOME").expect("HOME is set"))
}
