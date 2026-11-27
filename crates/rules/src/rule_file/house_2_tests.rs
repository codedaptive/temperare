//! ZUT for house_2.rs.

use super::house_1_tests::{expected_rows as house_1_rows, Row};
use super::house_2::HOUSE_2;
use super::load_set::load_set;
use crate::{Engine, Kind, Rule, Severity};

fn rules() -> Vec<Rule> {
    load_set(&HOUSE_2).unwrap_or_else(|p| panic!("house@2 has problems: {p:?}")).0
}

/// The 39 rules: house@1's 36 unchanged and in their order, then the three register-citation rules.
fn expected_rows() -> Vec<Row<'static>> {
    let citation = |id, check, path| {
        (id, "ast-grep", check, Kind::Quality, Severity::Warning, toml::Table::new(), Vec::new(), true, path, 4)
    };
    let mut rows = house_1_rows();
    rows.extend([
        citation("tm-house-no-register-citation-rust", "ast-grep/tm-house-no-register-citation-rust.yml", "meter.rs"),
        citation("tm-house-no-register-citation-swift", "ast-grep/tm-house-no-register-citation-swift.yml", "MeterTests.swift"),
        citation("tm-house-no-register-citation-python", "ast-grep/tm-house-no-register-citation-python.yml", "test_meter.py"),
    ]);
    rows
}

#[test]
fn house_2_rules_come_back_in_set_order_as_written() {
    let rules = rules();
    let found: Vec<Row> = rules
        .iter()
        .map(|rule| {
            let expected = &rule.cases().unwrap_or_else(|| panic!("{} has no cases", rule.id)).expected;
            (
                rule.id.as_str(),
                rule.engine.name(),
                rule.check.as_str(),
                rule.kind,
                rule.severity,
                rule.params.clone(),
                rule.required_params.clone(),
                rule.only_with_base,
                expected.path.as_str(),
                expected.line,
            )
        })
        .collect();
    assert_eq!(found, expected_rows());
}

#[test]
fn every_house_2_rule_carries_both_cases_for_itself() {
    let rules = rules();
    let found: Vec<_> = rules
        .iter()
        .map(|rule| {
            let cases = rule.cases().map(|c| (!c.failing.is_empty(), !c.passing.is_empty(), c.expected.rule_id.as_str()));
            (rule.id.as_str(), cases)
        })
        .collect();
    let expected: Vec<_> = rules.iter().map(|rule| (rule.id.as_str(), Some((true, true, rule.id.as_str())))).collect();
    assert_eq!(found, expected);
}

#[test]
fn every_house_2_ast_grep_rule_and_only_those_carry_yaml() {
    let rules = rules();
    let found: Vec<_> = rules.iter().map(|rule| (rule.id.as_str(), rule.embedded_yaml().is_some())).collect();
    let expected: Vec<_> = rules.iter().map(|rule| (rule.id.as_str(), rule.engine == Engine::AstGrep)).collect();
    assert_eq!(found, expected);
}
