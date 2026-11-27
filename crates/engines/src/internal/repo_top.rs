//! `repo_top`: the repository's top above a rule's folder, in the folder's canonical form. It runs git, so it has no
//! unit-test sibling: its callers' zone integration tests cover it.

use crate::git_stop::git_stop;
use crate::types::Stop;
use ccvcslib::{GitVcs, Vcs};
use std::path::{Path, PathBuf};

/// The top of the repository `folder` is in (ccvcslib's `working_copy_root`), canonicalized so it shares `folder`'s
/// form. A git error is `git_stop`'s `Stop`; a top that cannot be canonicalized is `Failed` with no exit code, naming
/// it.
pub(crate) fn repo_top(folder: &Path) -> Result<PathBuf, Stop> {
    let top = GitVcs::default().working_copy_root(folder).map_err(|err| git_stop(err, procio::which))?;
    std::fs::canonicalize(&top).map_err(|err| Stop::Failed {
        code: None,
        stderr: format!("cannot resolve the repository top {}: {err}", top.display()),
    })
}
