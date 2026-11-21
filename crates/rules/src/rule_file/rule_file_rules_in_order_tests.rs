//! Unit tests for rule_file_rules_in_order.rs, on a rule file built here.

use crate::{Engine, Rule, RuleFile};
use std::path::PathBuf;

fn at(id: &str, position: usize) -> Rule {
    Rule { position, ..Rule::new(id, Engine::Ruff, "E1") }
}

fn placed(rules: Vec<&Rule>) -> Vec<(&str, usize)> {
    rules.into_iter().map(|rule| (rule.id.as_str(), rule.position)).collect()
}

#[test]
fn rules_and_not_applicable_rules_interleave_by_position() {
    let mut file = RuleFile::new(PathBuf::from("."));
    file.rules = vec![at("r0", 0), at("r3", 3), at("r4", 4), at("r6", 6)];
    file.not_applicable_rules = vec![at("n1", 1), at("n2", 2), at("n5", 5)];
    let expected = vec![("r0", 0), ("n1", 1), ("n2", 2), ("r3", 3), ("r4", 4), ("n5", 5), ("r6", 6)];
    assert_eq!(placed(file.rules_in_order()), expected);
}

#[test]
fn the_order_is_by_position_not_by_id_or_list_order() {
    let mut file = RuleFile::new(PathBuf::from("."));
    file.rules = vec![at("a", 2), at("c", 0)];
    file.not_applicable_rules = vec![at("b", 1)];
    assert_eq!(placed(file.rules_in_order()), vec![("c", 0), ("b", 1), ("a", 2)]);
}

#[test]
fn rules_sharing_a_position_keep_rules_first_then_not_applicable_rules() {
    let mut file = RuleFile::new(PathBuf::from("."));
    file.rules = vec![at("z", 0), at("y", 0)];
    file.not_applicable_rules = vec![at("a", 0)];
    assert_eq!(placed(file.rules_in_order()), vec![("z", 0), ("y", 0), ("a", 0)]);
}

#[test]
fn an_empty_file_has_no_rules_in_order() {
    let file = RuleFile::new(PathBuf::from("."));
    assert_eq!(placed(file.rules_in_order()), Vec::<(&str, usize)>::new());
}
