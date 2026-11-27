use crate::severity_name;
use rules::Severity;

#[test]
fn severity_names_match_rule_file_names() {
    // This list is by hand: add Severity::Watch when rules adds it (WO-rules-9, WO-report-5).
    for (severity, name) in [(Severity::Error, "error"), (Severity::Warning, "warning")] {
        assert_eq!(severity_name(severity), name, "{severity:?}");
        assert_eq!(Severity::from_name(&severity_name(severity)), Some(severity), "{severity:?}");
    }
}
