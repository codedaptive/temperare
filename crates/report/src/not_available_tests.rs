//! ZUT for not_available.rs: the exact JSON text.

use crate::test_json::to_json;
use crate::NotAvailable;
use rules::Engine;

#[test]
fn serializes_rule_engine_and_why_in_order() {
    let item = NotAvailable { rule: "no-bool".to_string(), engine: Engine::AstGrep, why: "not installed".to_string() };
    assert_eq!(to_json(&item), r#"{"rule":"no-bool","engine":"ast-grep","why":"not installed"}"#);
}
