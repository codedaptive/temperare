//! Unit tests for not_applicable.rs: the exact JSON text.

use crate::test_json::to_json;
use crate::NotApplicable;
use rules::Engine;

#[test]
fn serializes_rule_engine_and_why_in_order() {
    let item = NotApplicable {
        rule: "swift-x".to_string(),
        engine: Engine::SwiftLint,
        why: "reports only what a change adds; run with --base".to_string(),
    };
    assert_eq!(
        to_json(&item),
        r#"{"rule":"swift-x","engine":"swiftlint","why":"reports only what a change adds; run with --base"}"#
    );
}

#[test]
fn the_engine_serializes_as_its_rule_file_name() {
    let item = NotApplicable { rule: "a".to_string(), engine: Engine::AstGrep, why: "param x not given".to_string() };
    assert_eq!(to_json(&item), r#"{"rule":"a","engine":"ast-grep","why":"param x not given"}"#);
}
