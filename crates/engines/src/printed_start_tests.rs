//! Unit tests for printed_start.rs: which stream is quoted and where it is cut.

use crate::printed_start::printed_start;
use procio::Output;

fn output(stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(1), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

#[test]
fn stderr_is_quoted_when_it_is_not_empty() {
    assert_eq!(printed_start(&output(b"out text", b"err text")), "err text");
}

#[test]
fn stdout_is_quoted_when_stderr_is_empty() {
    assert_eq!(printed_start(&output(b"out text", b"")), "out text");
}

#[test]
fn both_empty_quote_nothing() {
    assert_eq!(printed_start(&output(b"", b"")), "");
}

#[test]
fn the_quote_is_cut_at_200_bytes() {
    let long = [b"0123456789".repeat(20).as_slice(), b"CUT".as_slice()].concat();
    let quoted = printed_start(&output(b"", &long));
    assert_eq!(quoted.len(), 200);
    assert!(quoted.ends_with("0123456789"), "{quoted}");
    assert!(!quoted.contains("CUT"), "{quoted}");
}
