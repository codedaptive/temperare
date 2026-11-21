//! Unit tests for ratio_hit.rs.

use super::{ratio_hit, Counts, TestRatio};
use crate::Hit;
use rules::{RelPath, Scope};

fn ratio(max_ratio: f64, floor: u32) -> TestRatio {
    TestRatio { max_ratio, floor, test_paths: Scope::build(&["**/tests/**".to_string()], &[], None).unwrap() }
}

fn counts(test: u32, code: u32) -> Counts {
    Counts { test, code, top_test: Some((RelPath::new("tests/big.rs"), 12)) }
}

fn hit(what: &str) -> Option<Hit> {
    Some(Hit { path: RelPath::new("tests/big.rs"), line: Some(12), col: None, what: what.to_string() })
}

#[test]
fn reports_a_change_over_the_ratio() {
    let cases = [
        (
            counts(80, 10),
            ratio(1.5, 40),
            hit("this change adds 80 lines of test for 10 lines of code (8.0 to 1; the rule's limit is 1.5 past 40 lines)"),
        ),
        (
            counts(10, 4),
            ratio(1.5, 0),
            hit("this change adds 10 lines of test for 4 lines of code (2.5 to 1; the rule's limit is 1.5 past 0 lines)"),
        ),
        (
            counts(5, 3),
            ratio(1.5, 0),
            hit("this change adds 5 lines of test for 3 lines of code (1.7 to 1; the rule's limit is 1.5 past 0 lines)"),
        ),
        (
            counts(5, 4),
            ratio(1.2, 0),
            hit("this change adds 5 lines of test for 4 lines of code (1.3 to 1; the rule's limit is 1.2 past 0 lines)"),
        ),
        (
            counts(1, 4),
            ratio(0.1, 0),
            hit("this change adds 1 lines of test for 4 lines of code (0.3 to 1; the rule's limit is 0.1 past 0 lines)"),
        ),
        (counts(1, 6), ratio(1.5, 0), None),
        (counts(15, 10), ratio(1.5, 0), None),
        (counts(39, 1), ratio(1.5, 40), None),
        (
            counts(40, 1),
            ratio(1.5, 40),
            hit("this change adds 40 lines of test for 1 lines of code (40.0 to 1; the rule's limit is 1.5 past 40 lines)"),
        ),
        (
            counts(45, 0),
            ratio(2.0, 40),
            hit("this change adds 45 lines of test for 0 lines of code (test only; the rule's limit is 2 past 40 lines)"),
        ),
        (counts(63, 90), ratio(0.7, 0), None),
        (
            counts(64, 90),
            ratio(0.7, 0),
            hit("this change adds 64 lines of test for 90 lines of code (0.7 to 1; the rule's limit is 0.7 past 0 lines)"),
        ),
        (
            counts(16, 10),
            ratio(1.5, 0),
            hit("this change adds 16 lines of test for 10 lines of code (1.6 to 1; the rule's limit is 1.5 past 0 lines)"),
        ),
        (counts(1333, 1000), ratio(1.333, 0), None),
        (
            counts(1334, 1000),
            ratio(1.333, 0),
            hit("this change adds 1334 lines of test for 1000 lines of code (1.3 to 1; the rule's limit is 1.333 past 0 lines)"),
        ),
        (counts(1, 10), ratio(0.1, 0), None),
        (
            counts(2, 10),
            ratio(0.1, 0),
            hit("this change adds 2 lines of test for 10 lines of code (0.2 to 1; the rule's limit is 0.1 past 0 lines)"),
        ),
        (Counts { test: 0, code: 0, top_test: None }, ratio(1.5, 0), None),
        (Counts { test: 50, code: 1, top_test: None }, ratio(1.5, 40), None),
    ];
    for (given, ratio, want) in cases {
        assert_eq!(ratio_hit(&given, &ratio), want, "{given:?}");
    }
}
