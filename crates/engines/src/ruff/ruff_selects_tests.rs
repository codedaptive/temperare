//! Unit tests for ruff_selects.rs: which codes a `--select` list selects, the cases written out from the contract.

use super::ruff_selects;

#[test]
fn a_prefix_selects_the_codes_it_starts() {
    assert!(ruff_selects("E7", "E701"));
    assert!(ruff_selects("E7", "E711"));
}

#[test]
fn a_prefix_does_not_select_other_codes() {
    assert!(!ruff_selects("E7", "E501"));
    assert!(!ruff_selects("E7", "F401"));
}

#[test]
fn an_exact_code_selects_itself_and_not_its_neighbour() {
    assert!(ruff_selects("E711", "E711"));
    assert!(!ruff_selects("E712", "E711"));
}

#[test]
fn all_selects_anything() {
    assert!(ruff_selects("ALL", "E711"));
    assert!(ruff_selects("ALL", "F401"));
    assert!(ruff_selects("E7,ALL", "F401"));
}

#[test]
fn any_part_of_a_list_selects() {
    assert!(ruff_selects("E7,F4", "F401"));
    assert!(ruff_selects("E7,F4", "E711"));
}

#[test]
fn an_empty_part_selects_nothing() {
    assert!(!ruff_selects("E7,", "F401"));
    assert!(!ruff_selects(",", "F401"));
    assert!(!ruff_selects("", "E711"));
}

#[test]
fn a_part_is_not_trimmed() {
    assert!(!ruff_selects("E7, F4", "F401"));
}
