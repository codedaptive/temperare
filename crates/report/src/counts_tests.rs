//! Unit tests for counts.rs: the exact JSON text, keys in field order.

use crate::test_json::to_json;
use crate::Counts;

#[test]
fn serializes_every_count_in_order() {
    let counts = Counts {
        findings: 1,
        errors: 2,
        warnings: 3,
        quality: 4,
        security: 5,
        silenced: 6,
        not_available: 7,
        failed: 8,
    };
    assert_eq!(
        to_json(&counts),
        r#"{"findings":1,"errors":2,"warnings":3,"quality":4,"security":5,"silenced":6,"not_available":7,"failed":8}"#
    );
}
