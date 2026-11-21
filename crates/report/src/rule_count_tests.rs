//! Unit tests for rule_count.rs: the exact JSON text.

use crate::test_json::to_json;
use crate::RuleCount;

#[test]
fn serializes_rule_then_count() {
    let count = RuleCount { rule: "no-unwrap".to_string(), count: 3 };
    assert_eq!(to_json(&count), r#"{"rule":"no-unwrap","count":3}"#);
}
