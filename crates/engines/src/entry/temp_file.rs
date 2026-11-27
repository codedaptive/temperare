//! `TempFile`: a file cctao writes for one engine run, living as long as the value.

use std::path::PathBuf;

/// A file written for one run (a filled check file, a config made from a rule's params); dropping it removes the
/// file.
#[derive(Debug)]
pub struct TempFile {
    /// The file, under the system's temporary folder.
    pub path: PathBuf,
}
