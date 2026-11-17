use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;

use super::GitError;
use crate::ProcError;

fn args(list: &[&str]) -> Vec<OsString> {
    list.iter().map(OsString::from).collect()
}

#[test]
fn run_shows_the_proc_errors_own_line() {
    let err = GitError::Run(ProcError::NotFound { program: "git".into() });
    assert_eq!(err.to_string(), "git: not found");
}

#[test]
fn not_a_repository_names_the_repo() {
    let err = GitError::NotARepository { repo: "/tmp/plain".into() };
    assert_eq!(err.to_string(), "/tmp/plain: not a git repository");
}

#[test]
fn exit_names_the_args_the_code_and_the_trimmed_stderr() {
    let err = GitError::Exit {
        args: args(&["-C", "/r", "show", "HEAD:a.txt"]),
        code: Some(128),
        stderr: b"fatal: path 'a.txt' does not exist in 'HEAD'\n".to_vec(),
    };
    assert_eq!(err.to_string(), "git -C /r show HEAD:a.txt: exit 128: fatal: path 'a.txt' does not exist in 'HEAD'");
}

#[test]
fn exit_with_empty_stderr_ends_at_the_code() {
    let err = GitError::Exit { args: args(&["-C", "/r", "diff"]), code: Some(2), stderr: Vec::new() };
    assert_eq!(err.to_string(), "git -C /r diff: exit 2");
}

#[test]
fn exit_with_whitespace_only_stderr_ends_at_the_code() {
    let err = GitError::Exit { args: args(&["-C", "/r", "diff"]), code: Some(1), stderr: b" \n".to_vec() };
    assert_eq!(err.to_string(), "git -C /r diff: exit 1");
}

#[test]
fn exit_without_a_code_says_a_signal_ended_it() {
    let err = GitError::Exit { args: args(&["-C", "/r", "diff"]), code: None, stderr: b"x\n".to_vec() };
    assert_eq!(err.to_string(), "git -C /r diff: killed by a signal: x");
}

#[test]
fn non_utf8_args_and_stderr_are_written_lossily() {
    let err = GitError::Exit {
        args: vec![OsString::from("-C"), OsString::from_vec(vec![b'/', 0xff])],
        code: Some(128),
        stderr: vec![b'e', 0xfe],
    };
    assert_eq!(err.to_string(), "git -C /\u{fffd}: exit 128: e\u{fffd}");
}

#[test]
fn bad_output_names_the_args() {
    let err = GitError::BadOutput { args: args(&["-C", "/r", "rev-parse", "--show-toplevel"]), stdout: b"?".to_vec() };
    assert_eq!(err.to_string(), "git -C /r rev-parse --show-toplevel: unexpected output");
}
