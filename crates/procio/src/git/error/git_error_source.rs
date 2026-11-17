//! `std::error::Error` for `GitError`, so callers can use `?` into boxed errors and walk the cause.

use std::error::Error;

use super::GitError;

impl Error for GitError {
    /// The `ProcError` behind a `Run`; the other variants are git's own answer and have no underlying cause.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GitError::Run(err) => Some(err),
            GitError::NotARepository { .. } | GitError::Exit { .. } | GitError::BadOutput { .. } => None,
        }
    }
}
