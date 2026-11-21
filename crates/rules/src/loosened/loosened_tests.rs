//! Unit tests for loosened.rs, on rule files built here: tightened and unchanged files give nothing; the groups' order.

use super::{Loosening, LooseningKind, loosened};
use crate::{Engine, Exception, Rule, RuleFile, Severity};
use std::path::PathBuf;

fn rule(id: &str, position: usize) -> Rule {
    Rule { position, ..Rule::new(id, Engine::Ruff, "E1") }
}

fn file(rules: Vec<Rule>, exceptions: Vec<Exception>, baseline: &str) -> RuleFile {
    let mut file = RuleFile::new(PathBuf::from("."));
    file.rules = rules;
    file.exceptions = exceptions;
    file.baseline = vec![baseline.to_string()];
    file
}

fn found(kind: LooseningKind, about: &str, before: Option<&str>, after: Option<&str>) -> Loosening {
    Loosening { kind, about: about.to_string(), before: before.map(String::from), after: after.map(String::from) }
}

#[test]
fn tightened_or_unchanged_files_give_nothing() {
    let a = Rule {
        severity: Severity::Warning,
        paths: vec!["src/**".to_string()],
        exclude: vec!["gen/**".to_string()],
        ..rule("a", 0)
    };
    let tighter_a = Rule { severity: Severity::Error, paths: vec!["src/**".to_string(), "lib/**".to_string()], ..rule("a", 0) };
    let base = file(vec![a], vec![Exception::new("a", "src/x.py", "legacy", "bob")], "standard@1");
    let tightened = file(vec![tighter_a, rule("b", 1)], vec![], "standard@2");
    assert_eq!(loosened(&base, &tightened), vec![], "tightened");
    assert_eq!(loosened(&base, &base.clone()), vec![], "unchanged");
}

#[test]
fn rules_then_exceptions_then_sets_with_rules_by_position() {
    let base = file(vec![rule("x", 1), rule("y", 0)], vec![], "standard@1");
    let mut now = file(vec![Rule { severity: Severity::Warning, ..rule("y", 0) }], vec![], "other@1");
    now.exceptions = vec![Exception::new("y", "src/y.py", "legacy", "bob")];
    let expected = vec![
        found(LooseningKind::SeverityLowered, "y", Some("error"), Some("warning")),
        found(LooseningKind::RuleGone, "x", None, None),
        found(LooseningKind::ExceptionAdded, "y", None, Some("src/y.py")),
        found(LooseningKind::SetDropped, "standard", Some("1"), None),
    ];
    assert_eq!(loosened(&base, &now), expected);
}
