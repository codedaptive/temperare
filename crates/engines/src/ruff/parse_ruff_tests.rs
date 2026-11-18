//! ZUT for parse_ruff.rs: real ruff 0.17.4 output, captured once, read into hits.
//!
//! e711_and_f401.json: `ruff check --no-cache --output-format json --select E711,F401 example.py`, exit 1.
//! clean.json: `ruff check --no-cache --output-format json --select E711 clean.py`, exit 0.
//! e7_prefix.json: `ruff check --no-cache --output-format json --select E7 .`, exit 1.
//! `ruff_cmd` passes these values as `--select=E711` and so on (S-35); ruff reads both spellings the same way, so
//! the captures stand for today's run.
//! In e711_and_f401.json the capture's home-folder paths are replaced by `/work/project`, so they and `DIR` start
//! with it. e7_prefix.json is the exception: it keeps its capture folder, the temp folder `/private/tmp/d1-ruff-probe`,
//! which its test passes as both `cwd` and `folder`.

use super::parse_ruff;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use std::path::Path;

const DIR: &str = "/work/project/cap-ruff";

fn output(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

fn two_rules() -> Output {
    output(1, include_bytes!("fixtures/e711_and_f401.json"), b"")
}

#[test]
fn only_the_checked_rule_becomes_a_hit() {
    let hits = parse_ruff(&two_rules(), "E711", Path::new(DIR), Path::new(DIR)).unwrap();
    assert_eq!(
        hits,
        vec![Hit {
            path: RelPath::new("example.py"),
            line: 3,
            col: Some(13),
            what: "Comparison to `None` should be `cond is None`".to_string(),
        }]
    );
}

#[test]
fn the_other_rule_is_read_when_it_is_the_check() {
    let hits = parse_ruff(&two_rules(), "F401", Path::new(DIR), Path::new(DIR)).unwrap();
    assert_eq!(
        hits,
        vec![Hit {
            path: RelPath::new("example.py"),
            line: 1,
            col: Some(8),
            what: "`os` imported but unused".to_string(),
        }]
    );
}

#[test]
fn a_diagnostic_in_a_file_outside_the_folder_is_not_a_hit() {
    let folder = Path::new(DIR).join("sub");
    assert_eq!(parse_ruff(&two_rules(), "E711", Path::new(DIR), &folder), Ok(vec![]));
}

#[test]
fn a_null_code_diagnostic_matches_no_check_and_does_not_fail_the_run() {
    // Built by hand, not captured: ruff prints `"code": null` for a syntax error in some versions.
    let stdout = br#"[
        {"code": null, "filename": "/w/py/broken.py", "location": {"row": 2, "column": 5}, "message": "SyntaxError: Expected an expression"},
        {"code": "E711", "filename": "/w/py/example.py", "location": {"row": 3, "column": 13}, "message": "Comparison to `None` should be `cond is None`"}
    ]"#;
    let hits = parse_ruff(&output(1, stdout, b""), "E711", Path::new("/w/py"), Path::new("/w/py")).unwrap();
    assert_eq!(
        hits,
        vec![Hit {
            path: RelPath::new("example.py"),
            line: 3,
            col: Some(13),
            what: "Comparison to `None` should be `cond is None`".to_string(),
        }]
    );
}

#[test]
fn a_clean_run_has_no_hits() {
    let out = output(0, include_bytes!("fixtures/clean.json"), b"");
    assert_eq!(parse_ruff(&out, "E711", Path::new(DIR), Path::new(DIR)), Ok(vec![]));
}

#[test]
fn stdout_that_is_not_json_quotes_stderr() {
    let out = output(2, b"garbage", b"ruff failed: invalid --select");
    assert_eq!(
        parse_ruff(&out, "E711", Path::new(DIR), Path::new(DIR)),
        Err(ParseError { why: "ruff printed no JSON findings: ruff failed: invalid --select".to_string() })
    );
}

#[test]
fn with_stderr_empty_the_error_quotes_the_first_200_bytes_of_stdout() {
    let stdout = format!("{}{}", "y".repeat(200), "TAIL");
    let out = output(0, stdout.as_bytes(), b"");
    let why = parse_ruff(&out, "E711", Path::new(DIR), Path::new(DIR)).unwrap_err().why;
    assert_eq!(why, format!("ruff printed no JSON findings: {}", "y".repeat(200)));
}

