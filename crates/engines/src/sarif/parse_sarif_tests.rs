//! Unit test for parse_sarif.rs: SARIF 2.1 hits, and Windows drive URIs under a verbatim folder.
//!
//! regions.sarif: two runs. Run 1: R1 in src/a.py at 3:5 ("first", with a second location that is not read); R2 by
//! `rule.id` in relative src/b.py at line 7, no column ("second"); R1 in `my%20dir/c.py` at 1:2 ("spaced"); R4
//! outside at `file:///elsewhere/d.py`; R3 with no location; a result with no rule id. Run 2: R2 in z.py at 9:1 with
//! no message. Checking R1 or R2 also proves an outside result of an unchecked rule leaves the answer Ok.
//! no_region.sarif: R1 in f.py with no region. empty_run.sarif: one run with `"results": []`.
//! Except in the verbatim-folder test (Windows only), `cwd` and (unless a test says otherwise) `folder` are
//! `/work/project`.

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
fn a_checked_rule_with_no_location_and_a_result_with_no_rule_id_are_skipped() {
    let hits = parse_sarif(&regions(), "R1,R2,R3", Path::new(DIR), Path::new(DIR));
    assert_eq!(hits, Ok(vec![first(), second(), spaced(), last()]));
}

#[test]
fn a_rule_id_must_equal_a_part_not_start_it() {
    assert_eq!(parse_sarif(&regions(), "R", Path::new(DIR), Path::new(DIR)), Ok(vec![]));
}

#[test]
fn a_checked_result_outside_the_folder_fails_naming_its_raw_uri() {
    let hits = parse_sarif(&regions(), "R1", Path::new(DIR), Path::new("/work/project/src"));
    assert_eq!(hits, Err(ParseError {
        why: "SARIF result names `file:///work/project/my%20dir/c.py`, a path not under the rule folder `/work/project/src`".to_string(),
    }));
}

#[test]
fn all_fails_for_the_first_checked_result_outside_the_folder() {
    let hits = parse_sarif(&regions(), "ALL", Path::new(DIR), Path::new(DIR));
    assert_eq!(hits, Err(ParseError {
        why: "SARIF result names `file:///elsewhere/d.py`, a path not under the rule folder `/work/project`".to_string(),
    }));
}

#[test]
fn all_keeps_every_checked_rules_in_folder_hits_in_output_order() {
    let out = output(0, br#"{"runs":[{"results":[
        {"ruleId":"R1","message":{"text":"one"},"locations":[{"physicalLocation":{
            "artifactLocation":{"uri":"file:///work/project/a.py"},"region":{"startLine":1,"startColumn":1}}}]},
        {"ruleId":"R2","message":{"text":"two"},"locations":[{"physicalLocation":{
            "artifactLocation":{"uri":"b.py"},"region":{"startLine":2,"startColumn":3}}}]},
        {"message":{"text":"no rule"},"locations":[{"physicalLocation":{
            "artifactLocation":{"uri":"c.py"},"region":{"startLine":4,"startColumn":4}}}]}
    ]}]}"#, b"");
    let hits = parse_sarif(&out, "ALL", Path::new(DIR), Path::new(DIR));
    assert_eq!(hits, Ok(vec![hit("a.py", Some(1), Some(1), "one"), hit("b.py", Some(2), Some(3), "two")]));
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
