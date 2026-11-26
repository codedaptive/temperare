//! `ignored_python`: what git ignores under a rule's folder that holds Python. It runs git through ccvcslib and
//! reads the real filesystem, so it has no unit-test sibling; the zone integration test tests/vulture.rs covers it,
//! and `ignored_entries`, which shapes the listing, has its own.

use super::ignored_entries;
use crate::git_stop::git_stop;
use crate::repo_top::repo_top;
use crate::types::Stop;
use ccvcslib::{GitVcs, Vcs, VcsError};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

/// The ignored entries under `folder` (canonical) as `ignored_entries` keeps them from ccvcslib's `ignored_folders`
/// (`git ls-files -z -o -i --exclude-standard --directory`), which gives a wholly ignored folder as one entry:
/// every `.py` file, and each folder only when a `.py` file sits somewhere below it. The listing is asked for at the
/// repository's top (`repo_top`), since ccvcslib lists relative to the folder it runs in. A folder in no working copy
/// has nothing ignored; any other failure to find the top is `repo_top`'s `Stop`, and a failed listing `git_stop`'s.
/// ccvcslib runs git itself, not the host, so the listing is not counted.
pub(crate) fn ignored_python(folder: &Path) -> Result<Vec<(PathBuf, bool)>, Stop> {
    let top = match repo_top(folder) {
        Ok(top) => top,
        Err(stop) => return in_no_working_copy(folder, stop),
    };
    let listed = GitVcs::default().ignored_folders(&top).map_err(|err| git_stop(err, procio::which))?;
    let mut entries = ignored_entries(&listed, &top, folder);
    entries.retain(|(path, is_folder)| !is_folder || holds_python(path));
    Ok(entries)
}

/// Nothing ignored when `folder` is in no working copy, else `repo_top`'s `stop`. `repo_top`'s `Stop` does not keep
/// ccvcslib's error, so git is asked again; only after `repo_top` failed, so a found top costs one call.
fn in_no_working_copy(folder: &Path, stop: Stop) -> Result<Vec<(PathBuf, bool)>, Stop> {
    match GitVcs::default().working_copy_root(folder) {
        Err(VcsError::NotAWorkingCopy) => Ok(Vec::new()),
        _ => Err(stop),
    }
}

/// Whether a `.py` regular file sits anywhere below `dir`, stopping at the first. `symlink_metadata` never follows a
/// symlink, so a linked file or folder is not looked into; a folder that cannot be read holds nothing.
fn holds_python(dir: &Path) -> bool {
    let Ok(read) = std::fs::read_dir(dir) else { return false };
    read.flatten().any(|child| {
        let path = child.path();
        match std::fs::symlink_metadata(&path) {
            Ok(meta) if meta.is_dir() => holds_python(&path),
            Ok(meta) => meta.is_file() && path.as_os_str().as_bytes().ends_with(b".py"),
            Err(_) => false,
        }
    })
}
