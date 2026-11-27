//! Unit tests for ruff_refusal.rs: which checks are refused and the reason given, the cases written out from the contract.

use super::ruff_refusal;

#[test]
fn codes_prefixes_and_all_are_not_refused() {
    for check in ["E", "E7", "PLC0414", "C90", "ALL", "ALL,E7", "E7,F4"] {
        assert_eq!(ruff_refusal(check), None, "{check}");
    }
}

#[test]
fn an_empty_check_is_refused() {
    assert_eq!(
        ruff_refusal(""),
        Some("ruff check ``: `` is not a rule code, a code prefix or ALL, so cctao cannot tell which findings it selects".to_string())
    );
}

#[test]
fn lowercase_is_refused() {
    assert_eq!(
        ruff_refusal("e7"),
        Some("ruff check `e7`: `e7` is not a rule code, a code prefix or ALL, so cctao cannot tell which findings it selects".to_string())
    );
}

#[test]
fn a_leading_space_is_refused() {
    assert_eq!(
        ruff_refusal(" E7"),
        Some("ruff check ` E7`: ` E7` is not a rule code, a code prefix or ALL, so cctao cannot tell which findings it selects".to_string())
    );
}

#[test]
fn a_dash_is_refused() {
    assert_eq!(
        ruff_refusal("E-7"),
        Some("ruff check `E-7`: `E-7` is not a rule code, a code prefix or ALL, so cctao cannot tell which findings it selects".to_string())
    );
}

#[test]
fn a_linter_name_is_refused() {
    assert_eq!(
        ruff_refusal("flake8-bugbear"),
        Some("ruff check `flake8-bugbear`: `flake8-bugbear` is not a rule code, a code prefix or ALL, so cctao cannot tell which findings it selects".to_string())
    );
}

#[test]
fn a_category_is_refused() {
    assert_eq!(
        ruff_refusal("correctness"),
        Some("ruff check `correctness`: `correctness` is not a rule code, a code prefix or ALL, so cctao cannot tell which findings it selects".to_string())
    );
}

#[test]
fn the_first_bad_part_of_a_list_is_named() {
    assert_eq!(
        ruff_refusal("E7,correctness"),
        Some("ruff check `E7,correctness`: `correctness` is not a rule code, a code prefix or ALL, so cctao cannot tell which findings it selects".to_string())
    );
}

#[test]
fn digits_before_letters_are_refused() {
    assert_eq!(
        ruff_refusal("7E"),
        Some("ruff check `7E`: `7E` is not a rule code, a code prefix or ALL, so cctao cannot tell which findings it selects".to_string())
    );
}
