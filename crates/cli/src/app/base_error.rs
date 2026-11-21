//! `BaseError`: why the rule file at a `--base` revision could not be read.

use crate::UsageError;
use std::path::PathBuf;

/// The base rule file's failure: a usage error (the revision, git, or the file's bytes), or its problems with the
/// label `<path> (at <rev>)` they are printed under.
pub(crate) enum BaseError {
    Usage(UsageError),
    Problems(PathBuf, Vec<rules::Problem>),
}
