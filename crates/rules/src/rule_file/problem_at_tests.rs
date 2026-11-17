//! ZUT for problem_at.rs.

use super::problem_at::problem_at;

#[test]
fn offset_zero_is_line_1_col_1() {
    let p = problem_at("version = 1\n", 0, "w".to_string(), "f");
    assert_eq!((p.line, p.col), (1, 1));
    assert_eq!(p.what, "w");
    assert_eq!(p.fix, "f");
}

#[test]
fn offset_on_a_later_line_counts_lines_and_columns_from_1() {
    // "ab\ncd": byte 4 is 'd', line 2 col 2.
    let p = problem_at("ab\ncd", 4, String::new(), "");
    assert_eq!((p.line, p.col), (2, 2));
}

#[test]
fn column_counts_chars_not_bytes() {
    // "é = 1\n" is 7 bytes; on line 2 "xé = 2" the '=' is byte 11 and the 4th char.
    let p = problem_at("é = 1\nxé = 2", 11, String::new(), "");
    assert_eq!((p.line, p.col), (2, 4));
}
