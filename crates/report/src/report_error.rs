//! `ReportError`: why a report against a base could not be built.

use std::path::PathBuf;

/// Why `build_report_with_no_files` gave no report.
#[derive(Debug)]
pub enum ReportError {
    /// A git operation failed: finding the repository top, the added lines or the untracked files.
    Git(procio::GitError),
    /// The rule file's folder could not be canonicalized, or is not inside the repository top.
    Folder { folder: PathBuf, why: String },
}
