//! ZUT for rules_args.rs: the default is no rule file and text output.

use super::RulesArgs;

#[test]
fn default_has_no_rule_file_and_no_json() {
    let d = RulesArgs::default();
    assert_eq!(d.rules, None);
    assert!(!d.json);
}
