//! Unit tests for engine_version.rs: the exact JSON text, with a version and with none.

use crate::test_json::to_json;
use crate::EngineVersion;
use rules::Engine;

#[test]
fn a_version_serializes_as_its_text_after_the_engine_name() {
    let one = EngineVersion { engine: Engine::AstGrep, version: Some("0.39.1".to_string()) };
    assert_eq!(to_json(&one), r#"{"engine":"ast-grep","version":"0.39.1"}"#);
}

#[test]
fn no_version_serializes_as_null() {
    let one = EngineVersion { engine: Engine::Ruff, version: None };
    assert_eq!(to_json(&one), r#"{"engine":"ruff","version":null}"#);
}
