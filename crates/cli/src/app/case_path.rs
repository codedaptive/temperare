//! `case_path`: where a baseline case file is written, kept inside its case folder.

use super::case_rel_path::case_rel_path;
use std::path::{Path, PathBuf};

/// The path of case file `name` inside `folder`: `folder` joined, part by part, with the parts of
/// `case_rel_path(name)`, and that function's why when it refuses the name. A name of only `.` parts (`.`, `./`) is
/// `folder` itself: its empty relative path joins nothing, so no trailing separator. Pure: it never canonicalizes
/// and never touches the disk.
pub(crate) fn case_path(folder: &Path, name: &str) -> Result<PathBuf, String> {
    let parts = case_rel_path(name)?;
    Ok(parts.split('/').filter(|part| !part.is_empty()).fold(folder.to_path_buf(), |path, part| path.join(part)))
}
