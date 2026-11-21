//! Unit tests for check_failing_path.rs.

use super::check_failing_path::check_failing_path;
use crate::Problem;

/// Checks `path` against `listed` as the failing case of rule `tm-a`, from the text `path = "<path>"`.
fn check(path: &str, listed: &[&str]) -> Vec<Problem> {
    let text = format!("path = \"{path}\"");
    let listed: Vec<String> = listed.iter().map(|file| file.to_string()).collect();
    let mut out = Vec::new();
    // `path = ` is 7 chars, so the value starts at byte 7, column 8.
    check_failing_path(&text, path, 7, &listed, "tm-a", &mut out);
    out
}

#[test]
fn a_path_that_is_one_of_the_listed_files_is_fine() {
    assert_eq!(check("src/lib.rs", &["Cargo.toml", "src/lib.rs"]), vec![]);
}

#[test]
fn a_path_that_is_not_listed_is_a_problem_at_it() {
    let expected = Problem {
        line: 1,
        col: 8,
        what: "failing path \"src/b.rs\" of baseline rule \"tm-a\" is not one of its failing files".to_string(),
        fix: "Set path to one of the failing case's files, or list it in files.".to_string(),
    };
    assert_eq!(check("src/b.rs", &["b.rs", "src/lib.rs"]), vec![expected]);
}

#[test]
fn no_listed_files_gives_nothing_here() {
    assert_eq!(check("src/lib.rs", &[]), vec![]);
}
