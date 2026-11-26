//! `ReportError`: why a report against a base could not be built.

use std::path::PathBuf;

/// Why `build_report_with_no_files` gave no report.
#[derive(Debug)]
pub enum ReportError {
    /// A ccvcslib question failed: the working-copy top, the added lines or the untracked entries.
    Vcs(ccvcslib::VcsError),
    /// The rule file's folder could not be canonicalized, or is not inside the repository top.
    Folder { folder: PathBuf, why: String },
}
