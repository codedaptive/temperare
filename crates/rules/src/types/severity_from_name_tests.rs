//! Unit tests for severity_from_name.rs: "error", "warning", and None for anything else.

use super::Severity;

#[test]
fn error() {
    assert_eq!(Severity::from_name("error"), Some(Severity::Error));
}

#[test]
fn warning() {
    assert_eq!(Severity::from_name("warning"), Some(Severity::Warning));
}

#[test]
fn unknown_names_are_none() {
    for name in ["info", "", "Error", "WARNING", "warn"] {
        assert_eq!(Severity::from_name(name), None, "{name:?}");
    }
}
