//! ZUT for known_lint.rs.

use super::known_lint;

fn lints() -> Vec<String> {
    vec!["unused_variables".to_string(), "clippy::bool_comparison".to_string()]
}

#[test]
fn a_listed_lint_is_known() {
    assert!(known_lint("clippy::bool_comparison", &lints()));
    assert!(known_lint("unused_variables", &lints()));
}

#[test]
fn a_dashed_name_is_read_with_underscores() {
    assert!(known_lint("clippy::bool-comparison", &lints()));
    assert!(known_lint("unused-variables", &lints()));
}

#[test]
fn warnings_is_known_though_not_listed() {
    assert!(known_lint("warnings", &lints()));
    assert!(known_lint("warnings", &[]));
}

#[test]
fn an_unlisted_name_is_not_known() {
    assert!(!known_lint("clippy::bool_comparisn", &lints()));
    assert!(!known_lint("clippy::stutter", &lints()));
    assert!(!known_lint("rustdoc::broken_intra_doc_links", &lints()));
}