#[test]
fn d1_ruff_prefix_keeps_both_captured_hits_in_order() {
    let dir = Path::new("/private/tmp/d1-ruff-probe");
    let out = output(1, include_bytes!("fixtures/e7_prefix.json"), b"");
    assert_eq!(parse_ruff(&out, "E7", dir, dir), Ok(vec![
        Hit { path: RelPath::new("example.py"), line: 3, col: Some(13), what: "Comparison to `None` should be `cond is None`".to_string() },
        Hit { path: RelPath::new("example.py"), line: 3, col: Some(17), what: "Multiple statements on one line (colon)".to_string() },
    ]));
}

#[test]
fn d1_ruff_all_selects_all_coded_diagnostics_in_order() {
    assert_eq!(parse_ruff(&two_rules(), "ALL", Path::new(DIR), Path::new(DIR)), Ok(vec![
        Hit { path: RelPath::new("example.py"), line: 1, col: Some(8), what: "`os` imported but unused".to_string() },
        Hit { path: RelPath::new("example.py"), line: 3, col: Some(13), what: "Comparison to `None` should be `cond is None`".to_string() },
    ]));
}

#[test]
fn d1_ruff_comma_separated_prefixes_select_both() {
    assert_eq!(parse_ruff(&two_rules(), "E7,F4", Path::new(DIR), Path::new(DIR)), Ok(vec![
        Hit { path: RelPath::new("example.py"), line: 1, col: Some(8), what: "`os` imported but unused".to_string() },
        Hit { path: RelPath::new("example.py"), line: 3, col: Some(13), what: "Comparison to `None` should be `cond is None`".to_string() },
    ]));
}

#[test]
fn d1_ruff_f4_selects_only_f401() {
    assert_eq!(parse_ruff(&two_rules(), "F4", Path::new(DIR), Path::new(DIR)), Ok(vec![
        Hit { path: RelPath::new("example.py"), line: 1, col: Some(8), what: "`os` imported but unused".to_string() },
    ]));
}

#[test]
fn d1_ruff_e71_selects_only_e711() {
    assert_eq!(parse_ruff(&two_rules(), "E71", Path::new(DIR), Path::new(DIR)), Ok(vec![
        Hit { path: RelPath::new("example.py"), line: 3, col: Some(13), what: "Comparison to `None` should be `cond is None`".to_string() },
    ]));
}

#[test]
fn d1_ruff_e712_does_not_select_e711() {
    assert_eq!(parse_ruff(&two_rules(), "E712", Path::new(DIR), Path::new(DIR)), Ok(vec![]));
}

#[test]
fn d1_ruff_empty_list_part_selects_nothing() {
    assert_eq!(parse_ruff(&two_rules(), "E7,", Path::new(DIR), Path::new(DIR)), Ok(vec![
        Hit { path: RelPath::new("example.py"), line: 3, col: Some(13), what: "Comparison to `None` should be `cond is None`".to_string() },
    ]));
}

#[test]
fn d1_ruff_null_code_is_not_selected_by_all_or_prefix() {
    let stdout = br#"[{"code":null,"filename":"/w/broken.py","location":{"row":2,"column":5},"message":"SyntaxError"}]"#;
    let out = output(1, stdout, b"");
    for check in ["ALL", "E7", "E711", ""] {
        assert_eq!(parse_ruff(&out, check, Path::new("/w"), Path::new("/w")), Ok(vec![]), "{check}");
    }
}

#[test]
fn d1_ruff_remapped_check_is_an_error() {
    let out = output(0, b"[]", b"warning: `TCH001` has been remapped to `TC001`.\n");
    let got = parse_ruff(&out, "TCH001", Path::new(DIR), Path::new(DIR));
    assert!(got.is_err(), "{got:?}");
    assert!(got.unwrap_err().why.contains("remapped to `TC001`"));
}

#[test]
fn d1_ruff_remap_of_a_code_the_check_does_not_select_is_not_an_error() {
    let out = output(1, include_bytes!("fixtures/e711_and_f401.json"), b"warning: `TCH001` has been remapped to `TC001`.\n");
    assert_eq!(parse_ruff(&out, "E7", Path::new(DIR), Path::new(DIR)), Ok(vec![
        Hit { path: RelPath::new("example.py"), line: 3, col: Some(13), what: "Comparison to `None` should be `cond is None`".to_string() },
    ]));
}
