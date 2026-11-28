//! ZUT for override_owner.rs.

use super::override_owner::override_owner;

#[test]
fn names_the_override_by_its_rule_when_it_has_one() {
    for (rule, expected) in [(Some("tm-x"), "[[override]] for \"tm-x\""), (None, "[[override]]")] {
        assert_eq!(override_owner(rule), expected, "rule {rule:?}");
    }
}
