//! ZUT for check_duplicate_case_file.rs.

use super::check_duplicate_case_file::check_duplicate_case_file;
use crate::Problem;

/// Checks the second entry of `files = ["a.rs", "<file>"]` in the passing case of rule `tm-a`, after the names
/// `earlier`.
fn check(file: &str, earlier: &[&str]) -> (bool, Vec<Problem>) {
    let text = format!("files = [\"a.rs\", \"{file}\"]");
    let earlier: Vec<String> = earlier.iter().map(|name| name.to_string()).collect();
    let mut out = Vec::new();
    // `files = ["a.rs", ` is 17 chars, so the second entry starts at byte 17, column 18.
    let new = check_duplicate_case_file(&text, file, 17, &earlier, "tm-a", "passing", &mut out);
    (new, out)
}

#[test]
fn a_name_not_listed_before_is_new() {
    assert_eq!(check("b.rs", &["a.rs"]), (true, vec![]));
    assert_eq!(check("a.rs", &[]), (true, vec![]));
}

#[test]
fn a_name_listed_before_is_a_problem_at_the_repeat() {
    let expected = Problem {
        line: 1,
        col: 18,
        what: "case file \"a.rs\" is listed twice in the passing case of baseline rule \"tm-a\"".to_string(),
        fix: "Remove the repeated entry from files.".to_string(),
    };
    assert_eq!(check("a.rs", &["src/a.rs", "a.rs"]), (false, vec![expected]));
}

#[test]
fn a_name_that_repeats_an_earlier_one_after_a_dot_part_is_a_problem_named_as_written() {
    let expected = Problem {
        line: 1,
        col: 18,
        what: "case file \"./a.rs\" is listed twice in the passing case of baseline rule \"tm-a\"".to_string(),
        fix: "Remove the repeated entry from files.".to_string(),
    };
    assert_eq!(check("./a.rs", &["a.rs"]), (false, vec![expected]));
}

#[test]
fn a_name_that_repeats_an_earlier_one_with_backslashes_is_a_problem_named_as_written() {
    let expected = Problem {
        line: 1,
        col: 18,
        what: "case file \"src\\lib.rs\" is listed twice in the passing case of baseline rule \"tm-a\"".to_string(),
        fix: "Remove the repeated entry from files.".to_string(),
    };
    assert_eq!(check("src\\lib.rs", &["src/lib.rs"]), (false, vec![expected]));
}

#[test]
fn a_name_that_repeats_an_earlier_one_with_an_empty_part_is_a_problem_named_as_written() {
    let expected = Problem {
        line: 1,
        col: 18,
        what: "case file \"src//lib.rs\" is listed twice in the passing case of baseline rule \"tm-a\"".to_string(),
        fix: "Remove the repeated entry from files.".to_string(),
    };
    assert_eq!(check("src//lib.rs", &["src/lib.rs"]), (false, vec![expected]));
}

#[test]
fn a_rooted_name_does_not_repeat_the_same_relative_name() {
    assert_eq!(check("/a.rs", &["a.rs"]), (true, vec![]));
}
