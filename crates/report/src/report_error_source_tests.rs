//! Unit tests for report_error_source.rs.

use crate::ReportError;
use std::error::Error;
use std::path::PathBuf;

#[test]
fn git_has_the_git_error_as_source() {
    let err = ReportError::Git(procio::GitError::NotARepository { repo: PathBuf::from("/tmp/x") });
    assert_eq!(err.source().map(ToString::to_string), Some("/tmp/x: not a git repository".to_string()));
}

#[test]
fn folder_has_no_source() {
    assert!(ReportError::Folder { folder: PathBuf::from("a"), why: "b".to_string() }.source().is_none());
}
