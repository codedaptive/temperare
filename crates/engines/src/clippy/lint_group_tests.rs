//! ZUT for lint_group.rs: checks, dashed and underscored, against a given group list.

use super::lint_group;

fn groups() -> Vec<String> {
    ["warnings", "nonstandard_style", "unused", "rust_2018_idioms", "clippy::all", "clippy::style", "clippy::new_group"]
        .map(String::from)
        .into()
}

#[test]
fn a_listed_group_is_a_group() {
    assert!(lint_group("clippy::style", &groups()));
    assert!(lint_group("unused", &groups()));
    assert!(lint_group("clippy::new_group", &groups()));
}

#[test]
fn a_dashed_check_is_read_with_underscores() {
    assert!(lint_group("nonstandard-style", &groups()));
    assert!(lint_group("rust-2018-idioms", &groups()));
    assert!(lint_group("clippy::new-group", &groups()));
}

#[test]
fn warnings_is_not_a_group_even_when_listed() {
    assert!(!lint_group("warnings", &groups()));
}

#[test]
fn a_check_not_in_the_list_is_not_a_group() {
    assert!(!lint_group("clippy::bool_comparison", &groups()));
    assert!(!lint_group("unused_variables", &groups()));
    assert!(!lint_group("clippy::pedantic", &groups()));
    assert!(!lint_group("unused", &[]));
}
