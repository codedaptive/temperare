use std::ffi::OsString;

use super::git_checked;
use crate::git::GitError;
use crate::{Cmd, Output};

fn cmd() -> Cmd {
    Cmd { args: vec!["-C".into(), "/r".into(), "show".into(), "HEAD:a".into()], ..Cmd::new("git") }
}

fn out(code: Option<i32>) -> Output {
    Output { code, stdout: b"out".to_vec(), stderr: b"fatal: no\n".to_vec() }
}

#[test]
fn exit_zero_is_the_output() {
    assert_eq!(git_checked(&cmd(), out(Some(0)), &[]).ok(), Some(out(Some(0))));
}

#[test]
fn a_listed_answer_code_is_the_output() {
    assert_eq!(git_checked(&cmd(), out(Some(128)), &[1, 128]).ok(), Some(out(Some(128))));
}

#[test]
fn an_unlisted_code_is_exit_with_args_code_and_stderr() {
    let Err(GitError::Exit { args, code, stderr }) = git_checked(&cmd(), out(Some(2)), &[1]) else {
        panic!("expected Exit")
    };
    assert_eq!(args, [OsString::from("-C"), "/r".into(), "show".into(), "HEAD:a".into()]);
    assert_eq!(code, Some(2));
    assert_eq!(stderr, b"fatal: no\n");
}

#[test]
fn a_signal_is_exit_with_no_code_even_with_answers() {
    let Err(GitError::Exit { code, .. }) = git_checked(&cmd(), out(None), &[1, 128]) else {
        panic!("expected Exit")
    };
    assert_eq!(code, None);
}

#[test]
fn a_nonzero_code_with_no_answers_is_exit() {
    assert!(matches!(git_checked(&cmd(), out(Some(1)), &[]), Err(GitError::Exit { code: Some(1), .. })));
}
