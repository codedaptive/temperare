//! `toplevel`: the top directory of the repository holding a path.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use super::exit_128_saying::exit_128_saying;
use crate::git::parse::path_from_bytes;
use crate::git::{git_checked, git_cmd, GitError};
use crate::{run, Cmd, Output};

/// The absolute path of the top of the work tree that holds `repo`, as git prints it (symlinks resolved).
/// Off unix, a non-UTF-8 top level becomes U+FFFD, so the path is not the exact bytes git printed.
/// `repo` outside any repository is `GitError::NotARepository`; a `repo` that does not exist is `GitError::Exit`.
pub fn toplevel(repo: &Path) -> Result<PathBuf, GitError> {
    let cmd = git_cmd(repo, toplevel_args());
    let out = run(&cmd)?;
    toplevel_answer(&cmd, out, repo)
}

pub(crate) fn toplevel_args() -> Vec<OsString> {
    vec!["rev-parse".into(), "--show-toplevel".into()]
}

/// git's "not a git repository" is told apart by its text; a missing directory fails with "cannot change to" under
/// the same exit code 128 and stays an `Exit`.
pub(crate) fn toplevel_answer(cmd: &Cmd, out: Output, repo: &Path) -> Result<PathBuf, GitError> {
    if exit_128_saying(&out, &[b"not a git repository"]) {
        return Err(GitError::NotARepository { repo: repo.to_path_buf() });
    }
    let out = git_checked(cmd, out, &[])?;
    let line = out.stdout.strip_suffix(b"\n").unwrap_or(&out.stdout);
    if line.is_empty() {
        return Err(GitError::BadOutput { args: cmd.args.clone(), stdout: out.stdout });
    }
    Ok(path_from_bytes(line.to_vec()))
}
