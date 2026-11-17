//! `untracked_files`: the untracked files git does not ignore.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::git::parse::{parse_nul_list, path_from_bytes};
use crate::git::{git_checked, git_cmd, GitError};
use crate::{run, Cmd, Output};

/// The untracked files not excluded by git's standard ignore rules (`.gitignore`, `.git/info/exclude`,
/// `core.excludesFile`), in git's order, each relative to the top of the repository. The listing covers the whole
/// work tree even when `repo` is a subdirectory.
pub fn untracked_files(repo: &Path) -> Result<Vec<PathBuf>, GitError> {
    let cmd = git_cmd(repo, untracked_files_args());
    let out = run(&cmd)?;
    untracked_files_answer(&cmd, out)
}

/// `-z` keeps each path's bytes exactly, with no quoting of unusual names. `--full-name` makes each path relative to
/// the top of the repository, and the `:/` pathspec (top magic, honoured because `git_cmd` clears
/// `GIT_LITERAL_PATHSPECS`) widens the listing from `repo`'s directory to the whole work tree.
pub(crate) fn untracked_files_args() -> Vec<OsString> {
    vec![
        "ls-files".into(),
        "--others".into(),
        "--exclude-standard".into(),
        "-z".into(),
        "--full-name".into(),
        "--".into(),
        ":/".into(),
    ]
}

pub(crate) fn untracked_files_answer(cmd: &Cmd, out: Output) -> Result<Vec<PathBuf>, GitError> {
    let out = git_checked(cmd, out, &[])?;
    Ok(parse_nul_list(&out.stdout).into_iter().map(path_from_bytes).collect())
}
