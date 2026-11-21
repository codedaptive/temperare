//! Unit tests for check_case_file_name.rs.

use super::check_case_file_name::check_case_file_name;
use crate::Problem;

/// Checks `file` as the one entry of `files = [...]` in the failing case of rule `tm-a`.
fn check(file: &str) -> (bool, Vec<Problem>) {
    let text = format!("files = [\"{file}\"]");
    let mut out = Vec::new();
    // `files = [` is 9 chars, so the entry starts at byte 9, column 10.
    let good = check_case_file_name(&text, file, 9, "tm-a", "failing", &mut out);
    (good, out)
}

#[test]
fn a_relative_name_inside_the_case_folder_is_good() {
    for file in ["a.rs", "src/lib.rs", "./src/lib.rs", "src/..lib.rs", "a..b/c.rs", "ab:c.rs"] {
        assert_eq!(check(file), (true, vec![]), "{file}");
    }
}

#[test]
fn an_absolute_name_or_one_with_a_parent_part_is_a_problem_at_its_entry() {
    for file in ["/etc/passwd", "\\src\\lib.rs", "../lib.rs", "src/../../lib.rs", "src/.."] {
        let what = format!("case file \"{file}\" of baseline rule \"tm-a\" must be a relative path inside its case folder");
        let expected = Problem {
            line: 1,
            col: 10,
            what,
            fix: "Name the file relative to cases/tm-a/failing/, with no leading / or \\, no drive letter and no .. part.".to_string(),
        };
        assert_eq!(check(file), (false, vec![expected]), "{file}");
    }
}

#[test]
fn a_parent_part_after_a_backslash_is_a_problem_on_every_host() {
    let expected = Problem {
        line: 1,
        col: 10,
        what: "case file \"..\\x.rs\" of baseline rule \"tm-a\" must be a relative path inside its case folder".to_string(),
        fix: "Name the file relative to cases/tm-a/failing/, with no leading / or \\, no drive letter and no .. part.".to_string(),
    };
    assert_eq!(check("..\\x.rs"), (false, vec![expected]));
}

#[test]
fn a_parent_part_between_backslashes_is_a_problem_on_every_host() {
    let expected = Problem {
        line: 1,
        col: 10,
        what: "case file \"src\\..\\x.rs\" of baseline rule \"tm-a\" must be a relative path inside its case folder".to_string(),
        fix: "Name the file relative to cases/tm-a/failing/, with no leading / or \\, no drive letter and no .. part.".to_string(),
    };
    assert_eq!(check("src\\..\\x.rs"), (false, vec![expected]));
}

#[test]
fn a_drive_letter_with_a_slash_is_a_problem_on_every_host() {
    let expected = Problem {
        line: 1,
        col: 10,
        what: "case file \"C:/x.rs\" of baseline rule \"tm-a\" must be a relative path inside its case folder".to_string(),
        fix: "Name the file relative to cases/tm-a/failing/, with no leading / or \\, no drive letter and no .. part.".to_string(),
    };
    assert_eq!(check("C:/x.rs"), (false, vec![expected]));
}

#[test]
fn a_drive_letter_with_a_backslash_is_a_problem_on_every_host() {
    let expected = Problem {
        line: 1,
        col: 10,
        what: "case file \"C:\\x.rs\" of baseline rule \"tm-a\" must be a relative path inside its case folder".to_string(),
        fix: "Name the file relative to cases/tm-a/failing/, with no leading / or \\, no drive letter and no .. part.".to_string(),
    };
    assert_eq!(check("C:\\x.rs"), (false, vec![expected]));
}

#[test]
fn a_drive_letter_with_no_separator_after_it_is_a_problem_on_every_host() {
    let expected = Problem {
        line: 1,
        col: 10,
        what: "case file \"a:b.rs\" of baseline rule \"tm-a\" must be a relative path inside its case folder".to_string(),
        fix: "Name the file relative to cases/tm-a/failing/, with no leading / or \\, no drive letter and no .. part.".to_string(),
    };
    assert_eq!(check("a:b.rs"), (false, vec![expected]));
}
