//! Unit tests for report.rs: the exact JSON text of an empty report, with and without a base, and of the rules that
//! cover nothing, the not-applicable rules, the expired exceptions, date, per-rule silenced counts and engines; the
//! default is the empty report with no base and no engines.

use crate::test_json::to_json;
use crate::{Counts, CoversNothing, EngineVersion, Expired, NotApplicable, Report, RuleCount};
use procio::Oid;
use rules::{Engine, RelPath};

const COUNTS: &str = r#"{"findings":0,"errors":0,"warnings":0,"quality":0,"security":0,"silenced":0,"not_available":0,"failed":0,"not_applicable":0,"covers_nothing":0,"expired":0}"#;
const HEX: &str = "0123456789abcdef0123456789abcdef01234567";

fn empty(base: Option<Oid>) -> Report {
    Report {
        findings: Vec::new(),
        silenced: Vec::new(),
        exceptions_in_force: 0,
        not_available: Vec::new(),
        failed: Vec::new(),
        base,
        covers_nothing: Vec::new(),
        not_applicable: Vec::new(),
        expired: Vec::new(),
        date: String::new(),
        silenced_by_rule: Vec::new(),
        engines: Vec::new(),
        counts: Counts {
            findings: 0,
            errors: 0,
            warnings: 0,
            quality: 0,
            security: 0,
            silenced: 0,
            not_available: 0,
            failed: 0,
            not_applicable: 0,
            covers_nothing: 0,
            expired: 0,
        },
    }
}

#[test]
fn no_base_leaves_base_out_and_keeps_exceptions_in_force_at_zero() {
    let expected = format!(
        r#"{{"findings":[],"silenced":[],"exceptions_in_force":0,"not_available":[],"failed":[],"covers_nothing":[],"not_applicable":[],"expired":[],"date":"","silenced_by_rule":[],"engines":[],"counts":{COUNTS}}}"#
    );
    assert_eq!(to_json(&empty(None)), expected);
}

#[test]
fn a_base_serializes_as_its_hex_text() {
    let expected = format!(
        r#"{{"findings":[],"silenced":[],"exceptions_in_force":0,"not_available":[],"failed":[],"base":"{HEX}","covers_nothing":[],"not_applicable":[],"expired":[],"date":"","silenced_by_rule":[],"engines":[],"counts":{COUNTS}}}"#
    );
    assert_eq!(to_json(&empty(Oid::parse(HEX))), expected);
}

