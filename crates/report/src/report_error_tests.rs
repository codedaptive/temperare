//! ZUT for report_error.rs: each variant carries what it was given.

use crate::ReportError;
use std::path::{Path, PathBuf};

#[test]
fn git_carries_the_git_error() {
    let err = ReportError::Git(procio::GitError::NotARepository { repo: PathBuf::from("/tmp/x") });
    assert!(matches!(err, ReportError::Git(procio::GitError::NotARepository { repo }) if repo == Path::new("/tmp/x")));
}

#[test]
fn folder_carries_the_folder_and_why() {
    let err = ReportError::Folder { folder: PathBuf::from("rules"), why: "gone".to_string() };
    assert!(matches!(err, ReportError::Folder { folder, why } if folder == Path::new("rules") && why == "gone"));
}
