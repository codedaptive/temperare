//! `InstallPlan`: what one `cctao install` will do, decided before anything is written.

use std::path::PathBuf;

/// `place`: where to copy the running binary, or `None` to copy nothing. `skills`: each skill file to write. `notes`:
/// one line for each thing skipped or worth knowing (no copy, a folder not on PATH).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallPlan {
    pub place: Option<PathBuf>,
    pub skills: Vec<PathBuf>,
    pub notes: Vec<String>,
}
