//! Unit tests for rule_cases.rs: RuleCases equals another only when its failing files, passing files and expected finding
//! all match, the files in the same order.

use super::{CaseFile, ExpectedFinding, RuleCases};

fn file(path: &str, text: &'static str) -> CaseFile {
    CaseFile { path: path.to_string(), text }
}

fn cases() -> RuleCases {
    RuleCases {
        failing: vec![file("src/lib.rs", "fn f() { x.unwrap(); }\n"), file("Cargo.toml", "[package]\n")],
        passing: vec![file("src/lib.rs", "fn f() { x?; }\n")],
        expected: ExpectedFinding { rule_id: "tm-no-unwrap".to_string(), path: "src/lib.rs".to_string(), line: 1 },
    }
}

#[test]
fn a_clone_equals_the_original() {
    assert_eq!(
        cases().clone(),
        RuleCases {
            failing: vec![
                CaseFile { path: "src/lib.rs".to_string(), text: "fn f() { x.unwrap(); }\n" },
                CaseFile { path: "Cargo.toml".to_string(), text: "[package]\n" },
            ],
            passing: vec![CaseFile { path: "src/lib.rs".to_string(), text: "fn f() { x?; }\n" }],
            expected: ExpectedFinding {
                rule_id: "tm-no-unwrap".to_string(),
                path: "src/lib.rs".to_string(),
                line: 1,
            },
        }
    );
}

#[test]
fn different_failing_files_are_different_cases() {
    let other = RuleCases { failing: vec![file("src/lib.rs", "fn f() { x.unwrap(); }\n")], ..cases() };
    assert_ne!(cases(), other);
}

#[test]
fn failing_files_in_another_order_are_different_cases() {
    let other = RuleCases {
        failing: vec![file("Cargo.toml", "[package]\n"), file("src/lib.rs", "fn f() { x.unwrap(); }\n")],
        ..cases()
    };
    assert_ne!(cases(), other);
}

#[test]
fn different_passing_files_are_different_cases() {
    let other = RuleCases { passing: vec![file("src/lib.rs", "fn f() {}\n")], ..cases() };
    assert_ne!(cases(), other);
}

#[test]
fn a_different_expected_finding_is_a_different_case() {
    let other = RuleCases {
        expected: ExpectedFinding { rule_id: "tm-no-unwrap".to_string(), path: "src/lib.rs".to_string(), line: 2 },
        ..cases()
    };
    assert_ne!(cases(), other);
}
