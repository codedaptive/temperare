//! `folder_from_top`: the rule file's folder relative to the repository top.

use std::path::{Path, PathBuf};

/// `folder` relative to `top`: empty when they are the same, the rest of the path when `folder` lies under `top`
/// (compared component by component), `None` when it does not. The caller canonicalizes both, so they are absolute,
/// have symlinks resolved and share one path form (a prefix of one kind never matches a prefix of another).
pub(crate) fn folder_from_top(top: &Path, folder: &Path) -> Option<PathBuf> {
    folder.strip_prefix(top).ok().map(Path::to_path_buf)
}
