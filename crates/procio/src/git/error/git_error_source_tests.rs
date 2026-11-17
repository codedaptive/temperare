use std::error::Error;

use super::GitError;
use crate::ProcError;

#[test]
fn run_has_the_proc_error_as_source() {
    let err = GitError::Run(ProcError::NotFound { program: "git".into() });
    assert_eq!(err.source().map(ToString::to_string), Some("git: not found".to_string()));
}

#[test]
fn not_a_repository_has_no_source() {
    assert!(GitError::NotARepository { repo: "/r".into() }.source().is_none());
}

#[test]
fn exit_has_no_source() {
    assert!(GitError::Exit { args: Vec::new(), code: Some(1), stderr: Vec::new() }.source().is_none());
}

#[test]
fn bad_output_has_no_source() {
    assert!(GitError::BadOutput { args: Vec::new(), stdout: Vec::new() }.source().is_none());
}
