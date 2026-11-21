//! Unit tests for apply_override.rs.

use super::apply_override::apply_override;
use super::override_entry::OverrideEntry;
use crate::{Engine, Problem, RelPath, Rule, Severity};

/// The override's `rule` string starts at byte 7 of this text: line 1, column 8.
const TEXT: &str = "rule = \"tm-a\"\n";

fn rule() -> Rule {
    let mut rule = Rule::new("tm-a", Engine::Clippy, "clippy::a");
    rule.paths = vec!["**/*.rs".to_string()];
    rule.params.insert("keep".to_string(), toml::Value::Integer(1));
    rule.params.insert("change".to_string(), toml::Value::Integer(2));
    rule
}

fn nothing() -> OverrideEntry {
    OverrideEntry { rule: "tm-a".to_string(), at: 7, severity: None, off: false, paths: None, exclude: None, params: None }
}

fn apply(o: &OverrideEntry) -> (Option<Rule>, Vec<Problem>) {
    let mut out = Vec::new();
    let applied = apply_override(TEXT, rule(), o, &mut out);
    (applied, out)
}

#[test]
fn off_gives_no_rule_and_no_problem() {
    let (applied, out) = apply(&OverrideEntry { off: true, severity: Some(Severity::Warning), ..nothing() });
    assert!(applied.is_none(), "{applied:?}");
    assert_eq!(out, vec![]);
}

#[test]
fn an_override_with_no_change_keeps_the_rule_as_it_is() {
    let (applied, out) = apply(&nothing());
    let applied = applied.expect("not off");
    assert_eq!(out, vec![]);
    assert_eq!(applied.severity, Severity::Error);
    assert_eq!(applied.paths, vec!["**/*.rs".to_string()]);
    assert_eq!(applied.params, rule().params);
}

#[test]
fn a_given_severity_replaces_the_rules() {
    let (applied, _) = apply(&OverrideEntry { severity: Some(Severity::Warning), ..nothing() });
    assert_eq!(applied.expect("not off").severity, Severity::Warning);
}

#[test]
fn given_paths_and_exclude_replace_the_rules_and_rebuild_its_scope() {
    let o = OverrideEntry {
        paths: Some(vec!["src/**".to_string()]),
        exclude: Some(vec!["src/gen/**".to_string()]),
        ..nothing()
    };
    let (applied, out) = apply(&o);
    let applied = applied.expect("not off");
    assert_eq!(out, vec![]);
    assert_eq!(applied.paths, vec!["src/**".to_string()]);
    assert_eq!(applied.exclude, vec!["src/gen/**".to_string()]);
    assert!(applied.applies(&RelPath::new("src/a.py")), "in the new paths");
    assert!(!applied.applies(&RelPath::new("lib/a.rs")), "in the old paths only");
    assert!(!applied.applies(&RelPath::new("src/gen/a.py")), "excluded");
}

#[test]
fn given_params_are_merged_key_by_key_with_the_override_winning() {
    let mut params = toml::Table::new();
    params.insert("change".to_string(), toml::Value::Integer(20));
    params.insert("add".to_string(), toml::Value::String("x".to_string()));
    let (applied, _) = apply(&OverrideEntry { params: Some(params), ..nothing() });
    let mut expected = toml::Table::new();
    expected.insert("keep".to_string(), toml::Value::Integer(1));
    expected.insert("change".to_string(), toml::Value::Integer(20));
    expected.insert("add".to_string(), toml::Value::String("x".to_string()));
    assert_eq!(applied.expect("not off").params, expected);
}

#[test]
fn a_bad_glob_is_a_problem_at_the_override_and_no_rule() {
    let (applied, out) = apply(&OverrideEntry { paths: Some(vec!["src/[a.rs".to_string()]), ..nothing() });
    assert!(applied.is_none(), "{applied:?}");
    let what = "bad glob \"src/[a.rs\": error parsing glob 'src/[a.rs': unclosed character class; missing ']'";
    let expected = Problem { line: 1, col: 8, what: what.to_string(), fix: "Correct the override's globs.".to_string() };
    assert_eq!(out, vec![expected]);
}
