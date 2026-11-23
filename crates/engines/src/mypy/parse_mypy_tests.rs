//! ZUT for parse_mypy.rs: real mypy 2.4.1 output (fixtures/, made as fixtures/fixtures.txt records, each run in its
//! own folder, here `/w`, and each row's Output built with that run's exit code) read into hits, and the blocking
//! errors that are not an answer.

use super::parse_mypy;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use std::path::Path;

const ARG_TYPE: &[u8] = include_bytes!("fixtures/arg_type.jsonl");
const NOTES: &[u8] = include_bytes!("fixtures/notes.jsonl");
const SYNTAX: &[u8] = include_bytes!("fixtures/syntax.jsonl");
const DUPLICATE_MODULE: &[u8] = include_bytes!("fixtures/duplicate_module.jsonl");
const NO_PYTHON: &[u8] = include_bytes!("fixtures/no_python.txt");
const ARG_TYPE_WHAT: &str = "Argument 1 to \"f\" has incompatible type \"str\"; expected \"int\"";

fn hit(path: &str, line: u32, col: u32, what: &str) -> Hit {
    Hit { path: RelPath::new(path), line: Some(line), col: Some(col), what: what.to_string() }
}

fn blocked(check: &str, first: &str) -> ParseError {
    ParseError { why: format!("mypy stopped at a blocking error, so it checked nothing for `{check}`: {first}") }
}

#[test]
fn parse_mypy_cases() {
    let duplicate = "y/m.py:-1: Duplicate module named \"m\" (also at \"x/m.py\")";
    let rows: Vec<(&str, i32, &[u8], &[u8], &str, &str, Result<Vec<Hit>, ParseError>)> = vec![
        ("arg-type kept, import-not-found left", 1, ARG_TYPE, b"", "arg-type", "/w", Ok(vec![hit("a.py", 8, 3, ARG_TYPE_WHAT)])),
        ("import-not-found without its hint", 1, ARG_TYPE, b"", "import-not-found", "/w", Ok(vec![hit("a.py", 1, 1, "Cannot find implementation or library stub for module named \"nosuchmodule\"")])),
        ("a code with no finding", 1, ARG_TYPE, b"", "misc", "/w", Ok(vec![])),
        ("a note is not a hit", 1, NOTES, b"", "misc", "/w", Ok(vec![])),
        ("the error beside a note", 1, NOTES, b"", "arg-type", "/w", Ok(vec![hit("a.py", 6, 3, ARG_TYPE_WHAT)])),
        ("a file outside the folder", 1, ARG_TYPE, b"", "arg-type", "/w/sub", Ok(vec![])),
        ("syntax on syntax errors", 2, SYNTAX, b"", "syntax", "/w", Ok(vec![
            hit("b.py", 1, 8, "Expected a parameter or the end of the parameter list"),
            hit("b.py", 2, 2, "Unexpected EOF while parsing"),
        ])),
        ("arg-type on syntax errors", 2, SYNTAX, b"", "arg-type", "/w", Err(blocked("arg-type", "b.py:1: Expected a parameter or the end of the parameter list"))),
        ("arg-type on a duplicate module", 2, DUPLICATE_MODULE, b"", "arg-type", "/w", Err(blocked("arg-type", duplicate))),
        ("syntax on a duplicate module", 2, DUPLICATE_MODULE, b"", "syntax", "/w", Err(blocked("syntax", duplicate))),
        ("syntax with no Python file", 2, b"", NO_PYTHON, "syntax", "/w", Err(blocked("syntax", "There are no .py[i] files in directory '.'\n"))),
    ];
    for (label, code, stdout, stderr, check, folder, expected) in rows {
        let out = Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() };
        assert_eq!(parse_mypy(&out, check, Path::new("/w"), Path::new(folder)), expected, "{label}");
    }
}
