//! Unit tests for silenced_by_rule.rs: counts by rule, rule-file order, ids the rule file lacks last, and the sum.

use crate::{silenced_by_rule, RuleCount, Silenced};
use rules::{Engine, Exception, Finding, Kind, RelPath, Rule, RuleFile, Severity};
use std::path::PathBuf;

fn rule_file(ids: &[&str]) -> RuleFile {
    let mut rules = RuleFile::new(PathBuf::from("."));
    rules.rules = ids.iter().map(|id| Rule::new(id, Engine::Clippy, "lint")).collect();
    rules
}

fn silenced(rule: &str, line: u32) -> Silenced {
    let grade = (Engine::Clippy, Kind::Quality, Severity::Error);
    Silenced {
        finding: Finding::new(rule, grade, RelPath::new("a.rs"), line, None, "violation", "Fix it."),
        exception: Exception::new(rule, "a.rs", "approved use", "maintainer, 2026-11-17"),
    }
}

fn count(rule: &str, count: usize) -> RuleCount {
    RuleCount { rule: rule.to_string(), count }
}

#[test]
fn two_rules_one_silenced_twice() {
    let found = vec![silenced("b", 1), silenced("a", 2), silenced("b", 3)];
    assert_eq!(silenced_by_rule(&rule_file(&["a", "b"]), &found), vec![count("a", 1), count("b", 2)]);
}

#[test]
fn nothing_silenced_gives_an_empty_list() {
    assert_eq!(silenced_by_rule(&rule_file(&["a", "b"]), &[]), Vec::new());
}

#[test]
fn a_rule_with_nothing_silenced_is_left_out() {
    let found = vec![silenced("c", 1)];
    assert_eq!(silenced_by_rule(&rule_file(&["a", "c", "b"]), &found), vec![count("c", 1)]);
}

#[test]
fn rules_come_in_rule_file_order_not_alphabetical() {
    let found = vec![silenced("alpha", 1), silenced("zeta", 2), silenced("alpha", 3)];
    assert_eq!(silenced_by_rule(&rule_file(&["zeta", "alpha"]), &found), vec![count("zeta", 1), count("alpha", 2)]);
}

#[test]
fn a_repeated_rule_id_is_counted_once_at_its_first_place() {
    let found = vec![silenced("a", 1), silenced("b", 2)];
    assert_eq!(silenced_by_rule(&rule_file(&["b", "a", "b"]), &found), vec![count("b", 1), count("a", 1)]);
}

#[test]
fn ids_the_rule_file_lacks_go_last_in_id_order() {
    let found = vec![silenced("y", 1), silenced("m", 2), silenced("x", 3), silenced("y", 4)];
    assert_eq!(
        silenced_by_rule(&rule_file(&["m"]), &found),
        vec![count("m", 1), count("x", 1), count("y", 2)]
    );
}

#[test]
fn the_counts_add_up_to_the_silenced_findings() {
    let ids = ["q", "a", "k"];
    let found: Vec<Silenced> = ["k", "z", "a", "k", "b", "q", "k", "z", "a"]
        .iter()
        .zip(1..)
        .map(|(rule, line)| silenced(rule, line))
        .collect();
    let expected: &[&[(&str, usize)]] = &[
        &[],
        &[("k", 1)],
        &[("k", 1), ("z", 1)],
        &[("a", 1), ("k", 1), ("z", 1)],
        &[("a", 1), ("k", 2), ("z", 1)],
        &[("a", 1), ("k", 2), ("b", 1), ("z", 1)],
        &[("q", 1), ("a", 1), ("k", 2), ("b", 1), ("z", 1)],
        &[("q", 1), ("a", 1), ("k", 3), ("b", 1), ("z", 1)],
        &[("q", 1), ("a", 1), ("k", 3), ("b", 1), ("z", 2)],
        &[("q", 1), ("a", 2), ("k", 3), ("b", 1), ("z", 2)],
    ];
    for (upto, expected_counts) in expected.iter().enumerate() {
        let counts = silenced_by_rule(&rule_file(&ids), &found[..upto]);
        assert_eq!(counts.iter().map(|counted| counted.count).sum::<usize>(), upto);
        assert!(counts.iter().all(|counted| counted.count >= 1));
        let actual: Vec<_> = counts.iter().map(|counted| (counted.rule.as_str(), counted.count)).collect();
        assert_eq!(actual, *expected_counts, "first {upto} findings");
    }
}
