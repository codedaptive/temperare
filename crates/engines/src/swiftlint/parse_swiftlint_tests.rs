//! ZUT for parse_swiftlint.rs: real swiftlint 0.65.1 output, captured once, read into hits.
//!
//! force_cast_and_others.json: `swiftlint lint --reporter json --quiet --no-cache --config two.yml Example.swift
//! NoNewline.swift` (two.yml's only_rules: force_cast, trailing_whitespace, trailing_newline), exit 2.
//! clean.json: `swiftlint lint --reporter json --quiet --no-cache --only-rule force_cast Clean.swift`, exit 0.

use super::parse_swiftlint;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use std::path::Path;

const DIR: &str = "/Users/bob/devlop/team-temperare/workcrews/engines/scratchpad/cap-swiftlint";

fn output(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

fn mixed() -> Output {
    output(2, include_bytes!("fixtures/force_cast_and_others.json"), b"")
}

#[test]
fn only_the_checked_rule_becomes_hits_in_output_order() {
    let hits = parse_swiftlint(&mixed(), "force_cast", Path::new(DIR), Path::new(DIR)).unwrap();
    assert_eq!(
        hits,
        vec![
            Hit {
                path: RelPath::new("Example.swift"),
                line: 2,
                col: Some(22),
                what: "Force casts should be avoided".to_string(),
            },
            Hit {
                path: RelPath::new("NoNewline.swift"),
                line: 1,
                col: Some(11),
                what: "Force casts should be avoided".to_string(),
            },
        ]
    );
}

#[test]
fn a_null_character_is_no_column() {
    let hits = parse_swiftlint(&mixed(), "trailing_newline", Path::new(DIR), Path::new(DIR)).unwrap();
    assert_eq!(
        hits,
        vec![Hit {
            path: RelPath::new("NoNewline.swift"),
            line: 1,
            col: None,
            what: "Files should have a single trailing newline".to_string(),
        }]
    );
}

#[test]
fn a_violation_in_a_file_outside_the_folder_is_not_a_hit() {
    let folder = Path::new(DIR).join("sub");
    assert_eq!(parse_swiftlint(&mixed(), "force_cast", Path::new(DIR), &folder), Ok(vec![]));
}

#[test]
fn a_clean_run_has_no_hits() {
    let out = output(0, include_bytes!("fixtures/clean.json"), b"");
    assert_eq!(parse_swiftlint(&out, "force_cast", Path::new(DIR), Path::new(DIR)), Ok(vec![]));
}

#[test]
fn stdout_that_is_not_json_quotes_stderr() {
    let out = output(1, b"garbage", b"Error: could not read configuration file");
    assert_eq!(
        parse_swiftlint(&out, "force_cast", Path::new(DIR), Path::new(DIR)),
        Err(ParseError { why: "swiftlint printed no JSON findings: Error: could not read configuration file".to_string() })
    );
}

#[test]
fn with_stderr_empty_the_error_quotes_the_first_200_bytes_of_stdout() {
    let stdout = format!("{}{}", "x".repeat(200), "TAIL");
    let out = output(0, stdout.as_bytes(), b"");
    let why = parse_swiftlint(&out, "force_cast", Path::new(DIR), Path::new(DIR)).unwrap_err().why;
    assert_eq!(why, format!("swiftlint printed no JSON findings: {}", "x".repeat(200)));
}
