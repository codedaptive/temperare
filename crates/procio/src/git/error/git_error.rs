//! `GitError`: the ways a git operation can fail to give an answer.

use std::ffi::OsString;
use std::path::PathBuf;

use crate::ProcError;

/// Why a git operation gave no answer.
#[derive(Debug)]
pub enum GitError {
    /// git could not be run, or did not finish (ProcError::NotFound when git is not installed: "not available").
    Run(ProcError),
    /// `repo` is not inside a git work tree. Only `toplevel` returns it.
    NotARepository { repo: PathBuf },
    /// git exited with a code the operation does not take as an answer. `args` are the arguments given to git
    /// (from `-C <repo>` on); `code` is None when a signal ended it.
    Exit { args: Vec<OsString>, code: Option<i32>, stderr: Vec<u8> },
    /// git exited 0 but printed something the operation cannot read.
    BadOutput { args: Vec<OsString>, stdout: Vec<u8> },
}
