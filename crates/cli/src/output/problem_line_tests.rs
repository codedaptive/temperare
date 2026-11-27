//! ZUT for problem_line.rs: the `file:line:col  what  fix` format.

use super::problem_line;
use std::path::Path;

fn problem(line: usize, col: usize, what: &str, fix: &str) -> rules::Problem {
    rules::Problem { line, col, what: what.to_string(), fix: fix.to_string() }
}

#[test]
fn file_line_col_then_what_and_fix_two_spaces_apart() {
    let p = problem(3, 1, "unknown key `colour` in the top level", "Remove the key.");
    assert_eq!(
        problem_line(Path::new("repo/tao.toml"), &p),
        "repo/tao.toml:3:1  unknown key `colour` in the top level  Remove the key."
    );
}

#[test]
fn a_problem_with_no_position_prints_zero_zero() {
    let p = problem(0, 0, "cannot read the rule file", "Write it.");
    assert_eq!(problem_line(Path::new("t.toml"), &p), "t.toml:0:0  cannot read the rule file  Write it.");
}
