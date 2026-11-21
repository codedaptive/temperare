//! Unit tests for rules_args.rs: the default is no rule file, text output, no base and no --loosened.

use super::RulesArgs;

#[test]
fn default_has_no_rule_file_no_json_no_base_and_is_not_loosened() {
    let d = RulesArgs::default();
    assert_eq!(d.rules, None);
    assert!(!d.json);
    assert_eq!(d.base, None);
    assert!(!d.loosened);
}
