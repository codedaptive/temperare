//! Unit tests for failed.rs: the exact JSON text, `code` a number or null.

use crate::test_json::to_json;
use crate::Failed;
use rules::Engine;

fn failed(code: Option<i32>) -> Failed {
    Failed { rule: "lint".to_string(), engine: Engine::Clippy, code, stderr_tail: "a\nb".to_string() }
}

#[test]
fn an_exit_code_serializes_as_a_number() {
    assert_eq!(to_json(&failed(Some(101))), r#"{"rule":"lint","engine":"clippy","code":101,"stderr_tail":"a\nb"}"#);
}

#[test]
fn no_exit_code_serializes_as_null() {
    assert_eq!(to_json(&failed(None)), r#"{"rule":"lint","engine":"clippy","code":null,"stderr_tail":"a\nb"}"#);
}
