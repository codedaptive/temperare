//! Unit tests for param_why.rs: the stderr text of a bad and of an unknown param.

use super::param_why;
use crate::ParamError;

#[test]
fn a_bad_param_names_the_entry_the_param_and_why() {
    let err = ParamError::Bad { name: "allow".to_string(), why: "must be a list of strings".to_string() };
    assert_eq!(param_why("clippy", err), "clippy param allow: must be a list of strings");
}

#[test]
fn an_unknown_param_says_unknown() {
    assert_eq!(param_why("ruff", ParamError::Unknown { name: "width".to_string() }), "ruff param width: unknown");
}
