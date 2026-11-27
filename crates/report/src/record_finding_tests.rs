use crate::{test_json::to_json, RecordFinding};
use rules::RelPath;

#[test]
fn record_findings_serialize_in_field_order() {
    for (line, json) in [
        (Some(7), r#"{"path":"src/a.rs","line":7,"what":"violation"}"#),
        (None, r#"{"path":"src/a.rs","line":null,"what":"violation"}"#),
    ] {
        let finding = RecordFinding { path: RelPath::new("src/a.rs"), line, what: "violation".to_string() };
        assert_eq!(to_json(&finding), json, "{line:?}");
    }
}
