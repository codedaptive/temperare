//! Unit tests for check_json.rs: the report's top-level keys plus `exit` and `next`, and a finding's fields carried through.

use super::check_json::check_json;
use report::{Counts, Report};
use rules::{Engine, Finding, Kind, RelPath, Severity};
use std::collections::BTreeSet;

fn report(base: Option<procio::Oid>) -> Report {
    Report {
        findings: vec![Finding {
            rule: "no-print".to_string(),
            engine: Engine::Ruff,
            kind: Kind::Quality,
            severity: Severity::Error,
            owner: None,
            path: RelPath::new("a.py"),
            line: 7,
            col: Some(3),
            what: "print found".to_string(),
            fix: "use logging".to_string(),
        }],
        silenced: vec![],
        exceptions_in_force: 0,
        not_available: vec![],
        failed: vec![],
        base,
        counts: Counts {
            findings: 1,
            errors: 1,
            warnings: 0,
            quality: 1,
            security: 0,
            silenced: 0,
            not_available: 0,
            failed: 0,
        },
    }
}

fn keys(value: &serde_json::Value) -> BTreeSet<String> {
    value.as_object().map(|o| o.keys().cloned().collect()).unwrap_or_default()
}

fn expected_keys(with_base: bool) -> BTreeSet<String> {
    let mut names =
        vec!["findings", "silenced", "exceptions_in_force", "not_available", "failed", "counts", "exit", "next"];
    if with_base {
        names.push("base");
    }
    names.into_iter().map(str::to_string).collect()
}

#[test]
fn without_a_base_the_keys_are_the_report_fields_exit_and_next() {
    assert_eq!(keys(&check_json(&report(None), 1, "fix them")), expected_keys(false));
}

#[test]
fn with_a_base_the_keys_add_base_as_its_hex_text() {
    let oid = procio::Oid::parse("0123456789abcdef0123456789abcdef01234567");
    let value = check_json(&report(oid), 1, "fix them");
    assert_eq!(keys(&value), expected_keys(true));
    assert_eq!(value["base"], "0123456789abcdef0123456789abcdef01234567");
}

#[test]
fn exit_and_next_carry_the_values_given() {
    let value = check_json(&report(None), 3, "install swiftlint (or run without those rules)");
    assert_eq!(value["exit"], 3);
    assert_eq!(value["next"], "install swiftlint (or run without those rules)");
}

#[test]
fn a_findings_fields_come_through() {
    let value = check_json(&report(None), 1, "fix them");
    let f = &value["findings"][0];
    assert_eq!(f["rule"], "no-print");
    assert_eq!(f["path"], "a.py");
    assert_eq!(f["line"], 7);
    assert_eq!(f["fix"], "use logging");
}
