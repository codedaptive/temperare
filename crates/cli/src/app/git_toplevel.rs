//! `git_toplevel`: the top of the repository holding a folder, asked of git through ccvcslib.

use ccvcslib::{GitEnvPolicy, GitVcs, Vcs, VcsError};
use std::path::{Path, PathBuf};

/// The top folder of the working copy `dir` is in; `VcsError::NotAWorkingCopy` outside one.
pub(crate) fn git_toplevel(dir: &Path) -> Result<PathBuf, VcsError> {
    GitVcs { policy: GitEnvPolicy::Lifecycle }.working_copy_root(dir)
}
