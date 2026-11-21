//! Unit tests for parse_ast_grep.rs, given ast-grep 0.51.1's real output captured once (fixtures/): a two-document rule
//! (JavaScript `js-names` with a message, Python `py-names` without one) over `a.js` and `b.py`, exit 0; a rule
//! matching nothing, exit 0 with empty stdout; and a rule YAML ast-grep cannot parse, exit 8 with only stderr.

use std::path::Path;

use procio::Output;
use rules::RelPath;

use super::parse_ast_grep;
use crate::Hit;

const MATCHES: &[u8] = include_bytes!("fixtures/matches.jsonl");
const CLEAN: &[u8] = include_bytes!("fixtures/clean.jsonl");
const BROKEN_RULE_STDERR: &[u8] = include_bytes!("fixtures/broken_rule.stderr");

fn output(code: Option<i32>, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code, stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

fn hit(path: &str, line: u32, col: u32, what: &str) -> Hit {
    Hit { path: RelPath::new(path), line: Some(line), col: Some(col), what: what.to_string() }
}

#[test]
fn every_match_is_a_hit_in_output_order_with_one_based_positions() {
    let folder = Path::new("/w/repo");
    let got = parse_ast_grep(&output(Some(0), MATCHES, b""), folder, folder);
    assert_eq!(
        got,
        Ok(vec![
            hit("a.js", 1, 13, "matched js name"),
            hit("a.js", 2, 13, "matched js name"),
            hit("a.js", 3, 9, "matched js name"),
            hit("b.py", 2, 1, "py-names"),
        ])
    );
}

#[test]
fn paths_are_made_relative_to_the_folder_not_the_working_folder() {
    let got = parse_ast_grep(&output(Some(0), MATCHES, b""), Path::new("/w/repo"), Path::new("/w")).unwrap();
    let paths: Vec<RelPath> = got.into_iter().map(|hit| hit.path).collect();
    assert_eq!(paths, [RelPath::new("repo/a.js"), RelPath::new("repo/a.js"), RelPath::new("repo/a.js"), RelPath::new("repo/b.py")]);
}

#[test]
fn matches_outside_the_folder_are_skipped() {
    let got = parse_ast_grep(&output(Some(0), MATCHES, b""), Path::new("/w"), Path::new("/w/repo"));
    assert_eq!(got, Ok(vec![]));
}

#[test]
fn a_clean_run_has_no_hits() {
    assert_eq!(parse_ast_grep(&output(Some(0), CLEAN, b""), Path::new("/w"), Path::new("/w")), Ok(vec![]));
}

#[test]
fn matches_count_whatever_the_exit_code() {
    let got = parse_ast_grep(&output(Some(1), MATCHES, b""), Path::new("/w"), Path::new("/w")).unwrap();
    assert_eq!(got.len(), 4);
}

#[test]
fn a_rule_ast_grep_cannot_read_is_a_parse_error_quoting_stderr() {
    let why = parse_ast_grep(&output(Some(8), CLEAN, BROKEN_RULE_STDERR), Path::new("/w"), Path::new("/w")).unwrap_err().why;
    assert!(why.starts_with("ast-grep "), "{why}");
    assert!(why.contains("Error: Cannot parse rule"), "{why}");
}

#[test]
fn empty_stdout_from_a_killed_run_is_a_parse_error() {
    assert!(parse_ast_grep(&output(None, b"", b""), Path::new("/w"), Path::new("/w")).is_err());
}

#[test]
fn a_line_that_is_not_json_is_a_parse_error_quoting_stdout() {
    let why = parse_ast_grep(&output(Some(0), b"oops, not json\n", b""), Path::new("/w"), Path::new("/w")).unwrap_err().why;
    assert!(why.starts_with("ast-grep ") && why.contains("oops, not json"), "{why}");
}

#[test]
fn the_quote_is_cut_at_two_hundred_bytes() {
    let stderr = [b'x'; 300];
    let why = parse_ast_grep(&output(Some(2), b"", &stderr), Path::new("/w"), Path::new("/w")).unwrap_err().why;
    assert!(why.ends_with(&"x".repeat(200)) && !why.contains(&"x".repeat(201)), "{why}");
}
