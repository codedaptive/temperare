//! ZUT for check_json.rs: the required top-level keys (the report's fields, `exit`, `next`; `base` only with a base), and a finding's fields carried through.

use super::check_json::check_json;
use report::{Counts, Report};
use rules::{Engine, Finding, Kind, RelPath, Severity};

fn report(base: Option<procio::Oid>) -> Report {
    Report {
        findings: vec![Finding::new(
            "no-print",
            (Engine::Ruff, Kind::Quality, Severity::Error),
            RelPath::new("a.py"),
            7,
            Some(3),
            "print found",
            "use logging",
        )],
        base,
        counts: Counts { findings: 1, errors: 1, quality: 1, ..Default::default() },
        ..Default::default()
    }
}

fn assert_required_keys(value: &serde_json::Value) {
    for key in ["findings", "silenced", "exceptions_in_force", "not_available", "failed", "counts", "exit", "next"] {
        assert!(value.get(key).is_some(), "missing {key}");
    }
}

#[test]
fn without_a_base_required_fields_are_present_and_base_is_absent() {
    let value = check_json(&report(None), 1, "fix them");
    assert_required_keys(&value);
    assert!(value.get("base").is_none());
}

#[test]
fn with_a_base_required_fields_include_its_hex_text() {
    let oid = procio::Oid::parse("0123456789abcdef0123456789abcdef01234567");
    let value = check_json(&report(oid), 1, "fix them");
    assert_required_keys(&value);
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
