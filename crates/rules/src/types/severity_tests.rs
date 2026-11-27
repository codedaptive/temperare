//! ZUT for severity.rs: each variant serializes to its rule-file name and reads back from it.

use super::Severity;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Wrap {
    severity: Severity,
}

#[test]
fn each_variant_round_trips_through_its_rule_file_name() {
    let rows = [
        (Severity::Error, "severity = \"error\"\n"),
        (Severity::Warning, "severity = \"warning\"\n"),
        (Severity::Watch, "severity = \"watch\"\n"),
    ];
    for (severity, text) in rows {
        assert_eq!(toml::to_string(&Wrap { severity }).unwrap(), text, "{severity:?} serialized");
        assert_eq!(toml::from_str::<Wrap>(text).unwrap(), Wrap { severity }, "{text:?} deserialized");
    }
}
