//! `git_rev_parse`: the commit a revision names, asked of git through ccvcslib.

use super::oid_of_revision::oid_of_revision;
use ccvcslib::{GitEnvPolicy, GitVcs, Vcs, VcsError};
use report::Oid;
use std::path::Path;

/// The commit `name` names in the repository holding `dir`; None when it names none.
pub(crate) fn git_rev_parse(dir: &Path, name: &str) -> Result<Option<Oid>, VcsError> {
    GitVcs { policy: GitEnvPolicy::Lifecycle }.resolve_revision(dir, name).and_then(oid_of_revision)
}
