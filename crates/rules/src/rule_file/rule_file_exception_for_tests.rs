//! ZUT for rule_file_exception_for.rs.

use crate::{Engine, Exception, Finding, Kind, RelPath, RuleFile, Severity};
use std::path::PathBuf;

fn exception(rule: &str, path: &str, what: Option<&str>, reason: &str) -> Exception {
    Exception {
        rule: rule.to_string(),
        path: RelPath::new(path),
        what: what.map(str::to_string),
        reason: reason.to_string(),
        approved: "Bob, 2026-11-17".to_string(),
    }
}

fn file(exceptions: Vec<Exception>) -> RuleFile {
    let mut f = RuleFile::new(PathBuf::from("."));
    f.exceptions = exceptions;
    f
}

fn finding(rule: &str, path: &str, what: &str) -> Finding {
    Finding {
        rule: rule.to_string(),
        engine: Engine::Clippy,
        kind: Kind::Quality,
        severity: Severity::Error,
        owner: None,
        path: RelPath::new(path),
        line: 3,
        col: Some(9),
        what: what.to_string(),
        fix: "Return a typed error.".to_string(),
    }
}

fn reason_for<'a>(rules: &'a RuleFile, f: &Finding) -> Option<&'a str> {
    rules.exception_for(f).map(|e| e.reason.as_str())
}

#[test]
fn what_contained_in_the_finding_matches() {
    let rules = file(vec![exception("no-unwrap", "src/a.rs", Some("unwrap"), "one")]);
    assert_eq!(reason_for(&rules, &finding("no-unwrap", "src/a.rs", "used `unwrap()` on a Result")), Some("one"));
}

#[test]
fn what_not_contained_in_the_finding_does_not_match() {
    let rules = file(vec![exception("no-unwrap", "src/a.rs", Some("expect"), "one")]);
    assert_eq!(reason_for(&rules, &finding("no-unwrap", "src/a.rs", "used `unwrap()` on a Result")), None);
}

#[test]
fn no_what_matches_any_finding_of_the_rule_at_the_path() {
    let rules = file(vec![exception("no-unwrap", "src/a.rs", None, "one")]);
    assert_eq!(reason_for(&rules, &finding("no-unwrap", "src/a.rs", "anything")), Some("one"));
}

#[test]
fn another_rule_does_not_match() {
    let rules = file(vec![exception("no-unwrap", "src/a.rs", None, "one")]);
    assert_eq!(reason_for(&rules, &finding("no-expect", "src/a.rs", "anything")), None);
}

#[test]
fn another_path_does_not_match() {
    let rules = file(vec![exception("no-unwrap", "src/a.rs", None, "one")]);
    assert_eq!(reason_for(&rules, &finding("no-unwrap", "src/b.rs", "anything")), None);
}

#[test]
fn the_first_of_two_matching_exceptions_wins() {
    let rules = file(vec![
        exception("no-unwrap", "src/a.rs", Some("other"), "skipped"),
        exception("no-unwrap", "src/a.rs", None, "first"),
        exception("no-unwrap", "src/a.rs", Some("unwrap"), "second"),
    ]);
    assert_eq!(reason_for(&rules, &finding("no-unwrap", "src/a.rs", "used unwrap")), Some("first"));
}

#[test]
fn no_exceptions_match_nothing() {
    assert_eq!(reason_for(&file(Vec::new()), &finding("no-unwrap", "src/a.rs", "x")), None);
}
