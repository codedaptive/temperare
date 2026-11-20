//! ZUT for git_stop.rs: each kind of `GitError`, built by hand, read as its `Stop`. Nothing runs.

use super::git_stop;
use crate::types::Stop;
use procio::{GitError, ProcError};
use std::ffi::OsString;
use std::path::PathBuf;
use std::time::Duration;

fn args(args: &[&str]) -> Vec<OsString> {
    args.iter().map(OsString::from).collect()
}

#[test]
fn git_not_found_is_not_available() {
    let got = git_stop(GitError::Run(ProcError::NotFound { program: "git".into() }));
    assert_eq!(got, Stop::NotAvailable { why: "git not found on PATH".to_string() });
}

#[test]
fn git_that_could_not_run_is_failed_with_no_code() {
    let io = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
    let got = git_stop(GitError::Run(ProcError::Spawn { program: "git".into(), io }));
    assert_eq!(got, Stop::Failed { code: None, stderr: "git: could not run: denied".to_string() });
}

#[test]
fn git_that_timed_out_is_failed_with_no_code() {
    let got = git_stop(GitError::Run(ProcError::TimedOut { program: "git".into(), after: Duration::from_secs(2) }));
    assert_eq!(got, Stop::Failed { code: None, stderr: "git: timed out after 2s".to_string() });
}

#[test]
fn an_exit_is_failed_with_its_code_and_the_one_line_error() {
    let err = GitError::Exit { args: args(&["ls-files"]), code: Some(128), stderr: b"fatal: x\n".to_vec() };
    assert_eq!(git_stop(err), Stop::Failed { code: Some(128), stderr: "git ls-files: exit 128: fatal: x".to_string() });
}

#[test]
fn an_exit_by_a_signal_is_failed_with_no_code() {
    let err = GitError::Exit { args: args(&["ls-files"]), code: None, stderr: Vec::new() };
    assert_eq!(git_stop(err), Stop::Failed { code: None, stderr: "git ls-files: killed by a signal".to_string() });
}

#[test]
fn not_a_repository_is_failed_with_no_code() {
    let got = git_stop(GitError::NotARepository { repo: PathBuf::from("/no/such/repo") });
    assert_eq!(got, Stop::Failed { code: None, stderr: "/no/such/repo: not a git repository".to_string() });
}

#[test]
fn bad_output_is_failed_with_no_code() {
    let err = GitError::BadOutput { args: args(&["rev-parse", "--show-toplevel"]), stdout: Vec::new() };
    assert_eq!(
        git_stop(err),
        Stop::Failed { code: None, stderr: "git rev-parse --show-toplevel: unexpected output".to_string() }
    );
}
