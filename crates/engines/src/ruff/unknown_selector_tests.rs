//! Unit tests for unknown_selector.rs: ruff's unknown-selector line picked out of its stderr when it names a part of the
//! rule's check, and left alone when it names another selector.

use super::unknown_selector::unknown_selector;
use procio::Output;

fn with_stderr(stderr: &[u8]) -> Output {
    Output { code: Some(2), stdout: Vec::new(), stderr: stderr.to_vec() }
}

const CAUSE: &[u8] = b"ruff failed\n  Cause: Unknown rule selector `ZZZ999` in `select` from the CLI\n";

#[test]
fn the_cause_line_ruff_prints_is_that_line_trimmed() {
    assert_eq!(
        unknown_selector(&with_stderr(CAUSE), "ZZZ999"),
        Some("Cause: Unknown rule selector `ZZZ999` in `select` from the CLI".to_string())
    );
}

#[test]
fn the_invalid_value_form_naming_the_check_is_found_too() {
    let out = with_stderr(
        b"error: invalid value 'ZZZ999' for '--select <RULE_CODE>': Unknown rule selector: `ZZZ999`\n\nFor more information, try '--help'.\n",
    );
    assert_eq!(
        unknown_selector(&out, "ZZZ999"),
        Some("error: invalid value 'ZZZ999' for '--select <RULE_CODE>': Unknown rule selector: `ZZZ999`".to_string())
    );
    assert_eq!(unknown_selector(&out, "E711"), None);
}

#[test]
fn any_part_of_a_list_check_is_found() {
    assert_eq!(
        unknown_selector(&with_stderr(CAUSE), "E7,ZZZ999"),
        Some("Cause: Unknown rule selector `ZZZ999` in `select` from the CLI".to_string())
    );
}

#[test]
fn a_part_is_matched_untrimmed() {
    let out = with_stderr(b"ruff failed\n  Cause: Unknown rule selector ` F4` in `select` from the CLI\n");
    assert_eq!(
        unknown_selector(&out, "E7, F4"),
        Some("Cause: Unknown rule selector ` F4` in `select` from the CLI".to_string())
    );
    assert_eq!(unknown_selector(&out, "E7,F4"), None);
}

#[test]
fn a_line_naming_another_selector_is_none() {
    let out = with_stderr(b"ruff failed\n  Cause: Unknown rule selector `QQQ1` in `lint.select` in ruff.toml\n");
    assert_eq!(unknown_selector(&out, "E711"), None);
}

#[test]
fn a_selector_the_check_only_starts_with_is_none() {
    assert_eq!(unknown_selector(&with_stderr(CAUSE), "ZZZ9"), None);
}

#[test]
fn stderr_without_the_words_is_none() {
    assert_eq!(unknown_selector(&with_stderr(b"ruff failed\n  Cause: bad config\n"), "E711"), None);
}

#[test]
fn empty_stderr_is_none() {
    assert_eq!(unknown_selector(&with_stderr(b""), "E711"), None);
}
