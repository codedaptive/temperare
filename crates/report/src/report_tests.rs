//! Unit tests for report.rs: the exact JSON text of an empty report, with and without a base, and of the not-applicable
//! rules, date, per-rule silenced counts and engines; the default is the empty report with no base and no engines.

use crate::test_json::to_json;
use crate::{Counts, EngineVersion, NotApplicable, Report, RuleCount};
use procio::Oid;
use rules::Engine;

const COUNTS: &str = r#"{"findings":0,"errors":0,"warnings":0,"quality":0,"security":0,"silenced":0,"not_available":0,"failed":0,"not_applicable":0}"#;
const HEX: &str = "0123456789abcdef0123456789abcdef01234567";

fn empty(base: Option<Oid>) -> Report {
    Report {
        findings: Vec::new(),
        silenced: Vec::new(),
        exceptions_in_force: 0,
        not_available: Vec::new(),
        failed: Vec::new(),
        base,
        not_applicable: Vec::new(),
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
        },
    }
}

#[test]
fn no_base_leaves_base_out_and_keeps_exceptions_in_force_at_zero() {
    let expected = format!(
        r#"{{"findings":[],"silenced":[],"exceptions_in_force":0,"not_available":[],"failed":[],"not_applicable":[],"date":"","silenced_by_rule":[],"engines":[],"counts":{COUNTS}}}"#
    );
    assert_eq!(to_json(&empty(None)), expected);
}

#[test]
fn a_base_serializes_as_its_hex_text() {
    let expected = format!(
        r#"{{"findings":[],"silenced":[],"exceptions_in_force":0,"not_available":[],"failed":[],"base":"{HEX}","not_applicable":[],"date":"","silenced_by_rule":[],"engines":[],"counts":{COUNTS}}}"#
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
        r#"{{"findings":[],"silenced":[],"exceptions_in_force":0,"not_available":[],"failed":[],"not_applicable":[],"date":"2026-11-20","silenced_by_rule":[{{"rule":"b","count":2}},{{"rule":"a","count":1}}],"engines":[],"counts":{COUNTS}}}"#
    );
    assert_eq!(to_json(&report), expected);
}

#[test]
fn default_is_the_empty_report_with_no_base() {
    assert_eq!(Report::default(), empty(None));
    let expected = format!(
        r#"{{"findings":[],"silenced":[],"exceptions_in_force":0,"not_available":[],"failed":[],"not_applicable":[],"date":"","silenced_by_rule":[],"engines":[],"counts":{COUNTS}}}"#
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
        r#"{{"findings":[],"silenced":[],"exceptions_in_force":0,"not_available":[],"failed":[],"not_applicable":[],"date":"","silenced_by_rule":[],"engines":[{{"engine":"ruff","version":"0.6.0"}},{{"engine":"clippy","version":null}}],"counts":{COUNTS}}}"#
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
        r#"{{"findings":[],"silenced":[],"exceptions_in_force":0,"not_available":[],"failed":[],"base":"{HEX}","not_applicable":[{{"rule":"swift-x","engine":"swiftlint","why":"no reason given"}}],"date":"2026-11-20","silenced_by_rule":[],"engines":[],"counts":{COUNTS}}}"#
    );
    assert_eq!(to_json(&report), expected);
}