#[test]
fn exceptions_in_force_carries_its_count() {
    let report = Report { exceptions_in_force: 3, ..empty(None) };
    assert!(to_json(&report).contains(r#""exceptions_in_force":3,"#));
}

#[test]
fn the_date_and_silenced_by_rule_serialize_in_their_order_before_counts() {
    let report = Report {
        date: "2026-11-20".to_string(),
        silenced_by_rule: vec![
            RuleCount { rule: "b".to_string(), count: 2 },
            RuleCount { rule: "a".to_string(), count: 1 },
        ],
        ..empty(None)
    };
    let expected = format!(
        r#"{{"findings":[],"silenced":[],"exceptions_in_force":0,"not_available":[],"failed":[],"covers_nothing":[],"not_applicable":[],"expired":[],"date":"2026-11-20","silenced_by_rule":[{{"rule":"b","count":2}},{{"rule":"a","count":1}}],"engines":[],"counts":{COUNTS}}}"#
    );
    assert_eq!(to_json(&report), expected);
}

#[test]
fn default_is_the_empty_report_with_no_base() {
    assert_eq!(Report::default(), empty(None));
    let expected = format!(
        r#"{{"findings":[],"silenced":[],"exceptions_in_force":0,"not_available":[],"failed":[],"covers_nothing":[],"not_applicable":[],"expired":[],"date":"","silenced_by_rule":[],"engines":[],"counts":{COUNTS}}}"#
    );
    assert_eq!(to_json(&Report::default()), expected);
}

#[test]
fn the_default_has_no_engines_and_serializes_them_as_an_empty_list() {
    let report = Report::default();
    assert!(report.engines.is_empty());
    assert!(to_json(&report).contains(r#","silenced_by_rule":[],"engines":[],"counts":"#));
}

#[test]
fn engines_serialize_in_the_order_given_with_a_missing_version_as_null() {
    let report = Report {
        engines: vec![
            EngineVersion { engine: Engine::Ruff, version: Some("0.6.0".to_string()) },
            EngineVersion { engine: Engine::Clippy, version: None },
        ],
        ..empty(None)
    };
    let expected = format!(
        r#"{{"findings":[],"silenced":[],"exceptions_in_force":0,"not_available":[],"failed":[],"covers_nothing":[],"not_applicable":[],"expired":[],"date":"","silenced_by_rule":[],"engines":[{{"engine":"ruff","version":"0.6.0"}},{{"engine":"clippy","version":null}}],"counts":{COUNTS}}}"#
    );
    assert_eq!(to_json(&report), expected);
}

#[test]
fn not_applicable_serializes_after_base_and_before_the_date() {
    let report = Report {
        not_applicable: vec![NotApplicable {
            rule: "swift-x".to_string(),
            engine: Engine::SwiftLint,
            why: "no reason given".to_string(),
        }],
        date: "2026-11-20".to_string(),
        ..empty(Oid::parse(HEX))
    };
    let expected = format!(
        r#"{{"findings":[],"silenced":[],"exceptions_in_force":0,"not_available":[],"failed":[],"base":"{HEX}","covers_nothing":[],"not_applicable":[{{"rule":"swift-x","engine":"swiftlint","why":"no reason given"}}],"expired":[],"date":"2026-11-20","silenced_by_rule":[],"engines":[],"counts":{COUNTS}}}"#
    );
    assert_eq!(to_json(&report), expected);
}

#[test]
fn covers_nothing_serializes_after_base_and_before_not_applicable() {
    let report = Report {
        covers_nothing: vec![CoversNothing { rule: "own-x".to_string(), engine: Engine::SwiftLint }],
        not_applicable: vec![NotApplicable {
            rule: "tm-y".to_string(),
            engine: Engine::Ruff,
            why: "no file its engine checks".to_string(),
        }],
        ..empty(Oid::parse(HEX))
    };
    let expected = format!(
        r#"{{"findings":[],"silenced":[],"exceptions_in_force":0,"not_available":[],"failed":[],"base":"{HEX}","covers_nothing":[{{"rule":"own-x","engine":"swiftlint"}}],"not_applicable":[{{"rule":"tm-y","engine":"ruff","why":"no file its engine checks"}}],"expired":[],"date":"","silenced_by_rule":[],"engines":[],"counts":{COUNTS}}}"#
    );
    assert_eq!(to_json(&report), expected);
}

#[test]
fn expired_serializes_after_not_applicable_and_before_the_date() {
    let report = Report {
        not_applicable: vec![NotApplicable {
            rule: "swift-x".to_string(),
            engine: Engine::SwiftLint,
            why: "no reason given".to_string(),
        }],
        expired: vec![Expired {
            rule: "no-unwrap".to_string(),
            path: RelPath::new("src/a.rs"),
            what: None,
            expires: "2026-11-19".to_string(),
            reason: "startup only".to_string(),
            approved: "maintainer, 2026-11-17".to_string(),
        }],
        date: "2026-11-20".to_string(),
        ..empty(None)
    };
    let expected = format!(
        r#"{{"findings":[],"silenced":[],"exceptions_in_force":0,"not_available":[],"failed":[],"covers_nothing":[],"not_applicable":[{{"rule":"swift-x","engine":"swiftlint","why":"no reason given"}}],"expired":[{{"rule":"no-unwrap","path":"src/a.rs","what":null,"expires":"2026-11-19","reason":"startup only","approved":"maintainer, 2026-11-17"}}],"date":"2026-11-20","silenced_by_rule":[],"engines":[],"counts":{COUNTS}}}"#
    );
    assert_eq!(to_json(&report), expected);
}
