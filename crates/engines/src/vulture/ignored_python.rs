//! `ignored_python`: what git ignores under a rule's folder that holds Python. It runs git and reads the real
//! filesystem, so it has no unit-test sibling; the zone integration test tests/vulture.rs covers it, and
//! `ignored_entries`, which shapes the listing, has its own.

use super::ignored_entries;
use crate::repo_top::repo_top;
use std::ffi::OsString;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

/// The ignored entries under `folder` (canonical) as `ignored_entries` keeps them from
/// `git ls-files --others --ignored --exclude-standard --directory`, which gives a wholly ignored folder as one entry:
/// every `.py` file, and each folder only when a `.py` file sits somewhere below it. The listing is relative to the
/// repository's top (`repo_top`). `None` when the top cannot be found or
/// resolved, or git cannot run or exits other than 0: with no repository nothing is ignored. The listing runs git
/// through procio, not the host, so it is not counted.
pub(crate) fn ignored_python(folder: &Path) -> Option<Vec<(PathBuf, bool)>> {
    let top = repo_top(folder).ok()?;
    let args = ["ls-files", "--others", "--ignored", "--exclude-standard", "--directory", "-z", "--full-name", "--", ":/"];
    let out = procio::run(&procio::git_cmd(folder, args.map(OsString::from).to_vec())).ok()?;
    if out.code != Some(0) {
        return None;
    }
    let mut entries = ignored_entries(&out.stdout, &top, folder);
    entries.retain(|(path, is_folder)| !is_folder || holds_python(path));
    Some(entries)
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
