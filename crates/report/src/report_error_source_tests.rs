//! Unit tests for report_error_source.rs.

use crate::ReportError;
use ccvcslib::VcsError;
use std::error::Error;
use std::path::PathBuf;

#[test]
fn vcs_has_the_vcs_error_as_source() {
    let err = ReportError::Vcs(VcsError::NotAWorkingCopy);
    assert_eq!(err.source().map(ToString::to_string), Some("not a working copy".to_string()));
}

#[test]
fn folder_has_no_source() {
    assert!(ReportError::Folder { folder: PathBuf::from("a"), why: "b".to_string() }.source().is_none());
}
