//! `git_stop`: a git listing's error read as a `Stop`.

use crate::types::Stop;
use procio::{GitError, ProcError};

/// git not found is `NotAvailable` with the why `ran` gives a missing program, `git not found on PATH`; git's own exit
/// is `Failed` with its exit code and procio's error for it; any other error is `Failed` with no exit code and procio's
/// error. procio's error keeps git's message as git wrote it, trimmed at its ends.
pub(crate) fn git_stop(err: GitError) -> Stop {
    match err {
        GitError::Run(ProcError::NotFound { .. }) => Stop::NotAvailable { why: "git not found on PATH".to_string() },
        GitError::Exit { code, .. } => Stop::Failed { code, stderr: err.to_string() },
        other => Stop::Failed { code: None, stderr: other.to_string() },
    }
}
