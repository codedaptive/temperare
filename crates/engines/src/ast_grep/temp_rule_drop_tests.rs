//! ZUT for temp_rule_drop.rs: dropping a rule removes its file, and a file already gone is no panic.

use super::TempRule;

#[test]
fn dropping_the_rule_removes_its_file() {
    let rule = TempRule::write("id: a\n").unwrap();
    let path = rule.path.clone();
    assert!(path.exists());
    drop(rule);
    assert!(!path.exists());
}

#[test]
fn dropping_a_rule_whose_file_is_gone_does_not_panic() {
    let rule = TempRule::write("id: a\n").unwrap();
    std::fs::remove_file(&rule.path).unwrap();
    let path = rule.path.clone();
    drop(rule);
    assert!(!path.exists());
}
