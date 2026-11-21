//! Unit tests for version_word.rs on fixed `--version` outputs.

use super::version_word;
use procio::Output;

fn out(code: i32, stdout: &str) -> Output {
    Output { code: Some(code), stdout: stdout.as_bytes().to_vec(), stderr: Vec::new() }
}

#[test]
fn the_version_word_of_each_engines_output() {
    let cases = [
        ("ruff 0.17.4\n", "0.17.4"),
        ("clippy 0.1.95 (abc1234 2025-11-30)\n", "0.1.95"),
        ("0.59.1\n", "0.59.1"),
        ("ast-grep 0.39.6\n", "0.39.6"),
    ];
    for (stdout, version) in cases {
        assert_eq!(version_word(&out(0, stdout)), Some(version.to_string()), "{stdout:?}");
    }
}

#[test]
fn a_trailing_comma_or_parenthesis_is_trimmed() {
    assert_eq!(version_word(&out(0, "tool version 1.2.3, built today\n")), Some("1.2.3".to_string()));
    assert_eq!(version_word(&out(0, "tool (1.2.3)\n")), None);
    assert_eq!(version_word(&out(0, "tool 4.5)\n")), Some("4.5".to_string()));
}

#[test]
fn blank_lines_before_the_first_line_are_skipped() {
    assert_eq!(version_word(&out(0, "\n  \nruff 0.17.4\n")), Some("0.17.4".to_string()));
}

#[test]
fn only_the_first_non_empty_line_is_read() {
    assert_eq!(version_word(&out(0, "no version here\nruff 0.17.4\n")), None);
}

#[test]
fn a_probe_that_did_not_exit_0_gives_none() {
    assert_eq!(version_word(&out(1, "ruff 0.17.4\n")), None);
    assert_eq!(version_word(&Output { code: None, stdout: b"ruff 0.17.4\n".to_vec(), stderr: Vec::new() }), None);
}

#[test]
fn output_with_no_digit_word_gives_none() {
    assert_eq!(version_word(&out(0, "no digits")), None);
    assert_eq!(version_word(&out(0, "")), None);
}
