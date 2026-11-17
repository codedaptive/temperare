//! ZUT for report.rs: the exact JSON text of an empty report, with and without a base.

use crate::test_json::to_json;
use crate::{Counts, Report};
use procio::Oid;

const COUNTS: &str = r#"{"findings":0,"errors":0,"warnings":0,"quality":0,"security":0,"silenced":0,"not_available":0,"failed":0}"#;
const HEX: &str = "0123456789abcdef0123456789abcdef01234567";

fn empty(base: Option<Oid>) -> Report {
    Report {
        findings: Vec::new(),
        silenced: Vec::new(),
        exceptions_in_force: 0,
        not_available: Vec::new(),
        failed: Vec::new(),
        base,
        counts: Counts {
            findings: 0,
            errors: 0,
            warnings: 0,
            quality: 0,
            security: 0,
            silenced: 0,
            not_available: 0,
            failed: 0,
        },
    }
}

#[test]
fn no_base_leaves_base_out_and_keeps_exceptions_in_force_at_zero() {
    let expected = format!(
        r#"{{"findings":[],"silenced":[],"exceptions_in_force":0,"not_available":[],"failed":[],"counts":{COUNTS}}}"#
    );
    assert_eq!(to_json(&empty(None)), expected);
}

#[test]
fn a_base_serializes_as_its_hex_text() {
    let expected = format!(
        r#"{{"findings":[],"silenced":[],"exceptions_in_force":0,"not_available":[],"failed":[],"base":"{HEX}","counts":{COUNTS}}}"#
    );
    assert_eq!(to_json(&empty(Oid::parse(HEX))), expected);
}

#[test]
fn exceptions_in_force_carries_its_count() {
    let report = Report { exceptions_in_force: 3, ..empty(None) };
    assert!(to_json(&report).contains(r#""exceptions_in_force":3,"#));
}
