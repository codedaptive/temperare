//! Integration tests of the no-base path of build_report and build_report_with_no_files: they read the clock for
//! today's UTC day, so they are here, not with the unit tests. utc_date_helper_matches_known_dates tests the
//! test-support helper in tests/common and reads no clock.

mod common;

use report::{build_report, build_report_with_no_files, CoversNothing, NotApplicable};
use rules::{Engine, Finding, Kind, RelPath, Rule, RuleFile, RuleOutcome, Severity};
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

#[test]
fn a_flagged_rule_is_listed_by_whose_it_is() {
    let rows = [
        (
            "own-x",
            Engine::SwiftLint,
            vec![CoversNothing { rule: "own-x".to_string(), engine: Engine::SwiftLint }],
            vec![],
            (1, 0),
        ),
        (
            "tm-y",
            Engine::Ruff,
            vec![],
            vec![NotApplicable {
                rule: "tm-y".to_string(),
                engine: Engine::Ruff,
                why: "its paths match no file in the rule file's folder".to_string(),
            }],
            (0, 1),
        ),
    ];
    for (id, engine, covers_nothing, not_applicable, (covering, applying)) in rows {
        let mut rules = RuleFile::new(PathBuf::from("."));
        rules.rules = vec![Rule::new(id, engine, "lint")];
        let outcomes = vec![(id.to_string(), RuleOutcome::Findings(Vec::new()))];
        let no_files = vec![(id.to_string(), "its paths match no file in the rule file's folder".to_string())];
        let report = build_report_with_no_files(&rules, outcomes, None, no_files).unwrap();
        assert_eq!(report.covers_nothing, covers_nothing, "{id}");
        assert_eq!(report.not_applicable, not_applicable, "{id}");
        let counts = (report.counts.covers_nothing, report.counts.not_applicable, report.counts.findings);
        assert_eq!(counts, (covering, applying, 0), "{id}");
    }
}
