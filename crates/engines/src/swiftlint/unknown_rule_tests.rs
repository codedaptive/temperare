//! ZUT for unknown_rule.rs: swiftlint's unknown-id warning picked out of its stderr.

use super::unknown_rule::unknown_rule;
use procio::Output;

fn with_stderr(stderr: &[u8]) -> Output {
    Output { code: Some(0), stdout: b"[\n\n]".to_vec(), stderr: stderr.to_vec() }
}

#[test]
fn the_warning_amid_the_identifier_list_is_that_line() {
    let out = with_stderr(
        b"Linting Swift files in current working directory\nwarning: 'no_such_rule' is not a valid rule identifier\nValid rule identifiers:\nforce_cast\nforce_try\n",
    );
    assert_eq!(unknown_rule(&out), Some("warning: 'no_such_rule' is not a valid rule identifier".to_string()));
}

#[test]
fn a_crlf_line_loses_its_carriage_return() {
    let out = with_stderr(b"warning: 'no_such_rule' is not a valid rule identifier\r\nValid rule identifiers:\r\nforce_cast\r\n");
    assert_eq!(unknown_rule(&out), Some("warning: 'no_such_rule' is not a valid rule identifier".to_string()));
}

#[test]
fn stderr_without_the_warning_is_none() {
    let out = with_stderr(b"Linting Swift files in current working directory\nDone linting! Found 0 violations.\n");
    assert_eq!(unknown_rule(&out), None);
}

#[test]
fn empty_stderr_is_none() {
    assert_eq!(unknown_rule(&with_stderr(b"")), None);
}
