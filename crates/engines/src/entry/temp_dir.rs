//! `TempDir`: a folder holding one file cctao writes for one engine run, living as long as the value.

use std::path::PathBuf;

/// A new folder holding one file written for one run (a config an engine reads from a folder it is told of);
/// dropping it removes the file when this value created it, then the folder when it is empty.
#[derive(Debug)]
pub struct TempDir {
    /// The folder, under the system's temporary folder.
    pub path: PathBuf,
    /// The one file in it, set only once this value has created it.
    pub(super) file: Option<PathBuf>,
}
