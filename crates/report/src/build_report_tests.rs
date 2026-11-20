//! Unit tests for build_report.rs: the no-base path, which runs no program and reads no file (it reads only the clock,
//! for today's UTC day). The base path is proved by the integration test (tests/new_only.rs).

use crate::build_report;
use rules::{Engine, Finding, Kind, RelPath, RuleFile, RuleOutcome, Severity};
use std::path::PathBuf;

fn finding(path: &str, line: impl Into<Option<u32>>) -> Finding {
    Finding::new("r", (Engine::Clippy, Kind::Quality, Severity::Warning), RelPath::new(path), line, None, "w", "f")
}

#[test]
fn no_base_keeps_every_finding_without_touching_the_folder() {
    let rules = RuleFile::new(PathBuf::from("/no/such/folder/for/report/tests"));
    let outcomes = vec![("r".to_string(), RuleOutcome::Findings(vec![finding("b.rs", 1), finding("a.rs", 5)]))];
    let report = build_report(&rules, outcomes, None).unwrap();
    let places: Vec<_> = report.findings.iter().map(|f| (f.path.as_str(), f.line())).collect();
    assert_eq!(places, vec![("a.rs", Some(5)), ("b.rs", Some(1))]);
    assert_eq!((report.counts.findings, report.counts.warnings), (2, 2));
    assert_eq!(report.base, None);
}
