//! Unit tests for severity.rs: each variant serializes to its rule-file name.

use super::Severity;
use serde::Serialize;

#[derive(Serialize)]
struct Wrap {
    severity: Severity,
}

#[test]
fn error_serializes_as_error() {
    assert_eq!(toml::to_string(&Wrap { severity: Severity::Error }).unwrap(), "severity = \"error\"\n");
}

#[test]
fn warning_serializes_as_warning() {
    assert_eq!(toml::to_string(&Wrap { severity: Severity::Warning }).unwrap(), "severity = \"warning\"\n");
}
