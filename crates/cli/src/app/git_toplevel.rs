//! `git_toplevel`: the top of the repository holding a folder, asked of git through ccvcslib.

use ccvcslib::{GitEnvPolicy, GitVcs, Vcs, VcsError};
use std::path::{Path, PathBuf};

/// The top folder of the working copy `dir` is in, in the platform's native form;
/// `VcsError::NotAWorkingCopy` outside one.
///
/// Git for Windows prints `C:/...`; rebuilding from components gives `C:\...` (no `\\?\`
/// prefix, unlike canonicalize), so a path joined onto it has one separator style.
pub(crate) fn git_toplevel(dir: &Path) -> Result<PathBuf, VcsError> {
    GitVcs { policy: GitEnvPolicy::Lifecycle }
        .working_copy_root(dir)
        .map(|top| top.components().collect())
}
