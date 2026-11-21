//! `folder_files`: the files git lists for a rule file's folder, tracked and untracked, relative to the folder.

use crate::rel_to_folder::rel_to_folder;
use rules::RelPath;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// `tracked` then `untracked`, each a path relative to `top` (the repository's top, canonical like `folder`) as
/// procio lists it, made relative to `folder` by `rel_to_folder`, each path once. A path not under `folder` is
/// dropped: procio's untracked listing covers the whole work tree. A path ending in '/' is dropped too: it is a
/// folder git lists as one entry (an untracked nested repository prints as `inner/`), not a file, and its files are
/// not counted; the check reads the raw bytes because `rel_to_folder`'s path components lose the trailing slash, and
/// the entry would then read as a file named `inner` that a rule's paths could match. A tracked entry never ends in
/// '/' (a submodule prints as `subm`). Pure: it reads no file and runs nothing.
pub(crate) fn folder_files(top: &Path, folder: &Path, tracked: &[PathBuf], untracked: &[PathBuf]) -> Vec<RelPath> {
    let mut seen = HashSet::new();
    tracked
        .iter()
        .chain(untracked)
        .filter(|path| !path.as_os_str().as_encoded_bytes().ends_with(b"/"))
        .filter_map(|path| rel_to_folder(top, folder, &path.to_string_lossy()))
        .filter(|path| seen.insert(path.clone()))
        .collect()
}
