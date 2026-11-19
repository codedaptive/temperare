//! Unit tests for stderr_tail.rs.

use crate::stderr_tail;

#[test]
fn twenty_five_lines_keep_lines_six_to_twenty_five() {
    let stderr: String = (1..=25).map(|n| format!("line{n}\n")).collect();
    let tail = stderr_tail(&stderr);
    let kept: Vec<&str> = tail.lines().collect();
    assert_eq!(kept.len(), 20);
    assert_eq!(kept[0], "line6");
    assert_eq!(kept[19], "line25");
    assert!(tail.starts_with("line6\nline7\n"));
    assert!(tail.ends_with("line24\nline25"));
}

#[test]
fn three_lines_are_kept_whole() {
    assert_eq!(stderr_tail("a\nb\nc"), "a\nb\nc");
}

#[test]
fn a_trailing_newline_is_not_kept() {
    assert_eq!(stderr_tail("a\nb\nc\n"), "a\nb\nc");
}

#[test]
fn empty_stays_empty() {
    assert_eq!(stderr_tail(""), "");
}
