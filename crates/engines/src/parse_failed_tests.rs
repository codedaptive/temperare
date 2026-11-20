//! Unit tests for parse_failed.rs: the exit code kept, and the engine's stderr preferred over the parser's why.

use crate::parse_failed::parse_failed;
use crate::types::{ParseError, Stop};
use procio::Output;

fn why(text: &str) -> ParseError {
    ParseError { why: text.to_string() }
}

#[test]
fn the_engines_stderr_is_the_failure_text_when_it_wrote_any() {
    let out = Output { code: Some(101), stdout: b"{".to_vec(), stderr: b"error: could not compile".to_vec() };
    let got = parse_failed(&out, why("clippy printed no JSON"));
    assert_eq!(got, Stop::Failed { code: Some(101), stderr: "error: could not compile".to_string() });
}

#[test]
fn with_no_stderr_the_parsers_why_is_the_failure_text() {
    let out = Output { code: Some(2), stdout: b"garbage".to_vec(), stderr: Vec::new() };
    let got = parse_failed(&out, why("ruff output is not JSON: garbage"));
    assert_eq!(got, Stop::Failed { code: Some(2), stderr: "ruff output is not JSON: garbage".to_string() });
}

#[test]
fn stderr_that_is_not_utf8_is_read_lossily() {
    let out = Output { code: None, stdout: Vec::new(), stderr: vec![b'b', b'a', b'd', 0xff] };
    let got = parse_failed(&out, why("unused"));
    assert_eq!(got, Stop::Failed { code: None, stderr: "bad\u{fffd}".to_string() });
}
