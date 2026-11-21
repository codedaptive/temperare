//! Unit tests for check_failing_line.rs.

use super::check_failing_line::check_failing_line;
use crate::{CaseFile, Problem};

/// Checks `line` of the failing case of rule `tm-a`, whose path is `path`, from the text `line = <line>`, against the
/// carried files `src/lib.rs` (three lines, the last ending in a newline), `two.rs` (two lines, no final newline) and
/// `empty.rs` (empty).
fn check(path: &str, line: usize) -> Vec<Problem> {
    let text = format!("line = {line}");
    let files = [
        CaseFile { path: "src/lib.rs".to_string(), text: "one\ntwo\nthree\n" },
        CaseFile { path: "two.rs".to_string(), text: "one\ntwo" },
        CaseFile { path: "empty.rs".to_string(), text: "" },
    ];
    let mut out = Vec::new();
    // `line = ` is 7 chars, so the value starts at byte 7, column 8.
    check_failing_line(&text, path, line, 7, &files, "tm-a", &mut out);
    out
}

fn past(what: &str) -> Problem {
    Problem {
        line: 1,
        col: 8,
        what: what.to_string(),
        fix: "Set line to the line of the finding in that file, or correct the case file.".to_string(),
    }
}

#[test]
fn a_line_of_the_file_is_fine() {
    for (path, line) in [("src/lib.rs", 1), ("src/lib.rs", 3), ("two.rs", 2)] {
        assert_eq!(check(path, line), vec![], "{path}:{line}");
    }
}

#[test]
fn a_line_past_the_last_line_is_a_problem_at_the_line() {
    let expected = past("failing line 4 of baseline rule \"tm-a\" is past the end of \"src/lib.rs\", which has 3 lines");
    assert_eq!(check("src/lib.rs", 4), vec![expected]);
}

#[test]
fn a_file_with_no_final_newline_ends_at_its_last_line() {
    let expected = past("failing line 3 of baseline rule \"tm-a\" is past the end of \"two.rs\", which has 2 lines");
    assert_eq!(check("two.rs", 3), vec![expected]);
}

#[test]
fn every_line_is_past_an_empty_file() {
    let expected = past("failing line 1 of baseline rule \"tm-a\" is past the end of \"empty.rs\", which has 0 lines");
    assert_eq!(check("empty.rs", 1), vec![expected]);
}

#[test]
fn a_path_that_is_not_among_the_files_gives_nothing_here() {
    assert_eq!(check("gone.rs", 99), vec![]);
}
