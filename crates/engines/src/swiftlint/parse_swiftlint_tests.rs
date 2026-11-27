//! Unit tests for parse_swiftlint.rs: captured and inline swiftlint JSON read into hits.
//!
//! force_cast_and_others.json: `swiftlint lint --reporter json --quiet --no-cache --config two.yml Example.swift
//! NoNewline.swift` (two.yml's only_rules: force_cast, trailing_whitespace, trailing_newline), exit 2.
//! clean.json: `swiftlint lint --reporter json --quiet --no-cache --only-rule force_cast Clean.swift`, exit 0.
//! `SWIFTLINT`'s command passes `--only-rule=<check>` and `--config=<config>`; swiftlint reads both spellings
//! the same way, so the captures stand for the run it builds.
//! The captures' home-folder paths are replaced by `/work/project`, so they and `DIR` start with it.

use super::parse_swiftlint;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use std::path::Path;

const DIR: &str = "/work/project/cap-swiftlint";

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
                line: Some(2),
                col: Some(22),
                what: "Force casts should be avoided".to_string(),
            },
            Hit {
                path: RelPath::new("NoNewline.swift"),
                line: Some(1),
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
            line: Some(1),
            col: None,
            what: "Files should have a single trailing newline".to_string(),
        }]
    );
}

#[test]
fn private_path_checked_violation_outside_the_folder_is_an_error() {
    let folder = Path::new(DIR).join("sub");
    assert_eq!(
        parse_swiftlint(&mixed(), "force_cast", Path::new(DIR), &folder),
        Err(ParseError { why: format!(
            "swiftlint printed `/work/project/cap-swiftlint/Example.swift`, a path not under the rule folder `{}`",
            Path::new("/work/project/cap-swiftlint").join("sub").display()
        ) })
    );
}

#[test]
fn private_path_tmp_folder_keeps_a_printed_tmp_finding() {
    let out = output(2, br#"[{"file":"/tmp/cap/Sources/Cast.swift","line":3,"character":17,"reason":"Force casts should be avoided","rule_id":"force_cast","severity":"Error","type":"Force Cast"}]"#, b"");
    let folder = Path::new("/private/tmp/cap");
    assert_eq!(
        parse_swiftlint(&out, "force_cast", folder, folder),
        Ok(vec![Hit { path: RelPath::new("Sources/Cast.swift"), line: Some(3), col: Some(17), what: "Force casts should be avoided".to_string() }])
    );
}

#[test]
fn private_path_tmp_folder_keeps_a_printed_private_tmp_finding() {
    let out = output(2, br#"[{"file":"/private/tmp/cap/Sources/Cast.swift","line":3,"character":17,"reason":"Force casts should be avoided","rule_id":"force_cast","severity":"Error","type":"Force Cast"}]"#, b"");
    let folder = Path::new("/private/tmp/cap");
    assert_eq!(
        parse_swiftlint(&out, "force_cast", folder, folder),
        Ok(vec![Hit { path: RelPath::new("Sources/Cast.swift"), line: Some(3), col: Some(17), what: "Force casts should be avoided".to_string() }])
    );
}

#[test]
fn private_path_checked_finding_outside_both_tmp_forms_is_an_error() {
    let out = output(2, br#"[{"file":"/tmp/other/Cast.swift","line":3,"character":17,"reason":"Force casts should be avoided","rule_id":"force_cast","severity":"Error","type":"Force Cast"}]"#, b"");
    let folder = Path::new("/private/tmp/cap");
    assert_eq!(
        parse_swiftlint(&out, "force_cast", folder, folder),
        Err(ParseError { why: "swiftlint printed `/tmp/other/Cast.swift`, a path not under the rule folder `/private/tmp/cap`".to_string() })
    );
}

#[test]
fn private_path_outside_finding_of_another_rule_is_filtered_before_placement() {
    let out = output(2, br#"[{"file":"/tmp/other/Cast.swift","line":3,"character":17,"reason":"Force casts should be avoided","rule_id":"another_rule","severity":"Error","type":"Force Cast"}]"#, b"");
    let folder = Path::new("/private/tmp/cap");
    assert_eq!(parse_swiftlint(&out, "force_cast", folder, folder), Ok(vec![]));
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

#[test]
fn d1_swiftlint_invalid_identifier_warning_is_an_error() {
    let out = output(0, b"[\n\n]", b"warning: 'force_cast,force_try' is not a valid rule identifier\nValid rule identifiers:\nforce_cast\nforce_try\nline_length\n");
    assert_eq!(
        parse_swiftlint(&out, "force_cast,force_try", Path::new(DIR), Path::new(DIR)),
        Err(ParseError { why: "swiftlint check `force_cast,force_try` is not a rule this swiftlint knows (warning: 'force_cast,force_try' is not a valid rule identifier): cctao cannot report it; name a rule `swiftlint rules` lists, or a custom rule the rule's config defines".to_string() })
    );
}
