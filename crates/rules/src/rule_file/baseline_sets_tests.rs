//! Unit tests for baseline_sets.rs.

use super::baseline_set::BaselineSet;
use super::baseline_sets::{baseline_sets, baseline_sets_in};

/// A fixture set with no rules: only its name, version and ready mark are read.
const fn fixture(name: &'static str, version: u32, ready: bool) -> BaselineSet {
    BaselineSet { name, version, ready, text: "version = 1\n", yaml: &[], cases: &[] }
}

/// Out of name order, so a sorted list differs from the held order.
const FIXTURE: &[BaselineSet] = &[fixture("zeta", 2, false), fixture("alpha", 3, true)];

#[test]
fn baseline_sets_lists_each_set_in_held_order_with_its_ready_mark() {
    let cases: Vec<(&str, Vec<(String, bool)>, Vec<(String, bool)>)> = vec![
        ("built into cctao", baseline_sets(), vec![("house@1".to_string(), true), ("house@2".to_string(), true), ("standard@1".to_string(), true)]),
        ("fixture", baseline_sets_in(FIXTURE), vec![("zeta@2".to_string(), false), ("alpha@3".to_string(), true)]),
    ];
    for (case, actual, expected) in cases {
        assert_eq!(actual, expected, "case: {case}");
    }
}
