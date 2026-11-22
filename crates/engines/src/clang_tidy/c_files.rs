//! `c_files`: the C-family sources under a rule's folder that clang-tidy is given. It runs git and reads the real
//! filesystem, so its test is the zone integration test tests/clang_tidy.rs and it has no unit-test sibling.

use super::is_c_source;
use crate::folder_files::folder_files;
use crate::shellcheck::git_stop;
use crate::types::Stop;
use std::path::{Path, PathBuf};

/// The C-family sources under `folder` (canonical), relative to it, in listing order: the tracked files, then the
/// untracked files git does not ignore, shaped by `folder_files` against the repository's top (canonicalized so it
/// shares `folder`'s form). A file is kept when `is_c_source` says so and it is a regular file, not a symlink: git
/// lists a symlinked folder as one entry, so only the last component can be a link, and no link reaches clang-tidy.
/// The listing runs git through procio, not the host, so it is not counted; a git error is `git_stop`'s `Stop`.
pub(crate) fn c_files(folder: &Path) -> Result<Vec<PathBuf>, Stop> {
    let top = procio::toplevel(folder).map_err(git_stop)?;
    let top = std::fs::canonicalize(&top).map_err(|err| Stop::Failed {
        code: None,
        stderr: format!("cannot resolve the repository top {}: {err}", top.display()),
    })?;
    let tracked = procio::tracked_files::<&str>(folder, &[]).map_err(git_stop)?;
    let untracked = procio::untracked_files(folder).map_err(git_stop)?;
    Ok(folder_files(&top, folder, &tracked, &untracked)
        .into_iter()
        .map(|path| PathBuf::from(path.as_str()))
        .filter(|path| is_c_source(path))
        .filter(|path| std::fs::symlink_metadata(folder.join(path)).is_ok_and(|meta| meta.file_type().is_file()))
        .collect())
}
