use crate::{test_json::to_json, RecordFinding, RuleRecord, RuleState};
use rules::{Engine, RelPath};

#[test]
fn rule_records_serialize_in_field_order() {
    for (state, why, findings, json) in [
        (
            RuleState::Ran,
            None,
            vec![RecordFinding { path: RelPath::new("a.rs"), line: Some(3), what: "violation".to_string() }],
            r#"{"rule":"r","engine":"clippy","severity":"warning","state":"ran","why":null,"findings":[{"path":"a.rs","line":3,"what":"violation"}],"silenced":2}"#,
        ),
        (
            RuleState::NotAvailable,
            Some("missing".to_string()),
            vec![],
            r#"{"rule":"r","engine":"clippy","severity":"warning","state":"not_available","why":"missing","findings":[],"silenced":2}"#,
        ),
    ] {
        let record = RuleRecord { rule: "r".to_string(), engine: Engine::Clippy, severity: "warning".to_string(), state, why, findings, silenced: 2 };
        assert_eq!(to_json(&record), json, "{state:?}");
    }
}
