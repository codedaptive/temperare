//! ZUT for severity_from_name.rs: "error", "warning", "watch", and None for anything else.

use super::Severity;

#[test]
fn each_rule_file_name_is_its_severity() {
    let rows = [("error", Severity::Error), ("warning", Severity::Warning), ("watch", Severity::Watch)];
    for (name, expected) in rows {
        assert_eq!(Severity::from_name(name), Some(expected), "{name:?}");
    }
}

#[test]
fn unknown_names_are_none() {
    for name in ["info", "", "Error", "WARNING", "warn", "Watch", "watching"] {
        assert_eq!(Severity::from_name(name), None, "{name:?}");
    }
}
