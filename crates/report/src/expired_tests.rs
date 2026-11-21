//! Unit tests for expired.rs: the exact JSON text, keys in field order, `what` as its text or null.

use crate::test_json::to_json;
use crate::Expired;
use rules::RelPath;

#[test]
fn serializes_every_field_in_order_with_a_missing_what_as_null() {
    let rows = [
        (
            Some("unwrap"),
            r#"{"rule":"no-unwrap","path":"src/a.rs","what":"unwrap","expires":"2026-11-30","reason":"startup only","approved":"maintainer, 2026-11-17"}"#,
        ),
        (
            None,
            r#"{"rule":"no-unwrap","path":"src/a.rs","what":null,"expires":"2026-11-30","reason":"startup only","approved":"maintainer, 2026-11-17"}"#,
        ),
    ];
    for (what, expected) in rows {
        let expired = Expired {
            rule: "no-unwrap".to_string(),
            path: RelPath::new("src/a.rs"),
            what: what.map(str::to_string),
            expires: "2026-11-30".to_string(),
            reason: "startup only".to_string(),
            approved: "maintainer, 2026-11-17".to_string(),
        };
        assert_eq!(to_json(&expired), expected, "{what:?}");
    }
}
