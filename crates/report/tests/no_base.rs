//! Integration test of build_report's no-base path. It is here, not with the unit tests, because build_report reads
//! the clock for today's UTC day.

mod common;

use report::build_report;
use rules::{Engine, Finding, Kind, RelPath, RuleFile, RuleOutcome, Severity};
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn finding(path: &str, line: impl Into<Option<u32>>) -> Finding {
    Finding::new("r", (Engine::Clippy, Kind::Quality, Severity::Warning), RelPath::new(path), line, None, "w", "f")
}

#[test]
fn no_base_report_has_a_parseable_date() {
    let rules = RuleFile::new(PathBuf::from("/no/such/folder/for/report/tests"));
    let before = SystemTime::now();
    let report = build_report(&rules, vec![], None).unwrap();
    let after = SystemTime::now();

    assert_eq!(report.date.len(), 10);
    assert!(rules::Date::parse(&report.date).is_some());
    assert!(report.date == common::utc_date(before) || report.date == common::utc_date(after), "{}", report.date);
}

#[test]
fn utc_date_helper_matches_known_dates() {
    for (seconds, expected) in [
        (0, "1970-01-01"),
        (951_782_400, "2000-02-29"),
        (1_735_689_599, "2024-12-31"),
        (1_735_689_600, "2025-01-01"),
    ] {
        assert_eq!(common::utc_date(UNIX_EPOCH + Duration::from_secs(seconds)), expected);
    }
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
