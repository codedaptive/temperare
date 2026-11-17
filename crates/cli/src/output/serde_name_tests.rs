//! ZUT for serde_name.rs: a JSON string gives its text; anything else, or an error, gives "".

use super::serde_name::serde_name;
use serde_json::json;

#[test]
fn a_json_string_is_its_text() {
    assert_eq!(serde_name(Ok(json!("ruff"))), "ruff");
}

#[test]
fn a_json_number_is_empty() {
    assert_eq!(serde_name(Ok(json!(1))), "");
}

#[test]
fn an_error_is_empty() {
    assert_eq!(serde_name(serde_json::from_str::<serde_json::Value>("{")), "");
}
