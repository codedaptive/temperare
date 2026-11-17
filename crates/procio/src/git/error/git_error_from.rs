//! `From<ProcError>` for `GitError`, so a git operation can use `?` on `run`.

use crate::ProcError;

use super::GitError;

impl From<ProcError> for GitError {
    /// A failure to run git at all becomes `Run`, keeping the `ProcError` (and so `NotFound`) intact.
    fn from(err: ProcError) -> GitError {
        GitError::Run(err)
    }
}
