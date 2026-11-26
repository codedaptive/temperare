//! ZUT for parse_sarif.rs: SARIF 2.1 hits, and Windows drive URIs under a verbatim folder.
//!
//! regions.sarif: two runs. Run 1: R1 in src/a.py at 3:5 ("first", with a second location that is not read); R2 by
//! `rule.id` in relative src/b.py at line 7, no column ("second"); R1 in `my%20dir/c.py` at 1:2 ("spaced"); R1 outside
//! the folder; R3 with no location; a result with no rule id. Run 2: R2 in z.py at 9:1 with no message.
//! no_region.sarif: R1 in f.py with no region. empty_run.sarif: one run with `"results": []`.
//! Except in the verbatim-folder test (Windows only), every path is under `/work/project`, passed as `cwd` and (unless
//! a test says otherwise) `folder`.

use super::parse_sarif;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use std::path::Path;

const DIR: &str = "/work/project";

fn output(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

fn regions() -> Output {
    output(0, include_bytes!("fixtures/regions.sarif"), b"")
}

fn hit(path: &str, line: Option<u32>, col: Option<u32>, what: &str) -> Hit {
    Hit { path: RelPath::new(path), line, col, what: what.to_string() }
}

fn first() -> Hit {
    hit("src/a.py", Some(3), Some(5), "first")
}

fn second() -> Hit {
    hit("src/b.py", Some(7), None, "second")
}

fn spaced() -> Hit {
    hit("my dir/c.py", Some(1), Some(2), "spaced")
}

fn last() -> Hit {
    hit("z.py", Some(9), Some(1), "")
}

#[test]
fn the_checked_rule_id_keeps_its_results_with_path_line_col_and_message() {
    let hits = parse_sarif(&regions(), "R1", Path::new(DIR), Path::new(DIR));
    assert_eq!(hits, Ok(vec![first(), spaced()]));
}

#[test]
fn rule_id_falls_back_to_rule_dot_id_and_runs_are_read_in_order() {
    let hits = parse_sarif(&regions(), "R2", Path::new(DIR), Path::new(DIR));
    assert_eq!(hits, Ok(vec![second(), last()]));
}

#[test]
fn each_comma_part_of_the_check_keeps_its_rule_in_output_order() {
    let hits = parse_sarif(&regions(), "R2,R1", Path::new(DIR), Path::new(DIR));
    assert_eq!(hits, Ok(vec![first(), second(), spaced(), last()]));
}

#[test]
fn all_keeps_every_rule_but_not_a_result_with_no_rule_id_or_no_location() {
    let hits = parse_sarif(&regions(), "ALL", Path::new(DIR), Path::new(DIR));
    assert_eq!(hits, Ok(vec![first(), second(), spaced(), last()]));
}

#[test]
fn a_rule_id_must_equal_a_part_not_start_it() {
    assert_eq!(parse_sarif(&regions(), "R", Path::new(DIR), Path::new(DIR)), Ok(vec![]));
}

#[test]
fn a_result_outside_the_folder_is_skipped() {
    let hits = parse_sarif(&regions(), "R1", Path::new(DIR), Path::new("/work/project/src"));
    assert_eq!(hits, Ok(vec![hit("a.py", Some(3), Some(5), "first")]));
}

#[test]
fn a_result_with_no_region_has_no_line_and_no_col() {
    let out = output(0, include_bytes!("fixtures/no_region.sarif"), b"");
    let hits = parse_sarif(&out, "R1", Path::new(DIR), Path::new(DIR));
    assert_eq!(hits, Ok(vec![hit("f.py", None, None, "whole file")]));
}

#[test]
fn an_empty_run_has_no_hits() {
    let out = output(0, include_bytes!("fixtures/empty_run.sarif"), b"");
    assert_eq!(parse_sarif(&out, "ALL", Path::new(DIR), Path::new(DIR)), Ok(vec![]));
}

#[test]
fn a_run_with_no_results_key_has_no_hits() {
    let out = output(0, br#"{"version":"2.1.0","runs":[{"tool":{"driver":{"name":"x"}}}]}"#, b"");
    assert_eq!(parse_sarif(&out, "ALL", Path::new(DIR), Path::new(DIR)), Ok(vec![]));
}

#[test]
fn any_exit_code_with_a_sarif_log_is_an_answer() {
    let out = output(1, include_bytes!("fixtures/regions.sarif"), b"1 finding\n");
    let hits = parse_sarif(&out, "R1", Path::new(DIR), Path::new(DIR));
    assert_eq!(hits, Ok(vec![first(), spaced()]));
}

#[test]
fn a_log_with_no_runs_key_is_a_parse_error_quoting_stdout() {
    let out = output(0, br#"{"version":"2.1.0"}"#, b"");
    assert_eq!(
        parse_sarif(&out, "ALL", Path::new(DIR), Path::new(DIR)),
        Err(ParseError { why: r#"SARIF output is not a SARIF log: {"version":"2.1.0"}"#.to_string() })
    );
}

#[test]
fn output_that_is_not_json_is_a_parse_error_quoting_stderr() {
    let out = output(2, b"", b"engine: no such option\n");
    assert_eq!(
        parse_sarif(&out, "ALL", Path::new(DIR), Path::new(DIR)),
        Err(ParseError { why: "SARIF output is not a SARIF log: engine: no such option\n".to_string() })
    );
}

#[cfg(windows)]
#[test]
fn windows_drive_uris_under_a_verbatim_folder_are_kept_in_order() {
    let out = output(0, br#"{"runs":[{"results":[
        {"ruleId":"R1","locations":[{"physicalLocation":{"artifactLocation":{"uri":"file:///C:/work/project/src/a.py"}}}]},
        {"ruleId":"R1","locations":[{"physicalLocation":{"artifactLocation":{"uri":"file:///c:/work/project/src/b.py"}}}]}
    ]}]}"#, b"");
    let folder = Path::new(r"\\?\C:\work\project");
    assert_eq!(
        parse_sarif(&out, "R1", folder, folder),
        Ok(vec![hit("src/a.py", None, None, ""), hit("src/b.py", None, None, "")])
    );
}
