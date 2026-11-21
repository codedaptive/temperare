//! `ThresholdRun`: what a rule's threshold adds to its check run, holding the temporary file or folder it needs.

use super::{TempDir, TempFile};
use std::ffi::OsString;

/// The arguments and environment a threshold adds to one check run. The temporary file or folder it was written to
/// lives as long as the value, so the value is dropped right after the run.
#[derive(Debug)]
pub(crate) struct ThresholdRun {
    /// The arguments, passed after the param arguments.
    pub args: Vec<OsString>,
    /// The variables added to the run's environment.
    pub env: Vec<(OsString, OsString)>,
    pub(super) _file: Option<TempFile>,
    pub(super) _dir: Option<TempDir>,
}
