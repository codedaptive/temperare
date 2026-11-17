use std::ffi::OsString;

use super::rev_parse::{rev_parse_answer, rev_parse_args};
use crate::git::{GitError, Oid};
use crate::{Cmd, Output};

const SHA1: &str = "0123456789abcdef0123456789abcdef01234567";

fn args(list: &[&str]) -> Vec<OsString> {
    list.iter().map(OsString::from).collect()
}

fn cmd_args() -> Vec<OsString> {
    args(&["-C", "/r", "rev-parse", "--verify", "-q", "--end-of-options", "HEAD^{commit}"])
}

fn cmd() -> Cmd {
    Cmd { args: cmd_args(), ..Cmd::new("git") }
}

fn out(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

fn bad_output(result: Result<Option<Oid>, GitError>, stdout: &[u8]) {
    match result {
        Err(GitError::BadOutput { args: a, stdout: s }) => {
            assert_eq!(a, cmd_args());
            assert_eq!(s, stdout.to_vec());
        }
        other => panic!("expected BadOutput, got {other:?}"),
    }
}

#[test]
fn args_put_the_rev_after_end_of_options_peeled_to_a_commit() {
    assert_eq!(
        rev_parse_args("main"),
        args(&["rev-parse", "--verify", "-q", "--end-of-options", "main^{commit}"])
    );
}

#[test]
fn an_option_shaped_rev_stays_after_end_of_options() {
    assert_eq!(
        rev_parse_args("--help"),
        args(&["rev-parse", "--verify", "-q", "--end-of-options", "--help^{commit}"])
    );
}

#[test]
fn exit_0_is_the_oid_without_its_newline() {
    let got = rev_parse_answer(&cmd(), out(0, b"0123456789abcdef0123456789abcdef01234567\n", b"")).unwrap();
    assert_eq!(got.map(|o| o.as_str().to_string()), Some(SHA1.to_string()));
}

#[test]
fn exit_0_with_a_sha256_id_is_the_oid() {
    let id = "ab".repeat(32);
    let got = rev_parse_answer(&cmd(), out(0, format!("{id}\n").as_bytes(), b"")).unwrap();
    assert_eq!(got.map(|o| o.as_str().to_string()), Some(id));
}

#[test]
fn exit_1_is_none() {
    assert_eq!(rev_parse_answer(&cmd(), out(1, b"", b"")).unwrap(), None);
}

#[test]
fn exit_128_is_exit_with_args_code_and_stderr() {
    match rev_parse_answer(&cmd(), out(128, b"", b"fatal: not a git repository\n")) {
        Err(GitError::Exit { args: a, code, stderr }) => {
            assert_eq!(a, cmd_args());
            assert_eq!(code, Some(128));
            assert_eq!(stderr, b"fatal: not a git repository\n".to_vec());
        }
        other => panic!("expected Exit, got {other:?}"),
    }
}

#[test]
fn a_signal_is_exit_without_a_code() {
    let o = Output { code: None, stdout: Vec::new(), stderr: Vec::new() };
    match rev_parse_answer(&cmd(), o) {
        Err(GitError::Exit { code, .. }) => assert_eq!(code, None),
        other => panic!("expected Exit, got {other:?}"),
    }
}

#[test]
fn exit_0_with_a_short_id_is_bad_output() {
    bad_output(rev_parse_answer(&cmd(), out(0, b"0123456\n", b"")), b"0123456\n");
}

#[test]
fn exit_0_with_two_newlines_is_bad_output() {
    let stdout = b"0123456789abcdef0123456789abcdef01234567\n\n";
    bad_output(rev_parse_answer(&cmd(), out(0, stdout, b"")), stdout);
}

#[test]
fn exit_0_with_non_utf8_is_bad_output() {
    bad_output(rev_parse_answer(&cmd(), out(0, b"\xff\n", b"")), b"\xff\n");
}

#[test]
fn exit_0_with_nothing_is_bad_output() {
    bad_output(rev_parse_answer(&cmd(), out(0, b"", b"")), b"");
}
