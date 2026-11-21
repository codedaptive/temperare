//! Unit tests for swiftlint_refusal.rs: `custom_rules` is refused with its reason; any other check is not.

use super::swiftlint_refusal::swiftlint_refusal;

#[test]
fn custom_rules_is_refused_with_the_reason() {
    assert_eq!(
        swiftlint_refusal("custom_rules"),
        Some("swiftlint check `custom_rules` names no rule swiftlint runs under --only-rule: name the custom rule's own identifier".to_string())
    );
}

#[test]
fn other_checks_are_not_refused() {
    for check in ["force_cast", "my_custom_rule", "custom_rules_x", ""] {
        assert_eq!(swiftlint_refusal(check), None, "{check}");
    }
}
