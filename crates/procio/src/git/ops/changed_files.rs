//! `changed_files`: the tracked files that differ from a base commit.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::git::parse::{parse_nul_list, path_from_bytes};
use crate::git::{git_checked, git_cmd, GitError};
use crate::{run, Cmd, Output};

/// The files whose content in the working tree differs from `base`, in git's order, relative to the top of the
/// repository. Untracked files are not listed. A renamed file shows as its old path and its new one.
pub fn changed_files(repo: &Path, base: &str) -> Result<Vec<PathBuf>, GitError> {
    let cmd = git_cmd(repo, changed_files_args(base));
    let out = run(&cmd)?;
    changed_files_answer(&cmd, out)
}

/// `-z` keeps each path's bytes exactly, with no quoting of unusual names.
pub(crate) fn changed_files_args(base: &str) -> Vec<OsString> {
    vec![
        "diff".into(),
        "--name-only".into(),
        "-z".into(),
        "--no-renames".into(),
        "--end-of-options".into(),
        base.into(),
        "--".into(),
    ]
}

pub(crate) fn changed_files_answer(cmd: &Cmd, out: Output) -> Result<Vec<PathBuf>, GitError> {
    let out = git_checked(cmd, out, &[])?;
    Ok(parse_nul_list(&out.stdout).into_iter().map(path_from_bytes).collect())
}
