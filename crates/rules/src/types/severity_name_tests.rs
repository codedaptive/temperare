//! ZUT for severity_name.rs: each severity's rule-file name.

use super::Severity;

#[test]
fn each_severity_has_its_rule_file_name() {
    for (severity, name) in [(Severity::Error, "error"), (Severity::Warning, "warning"), (Severity::Watch, "watch")] {
        assert_eq!(severity.name(), name, "{severity:?}, {name:?}");
    }
}
