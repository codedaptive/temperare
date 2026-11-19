//! Unit tests for report_error_display.rs.

use crate::ReportError;
use std::path::PathBuf;

#[test]
fn git_is_prefixed_with_git() {
    let err = ReportError::Git(procio::GitError::NotARepository { repo: PathBuf::from("/tmp/x") });
    assert_eq!(err.to_string(), "git: /tmp/x: not a git repository");
}

#[test]
fn folder_names_the_folder_and_why() {
    let err = ReportError::Folder { folder: PathBuf::from("/a/b"), why: "not inside the repository at /r".to_string() };
    assert_eq!(err.to_string(), "rule file folder /a/b: not inside the repository at /r");
}
