//! `ignored_entries`: the Python files and folders under a rule's folder in the entries git lists as ignored,
//! read from ccvcslib's `ignored_folders`.

use std::path::{Path, PathBuf};

/// In `entries`' order, each non-empty entry (relative to `top`, the repository's top) as `(top.join(entry
/// minus any final /), is_folder)`, kept when it sits strictly under `folder` and is a folder (git ends one with `/`)
/// or ends in `.py`.
/// An entry whose name holds U+FFFD is left out: ccvcslib decodes git's listing lossily, so a non-UTF-8 name arrives
/// with U+FFFD in place of its bytes, and that path is not the real file or folder. tm leaves such an entry out of
/// --exclude, so vulture scans the ignored file or folder: what it finds there is reported, and a name used there
/// counts as used.
pub(crate) fn ignored_entries(entries: &[String], top: &Path, folder: &Path) -> Vec<(PathBuf, bool)> {
    entries
        .iter()
        .filter_map(|entry| {
            let (name, is_folder) = match entry.strip_suffix('/') {
                Some(name) => (name, true),
                None => (entry.as_str(), false),
            };
            let full = top.join(name);
            let kept = !name.is_empty()
                && !name.contains(char::REPLACEMENT_CHARACTER)
                && (is_folder || name.ends_with(".py"))
                && full != folder
                && full.starts_with(folder);
            kept.then_some((full, is_folder))
        })
        .collect()
}
