//! ZUT for parse_clippy.rs, on `cargo clippy --message-format=json --quiet -- -W clippy::bool_comparison` output
//! captured once from tiny crates: `lint_and_other.jsonl` (exit 0; a bool_comparison at src/lib.rs:3:5 and an
//! unused_variables warning at src/lib.rs:2:9), `clean.jsonl` (exit 0; no messages) and `build_failed.jsonl`
//! (exit 101; an E0425 error and build-finished success false). The capture crate's folder was `CAPTURED`.

use super::parse_clippy;
use crate::types::Hit;
use procio::Output;
use rules::RelPath;
use std::path::Path;

const CAPTURED: &str = "/Users/bob/devlop/team-temperare/workcrews/engines/scratchpad/clippy-capture/a";
const LINT_AND_OTHER: &[u8] = include_bytes!("fixtures/lint_and_other.jsonl");
const CLEAN: &[u8] = include_bytes!("fixtures/clean.jsonl");
const BUILD_FAILED: &[u8] = include_bytes!("fixtures/build_failed.jsonl");

fn output(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

fn hit(path: &str, line: u32, col: u32, what: &str) -> Hit {
    Hit { path: RelPath::new(path), line, col: Some(col), what: what.to_string() }
}

#[test]
fn only_the_requested_lint_becomes_a_hit() {
    let got = parse_clippy(&output(0, LINT_AND_OTHER, b""), "clippy::bool_comparison", Path::new(CAPTURED), Path::new(CAPTURED));
    assert_eq!(got, Ok(vec![hit("src/lib.rs", 3, 5, "equality checks against true are unnecessary")]));
}

#[test]
fn another_check_picks_its_own_message() {
    let got = parse_clippy(&output(0, LINT_AND_OTHER, b""), "unused_variables", Path::new(CAPTURED), Path::new(CAPTURED));
    assert_eq!(got, Ok(vec![hit("src/lib.rs", 2, 9, "unused variable: `unused`")]));
}

#[test]
fn a_clean_run_is_an_empty_answer() {
    let got = parse_clippy(&output(0, CLEAN, b""), "clippy::bool_comparison", Path::new(CAPTURED), Path::new(CAPTURED));
    assert_eq!(got, Ok(vec![]));
}

#[test]
fn paths_are_made_relative_to_a_folder_below_the_cargo_root() {
    let folder = format!("{CAPTURED}/src");
    let got = parse_clippy(&output(0, LINT_AND_OTHER, b""), "clippy::bool_comparison", Path::new(CAPTURED), Path::new(&folder));
    assert_eq!(got, Ok(vec![hit("lib.rs", 3, 5, "equality checks against true are unnecessary")]));
}

#[test]
fn a_message_outside_the_folder_is_skipped() {
    let folder = format!("{CAPTURED}/tests");
    let got = parse_clippy(&output(0, LINT_AND_OTHER, b""), "clippy::bool_comparison", Path::new(CAPTURED), Path::new(&folder));
    assert_eq!(got, Ok(vec![]));
}

#[test]
fn a_failed_build_is_a_parse_error_quoting_stderr() {
    let stderr = b"error: could not compile `cap_c` (lib) due to 1 previous error\n";
    let got = parse_clippy(&output(101, BUILD_FAILED, stderr), "E0425", Path::new(CAPTURED), Path::new(CAPTURED));
    let why = got.unwrap_err().why;
    assert!(why.starts_with("clippy"), "{why}");
    assert!(why.contains("error: could not compile `cap_c` (lib) due to 1 previous error"), "{why}");
}

#[test]
fn output_with_no_build_finished_record_is_a_parse_error() {
    let cut: Vec<u8> = LINT_AND_OTHER
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.windows(16).any(|w| w == b"\"build-finished\""))
        .flat_map(|line| line.iter().copied().chain([b'\n']))
        .collect();
    let got = parse_clippy(&output(0, &cut, b""), "clippy::bool_comparison", Path::new(CAPTURED), Path::new(CAPTURED));
    assert!(got.unwrap_err().why.starts_with("clippy"));
}

#[test]
fn a_non_json_line_is_a_parse_error_quoting_stdout_when_stderr_is_empty() {
    let stdout = [b"warning: not json\n", CLEAN].concat();
    let got = parse_clippy(&output(0, &stdout, b""), "clippy::bool_comparison", Path::new(CAPTURED), Path::new(CAPTURED));
    let why = got.unwrap_err().why;
    assert!(why.starts_with("clippy"), "{why}");
    assert!(why.contains("warning: not json"), "{why}");
}

#[test]
fn the_quote_is_the_first_200_bytes() {
    let stderr = [vec![b'x'; 200], vec![b'z'; 100]].concat();
    let got = parse_clippy(&output(1, b"not json\n", &stderr), "clippy::bool_comparison", Path::new(CAPTURED), Path::new(CAPTURED));
    let why = got.unwrap_err().why;
    assert!(why.contains(&"x".repeat(200)), "{why}");
    assert!(!why.contains('z'), "{why}");
}

#[test]
fn a_message_with_no_primary_span_is_skipped() {
    let stdout = concat!(
        r#"{"reason":"compiler-message","message":{"message":"no span","code":{"code":"clippy::bool_comparison","explanation":null},"spans":[]}}"#,
        "\n",
        r#"{"reason":"build-finished","success":true}"#,
        "\n"
    );
    let got = parse_clippy(&output(0, stdout.as_bytes(), b""), "clippy::bool_comparison", Path::new(CAPTURED), Path::new(CAPTURED));
    assert_eq!(got, Ok(vec![]));
}

#[test]
fn hits_keep_output_order() {
    let stdout = concat!(
        r#"{"reason":"compiler-message","message":{"message":"second","code":{"code":"c","explanation":null},"spans":[{"file_name":"b.rs","line_start":9,"column_start":1,"is_primary":true}]}}"#,
        "\n",
        r#"{"reason":"compiler-message","message":{"message":"first","code":{"code":"c","explanation":null},"spans":[{"file_name":"a.rs","line_start":1,"column_start":2,"is_primary":false},{"file_name":"a.rs","line_start":4,"column_start":7,"is_primary":true}]}}"#,
        "\n",
        r#"{"reason":"build-finished","success":true}"#,
        "\n"
    );
    let got = parse_clippy(&output(0, stdout.as_bytes(), b""), "c", Path::new("/w/crate"), Path::new("/w/crate"));
    assert_eq!(got, Ok(vec![hit("b.rs", 9, 1, "second"), hit("a.rs", 4, 7, "first")]));
}

#[test]
fn a_non_zero_exit_with_parseable_output_is_an_answer() {
    let got = parse_clippy(&output(1, LINT_AND_OTHER, b""), "clippy::bool_comparison", Path::new(CAPTURED), Path::new(CAPTURED));
    assert_eq!(got, Ok(vec![hit("src/lib.rs", 3, 5, "equality checks against true are unnecessary")]));
}
