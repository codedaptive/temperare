//! `git_stop`: a ccvcslib error read as a `Stop`.

use crate::types::Stop;
use ccvcslib::VcsError;
use std::path::PathBuf;

/// git that could not run, or was stopped before it answered (ccvcslib's time limits), is `NotOnPath` naming `git` when
/// `which` finds no git on PATH, as `ran` names a missing program, and else `Failed` with no exit code and ccvcslib's
/// error. A folder in no working copy is `Failed` with no exit code and ccvcslib's error. git's own failure is `Failed`
/// with git's exit code, if any, and ccvcslib's message. `which` is asked only for that first kind; it is the one PATH
/// lookup `git_stop` makes, and callers pass `procio::which`.
pub(crate) fn git_stop(err: VcsError, which: impl FnOnce(&'static str) -> Option<PathBuf>) -> Stop {
    match err {
        VcsError::Unavailable => match which("git") {
            None => Stop::NotOnPath { program: "git".to_string() },
            Some(_) => Stop::Failed { code: None, stderr: err.to_string() },
        },
        VcsError::Failed(failure) => Stop::Failed { code: failure.exit_status, stderr: failure.message },
        VcsError::NotAWorkingCopy => Stop::Failed { code: None, stderr: err.to_string() },
    }
}
