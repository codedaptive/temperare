//! Unit tests for expected_finding.rs: an ExpectedFinding equals another only when its rule id, path and line all match.

use super::ExpectedFinding;

fn expected(rule_id: &str, path: &str, line: usize) -> ExpectedFinding {
    ExpectedFinding { rule_id: rule_id.to_string(), path: path.to_string(), line }
}

#[test]
fn a_clone_equals_the_original() {
    let original = expected("tm-no-unwrap", "src/lib.rs", 3);
    assert_eq!(
        original.clone(),
        ExpectedFinding { rule_id: "tm-no-unwrap".to_string(), path: "src/lib.rs".to_string(), line: 3 }
    );
}

#[test]
fn a_different_rule_id_is_a_different_finding() {
    assert_ne!(expected("tm-no-unwrap", "src/lib.rs", 3), expected("tm-no-panic", "src/lib.rs", 3));
}

#[test]
fn a_different_path_is_a_different_finding() {
    assert_ne!(expected("tm-no-unwrap", "src/lib.rs", 3), expected("tm-no-unwrap", "src/main.rs", 3));
}

#[test]
fn a_different_line_is_a_different_finding() {
    assert_ne!(expected("tm-no-unwrap", "src/lib.rs", 3), expected("tm-no-unwrap", "src/lib.rs", 4));
}
