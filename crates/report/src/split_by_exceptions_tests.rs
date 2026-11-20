//! Unit tests for split_by_exceptions.rs.

use crate::split_by_exceptions;
use rules::{Date, Engine, Exception, Finding, Kind, RelPath, RuleFile, Severity};
use std::path::PathBuf;

fn exception(rule: &str, path: &str, what: Option<&str>, reason: &str) -> Exception {
    let mut exception = Exception::new(rule, path, reason, "maintainer, 2026-11-17");
    exception.what = what.map(str::to_string);
    exception
}

fn file(exceptions: Vec<Exception>) -> RuleFile {
    let mut rules = RuleFile::new(PathBuf::from("."));
    rules.exceptions = exceptions;
    rules
}

fn finding(rule: &str, path: &str, line: impl Into<Option<u32>>, what: &str) -> Finding {
    let grade = (Engine::Clippy, Kind::Quality, Severity::Error);
    Finding::new(rule, grade, RelPath::new(path), line, Some(9), what, "Return a typed error.")
}

fn day() -> Date {
    Date::new(2026, 11, 20).unwrap()
}

fn lines(findings: &[Finding]) -> Vec<Option<u32>> {
    findings.iter().map(Finding::line).collect()
}

fn silenced_lines_and_reasons(silenced: &[(Finding, Exception)]) -> Vec<(Option<u32>, &str)> {
    silenced.iter().map(|(f, e)| (f.line(), e.reason.as_str())).collect()
}

#[test]
fn a_what_substring_match_is_silenced_with_its_exception() {
    let rules = file(vec![exception("no-unwrap", "src/a.rs", Some("unwrap"), "startup only")]);
    let findings = vec![finding("no-unwrap", "src/a.rs", 3, "used `unwrap()` here")];
    let (kept, silenced) = split_by_exceptions(&rules, findings, day());
    assert!(kept.is_empty());
    assert_eq!(silenced_lines_and_reasons(&silenced), vec![(Some(3), "startup only")]);
    assert_eq!(silenced[0].1.what.as_deref(), Some("unwrap"));
}

#[test]
fn a_what_that_is_not_contained_keeps_the_finding() {
    let rules = file(vec![exception("no-unwrap", "src/a.rs", Some("expect"), "startup only")]);
    let findings = vec![finding("no-unwrap", "src/a.rs", 3, "used `unwrap()` here")];
    let (kept, silenced) = split_by_exceptions(&rules, findings, day());
    assert_eq!(lines(&kept), vec![Some(3)]);
    assert!(silenced.is_empty());
}

#[test]
fn no_what_silences_every_finding_of_the_rule_at_the_path() {
    let rules = file(vec![exception("no-unwrap", "src/a.rs", None, "whole file")]);
    let findings =
        vec![finding("no-unwrap", "src/a.rs", 3, "first message"), finding("no-unwrap", "src/a.rs", 8, "other text")];
    let (kept, silenced) = split_by_exceptions(&rules, findings, day());
    assert!(kept.is_empty());
    assert_eq!(silenced_lines_and_reasons(&silenced), vec![(Some(3), "whole file"), (Some(8), "whole file")]);
}

#[test]
fn another_path_and_another_rule_are_kept() {
    let rules = file(vec![exception("no-unwrap", "src/a.rs", None, "whole file")]);
    let findings = vec![finding("no-unwrap", "src/b.rs", 3, "x"), finding("no-expect", "src/a.rs", 4, "x")];
    let (kept, silenced) = split_by_exceptions(&rules, findings, day());
    assert_eq!(lines(&kept), vec![Some(3), Some(4)]);
    assert!(silenced.is_empty());
}

#[test]
fn mixed_input_keeps_the_input_order_in_both_lists() {
    let rules = file(vec![
        exception("no-unwrap", "src/a.rs", None, "unwraps"),
        exception("no-print", "src/b.rs", Some("println"), "prints"),
    ]);
    let findings = vec![
        finding("no-print", "src/b.rs", 1, "println! in library"),
        finding("no-unwrap", "src/c.rs", 2, "x"),
        finding("no-unwrap", "src/a.rs", 3, "x"),
        finding("no-print", "src/b.rs", 4, "eprintln! in library"),
        finding("no-print", "src/b.rs", 5, "dbg! in library"),
        finding("no-unwrap", "src/a.rs", 6, "y"),
    ];
    let (kept, silenced) = split_by_exceptions(&rules, findings, day());
    assert_eq!(lines(&kept), vec![Some(2), Some(5)]);
    let expected = vec![(Some(1), "prints"), (Some(3), "unwraps"), (Some(4), "prints"), (Some(6), "unwraps")];
    assert_eq!(silenced_lines_and_reasons(&silenced), expected);
}

#[test]
fn no_exceptions_keep_everything() {
    let findings = vec![finding("no-unwrap", "src/a.rs", 7, "x"), finding("no-print", "src/b.rs", 2, "y")];
    let (kept, silenced) = split_by_exceptions(&file(Vec::new()), findings, day());
    assert_eq!(lines(&kept), vec![Some(7), Some(2)]);
    assert!(silenced.is_empty());
}
