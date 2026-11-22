//! Unit tests for set_aside_why.rs.

use crate::set_aside_why;
use rules::{Engine, Rule};

#[test]
fn a_set_aside_rule_gives_its_own_reason_or_no_reason_given() {
    let rows = [
        ("(a) own reason", Some("param max not given"), "param max not given"),
        ("(b) no reason", None, "no reason given"),
    ];
    for (row, reason, expected) in rows {
        let mut rule = Rule::new("swift-x", Engine::SwiftLint, "lint");
        rule.not_applicable = reason.map(str::to_string);
        assert_eq!(set_aside_why(&rule), expected, "row {row}");
    }
}
