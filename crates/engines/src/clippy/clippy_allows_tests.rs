//! Unit tests for clippy_allows.rs: no params, the `allow` list in order, an unknown key, a bad `allow`, non-table params.

use serde_json::json;

use super::clippy_allows;
use crate::ParamError;

fn bad(name: &str, why: &str) -> Result<Vec<String>, ParamError> {
    Err(ParamError::Bad { name: name.to_string(), why: why.to_string() })
}

#[test]
fn empty_params_allow_nothing() {
    assert_eq!(clippy_allows(&json!({})), Ok(vec![]));
}

#[test]
fn the_allow_list_comes_back_in_order() {
    let got = clippy_allows(&json!({"allow": ["clippy::result_large_err", "clippy::module_inception"]}));
    assert_eq!(got, Ok(vec!["clippy::result_large_err".to_string(), "clippy::module_inception".to_string()]));
}

#[test]
fn an_empty_allow_list_allows_nothing() {
    assert_eq!(clippy_allows(&json!({"allow": []})), Ok(vec![]));
}

#[test]
fn a_key_other_than_allow_is_bad_and_named() {
    let got = clippy_allows(&json!({"alow": ["clippy::module_inception"]}));
    assert_eq!(got, bad("alow", "a clippy rule reads only the `allow` param"));
}

#[test]
fn an_unknown_key_beside_allow_is_still_bad() {
    let got = clippy_allows(&json!({"allow": ["x"], "deny": ["y"]}));
    assert_eq!(got, bad("deny", "a clippy rule reads only the `allow` param"));
}

#[test]
fn an_allow_that_is_a_string_is_bad() {
    assert_eq!(clippy_allows(&json!({"allow": "x"})), bad("allow", "must be a list of strings"));
}

#[test]
fn an_allow_list_with_a_non_string_item_is_bad() {
    assert_eq!(clippy_allows(&json!({"allow": ["x", 2]})), bad("allow", "must be a list of strings"));
}

#[test]
fn params_that_are_not_a_table_are_bad() {
    assert!(matches!(clippy_allows(&json!(["x"])), Err(ParamError::Bad { name, .. }) if name == "params"));
}
