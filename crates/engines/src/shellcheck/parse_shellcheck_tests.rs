//! Unit tests for parse_shellcheck.rs: real shellcheck 0.11.0 output, captured once on throwaway scripts, read into hits.
//!
//! sc2086.json: `shellcheck -f json1 --enable=all -- quote.sh`, exit 1.
//! clean.json: the same command on a clean script, exit 0.
//! several.json: `shellcheck -f json1 --enable=all -- several.sh`, exit 1.
//! unparsed_script.json: `shellcheck -f json1 --enable=all -- broken.sh quote.sh`, exit 1; broken.sh does not parse.
//! missing_file.stderr: stderr of a run on a file that does not exist; its stdout was `{"comments":[]}`, exit 2.
//! bad_option.stderr: stderr of a run with an unknown option; its stdout was empty, exit 3.
//! rcfile_missing.stderr: stderr of a run with `--rcfile=nope.rc`, a file that does not exist; its stdout was
//! identical to sc2086.json, exit 1.
//! The scripts' paths are printed relative to where shellcheck ran, so `DIR` stands for that folder.

use super::parse_shellcheck;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use std::path::Path;

const DIR: &str = "/work/project/cap-shellcheck";

fn output(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

fn read(out: &Output, check: &str) -> Result<Vec<Hit>, ParseError> {
    parse_shellcheck(out, check, Path::new(DIR), Path::new(DIR))
}

fn sc2086() -> Output {
    output(1, include_bytes!("fixtures/sc2086.json"), b"")
}

fn several() -> Output {
    output(1, include_bytes!("fixtures/several.json"), b"")
}

fn unparsed() -> Output {
    output(1, include_bytes!("fixtures/unparsed_script.json"), b"")
}

fn hit(path: &str, line: u32, col: u32, what: &str) -> Hit {
    Hit { path: RelPath::new(path), line: Some(line), col: Some(col), what: what.to_string() }
}

#[test]
fn only_the_checked_code_becomes_a_hit() {
    assert_eq!(read(&sc2086(), "SC2086"), Ok(vec![hit("quote.sh", 3, 6, "Double quote to prevent globbing and word splitting.")]));
}

#[test]
fn another_code_is_read_when_it_is_the_check() {
    assert_eq!(
        read(&sc2086(), "SC2250"),
        Ok(vec![hit("quote.sh", 3, 6, "Prefer putting braces around variable references even when not strictly required.")])
    );
}

#[test]
fn a_code_with_no_comment_has_no_hits() {
    assert_eq!(read(&sc2086(), "SC9999"), Ok(vec![]));
}

#[test]
fn a_clean_run_has_no_hits() {
    assert_eq!(read(&output(0, include_bytes!("fixtures/clean.json"), b""), "SC2086"), Ok(vec![]));
}

#[test]
fn each_check_of_several_gets_its_own_hit() {
    assert_eq!(
        read(&several(), "SC2164"),
        Ok(vec![hit("several.sh", 2, 1, "Use 'cd ... || exit' or 'cd ... || return' in case cd fails.")])
    );
    assert_eq!(
        read(&several(), "SC2034"),
        Ok(vec![hit("several.sh", 6, 6, "line appears unused. Verify use (or export if used externally).")])
    );
}

#[test]
fn an_error_level_comment_that_is_no_parse_code_is_a_hit() {
    assert_eq!(read(&several(), "SC2045"), Ok(vec![hit("several.sh", 3, 10, "Iterating over ls output is fragile. Use globs.")]));
}

#[test]
fn a_comment_on_a_file_outside_the_folder_is_not_a_hit() {
    let folder = Path::new(DIR).join("sub");
    assert_eq!(parse_shellcheck(&sc2086(), "SC2086", Path::new(DIR), &folder), Ok(vec![]));
}

#[test]
fn a_script_shellcheck_could_not_parse_fails_the_run() {
    assert_eq!(
        read(&unparsed(), "SC2086"),
        Err(ParseError {
            why: "shellcheck could not parse broken.sh, so it was not checked: SC1073 Couldn't parse this double quoted string. Fix to allow more checks.".to_string()
        })
    );
}

#[test]
fn a_parse_code_as_the_check_is_read_like_any_other() {
    assert_eq!(
        read(&unparsed(), "SC1073"),
        Ok(vec![hit("broken.sh", 3, 8, "Couldn't parse this double quoted string. Fix to allow more checks.")])
    );
}

#[test]
fn exit_2_with_empty_comments_is_not_an_answer() {
    let out = output(2, b"{\"comments\":[]}", include_bytes!("fixtures/missing_file.stderr"));
    assert_eq!(
        read(&out, "SC2086"),
        Err(ParseError {
            why: "shellcheck exited 2, which is not an answer: tm-no-such-file-xyz.sh: tm-no-such-file-xyz.sh: openBinaryFile: does not exist (No such file or directory)\n".to_string()
        })
    );
}

#[test]
fn a_bad_option_is_not_an_answer() {
    let out = output(3, b"", include_bytes!("fixtures/bad_option.stderr"));
    let why = read(&out, "SC2086").unwrap_err().why;
    assert!(why.starts_with("shellcheck exited 3, which is not an answer: unrecognized option `--bogus-flag'\n"), "{why}");
}

#[test]
fn a_run_ended_by_a_signal_is_not_an_answer() {
    let out = Output { code: None, stdout: b"{\"comments\":[]}".to_vec(), stderr: b"killed".to_vec() };
    assert_eq!(read(&out, "SC2086"), Err(ParseError { why: "shellcheck exited on a signal, which is not an answer: killed".to_string() }));
}

#[test]
fn an_unread_rcfile_fails_the_run_though_stdout_has_findings() {
    let out = output(1, include_bytes!("fixtures/sc2086.json"), include_bytes!("fixtures/rcfile_missing.stderr"));
    assert_eq!(
        read(&out, "SC2086"),
        Err(ParseError {
            why: "shellcheck did not read the rule's config, so it checked other settings: Warning: unable to read --rcfile nope.rc".to_string()
        })
    );
}

#[test]
fn stdout_that_is_not_json_is_an_error() {
    assert_eq!(
        read(&output(0, b"not json", b""), "SC2086"),
        Err(ParseError { why: "shellcheck printed no JSON findings: not json".to_string() })
    );
}
