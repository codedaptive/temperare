//! `shellcheck::shell_files`: the shell scripts under a rule's folder that shellcheck is given. It runs git and
//! reads the real filesystem, so its tests are the zone integration tests in integration/shell_files_tests.rs and it
//! has no unit-test sibling.

use super::{read_head, shell_files_with};
use crate::checked_out::checked_out;
use crate::git_stop::git_stop;
use crate::is_regular_file::is_regular_file;
use crate::repo_top::repo_top;
use crate::types::Stop;
use ccvcslib::{GitVcs, Vcs};
use std::path::{Path, PathBuf};

/// The scripts under `folder` (canonical), relative to it, as `shell_files_with` keeps them from the repository's
/// listing: ccvcslib's `tracked_entries` as `checked_out` keeps them (deleted and skip-worktree entries left out, an
/// unmerged path once), then the untracked files git does not ignore. Both listings cover the whole working copy, so
/// the folder filter keeps only what is under `folder`. No path reached through a symlink is kept: `is_regular_file`
/// follows no component below the repository's top, the file itself included, so neither a symlinked script nor a
/// script under a symlinked folder reaches shellcheck. The top is `repo_top`'s, so it shares `folder`'s form and holds
/// no link. The listing runs git through ccvcslib, not the host, so it is not counted; a git error is `git_stop`'s
/// `Stop`. Off unix `read_head` reads nothing, so a file with no extension is left out there.
pub(crate) fn shell_files(folder: &Path) -> Result<Vec<PathBuf>, Stop> {
    let top = repo_top(folder)?;
    let vcs = GitVcs::default();
    let mut listed = checked_out(vcs.tracked_entries(&top).map_err(|err| git_stop(err, procio::which))?);
    listed.extend(vcs.untracked_entries(&top).map_err(|err| git_stop(err, procio::which))?);
    Ok(shell_files_with(folder, &top, &listed, &|full: &Path| is_regular_file(&top, full), &read_head))
}
