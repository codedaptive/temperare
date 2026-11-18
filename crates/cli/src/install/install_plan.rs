//! `InstallPlan`: what one `temperare install` will do, decided before anything is written.

use std::path::PathBuf;

/// `place`: where to copy the running binary, or `None` to copy nothing. `link`: the `tm` symlink to make, as
/// (link, target), or `None`. `skills`: each skill file to write. `notes`: one line for each thing skipped or worth
/// knowing (no copy, a clash on `tm`, a folder not on PATH).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallPlan {
    pub place: Option<PathBuf>,
    pub link: Option<(PathBuf, PathBuf)>,
    pub skills: Vec<PathBuf>,
    pub notes: Vec<String>,
}
