//! `ignored_entries`: the Python files and folders under a rule's folder in git's NUL-separated listing of what it
//! ignores.

use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

/// In `listing`'s order, each non-empty entry (relative to `top`, the repository's top) as `(top.join(entry
/// minus any final /), is_folder)`, kept when it sits strictly under `folder` and is a folder (git ends one with `/`)
/// or ends in `.py`.
/// The paths are made from git's exact bytes, so a non-UTF-8 name survives.
pub(crate) fn ignored_entries(listing: &[u8], top: &Path, folder: &Path) -> Vec<(PathBuf, bool)> {
    listing
        .split(|byte| *byte == 0)
        .filter_map(|entry| {
            let (name, is_folder) = match entry.strip_suffix(b"/") {
                Some(name) => (name, true),
                None => (entry, false),
            };
            let full = top.join(OsStr::from_bytes(name));
            let kept = !name.is_empty()
                && (is_folder || name.ends_with(b".py"))
                && full != folder
                && full.starts_with(folder);
            kept.then_some((full, is_folder))
        })
        .collect()
}
