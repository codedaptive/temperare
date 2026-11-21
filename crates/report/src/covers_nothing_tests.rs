//! Unit tests for covers_nothing.rs: the exact JSON text.

use crate::test_json::to_json;
use crate::CoversNothing;
use rules::Engine;

#[test]
fn serializes_rule_and_engine_in_order() {
    let item = CoversNothing { rule: "own-x".to_string(), engine: Engine::SwiftLint };
    assert_eq!(to_json(&item), r#"{"rule":"own-x","engine":"swiftlint"}"#);
}
