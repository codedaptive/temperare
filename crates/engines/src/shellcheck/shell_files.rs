//! `shellcheck::shell_files`: the shell scripts under a rule's folder that shellcheck is given. It runs git and
//! reads the real filesystem, so its tests are the zone integration tests in integration/shell_files_tests.rs and it
//! has no unit-test sibling.

use super::{git_stop, is_regular_file, read_head, shell_files_with};
use crate::types::Stop;
use std::path::{Path, PathBuf};

/// The scripts under `folder` (canonical), relative to it, as `shell_files_with` keeps them from the repository's
/// listing: the tracked files (procio drops those deleted from disk and every skip-worktree entry, and lists an
/// unmerged file once, so a conflicted script reaches shellcheck and its conflict markers fail the rule rather than
/// read clean), then the untracked files git does not ignore. The untracked listing covers the whole work tree, so
/// the folder filter keeps only what is under `folder`. No path reached through a symlink is kept: `is_regular_file`
/// follows no component below the repository's top, the file itself included, so neither a symlinked script nor a
/// script under a symlinked folder reaches shellcheck. The top is canonicalized so it shares `folder`'s form and
/// holds no link. The listing runs git through procio, not
/// the host, so it is not counted; a git error is `git_stop`'s `Stop`. Off unix `read_head` reads nothing, so a file
/// with no extension is left out there.
pub(crate) fn shell_files(folder: &Path) -> Result<Vec<PathBuf>, Stop> {
    let top = procio::toplevel(folder).map_err(git_stop)?;
    let top = std::fs::canonicalize(&top).map_err(|err| Stop::Failed {
        code: None,
        stderr: format!("cannot resolve the repository top {}: {err}", top.display()),
    })?;
    let mut listed = procio::tracked_files::<&str>(folder, &[]).map_err(git_stop)?;
    listed.extend(procio::untracked_files(folder).map_err(git_stop)?);
    Ok(shell_files_with(folder, &top, &listed, &|full: &Path| is_regular_file(&top, full), &read_head))
}
