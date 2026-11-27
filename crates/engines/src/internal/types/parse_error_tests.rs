//! Unit test for parse_error.rs: plain data; equality is by its why.

use super::ParseError;

#[test]
fn parse_errors_compare_by_why() {
    let error = ParseError { why: "clippy: no build-finished record".to_string() };
    assert_eq!(error.clone(), ParseError { why: "clippy: no build-finished record".to_string() });
    assert_ne!(error, ParseError { why: "ruff: not JSON".to_string() });
}
