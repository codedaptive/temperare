//! `folder_from_top`: the rule file's folder relative to the repository top.

use std::path::{Path, PathBuf};

/// `folder` relative to `top`: empty when they are the same, the rest of the path when `folder` lies under `top`
/// (compared component by component), `None` when it does not. Both are absolute with symlinks resolved:
/// `top` as git prints it, `folder` as the caller canonicalized it.
pub(crate) fn folder_from_top(top: &Path, folder: &Path) -> Option<PathBuf> {
    folder.strip_prefix(top).ok().map(Path::to_path_buf)
}
