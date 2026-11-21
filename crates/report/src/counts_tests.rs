//! Unit tests for counts.rs: the exact JSON text, keys in field order; the default is every count at 0.

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
        not_applicable: 9,
        covers_nothing: 10,
        expired: 11,
    };
    assert_eq!(
        to_json(&counts),
        r#"{"findings":1,"errors":2,"warnings":3,"quality":4,"security":5,"silenced":6,"not_available":7,"failed":8,"not_applicable":9,"covers_nothing":10,"expired":11}"#
    );
}

#[test]
fn default_is_every_count_at_zero() {
    let zero = Counts {
        findings: 0,
        errors: 0,
        warnings: 0,
        quality: 0,
        security: 0,
        silenced: 0,
        not_available: 0,
        failed: 0,
        not_applicable: 0,
        covers_nothing: 0,
        expired: 0,
    };
    assert_eq!(Counts::default(), zero);
}
