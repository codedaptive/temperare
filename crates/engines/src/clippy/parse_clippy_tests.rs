//! ZUT for parse_clippy.rs, on `cargo clippy --message-format=json --quiet -- -W clippy::bool_comparison` output
//! captured once from tiny crates: `lint_and_other.jsonl` (exit 0; a bool_comparison at src/lib.rs:3:5 and an
//! unused_variables warning at src/lib.rs:2:9), `clean.jsonl` (exit 0; no messages) and `build_failed.jsonl`
//! (exit 101; an E0425 error and build-finished success false). The capture crate's folder was `CAPTURED`.
//! `CLIPPY`'s command passes `--warn=<check>` and `--allow=<lint>` (S-35); clippy reads both spellings the same way,
//! so the captures stand for the run it builds.
//! `all_targets_warnings.jsonl` was not captured: no capture folder was available, so it is hand-built in the shape of
//! a `cargo clippy --all-targets` capture of a crate in `ALL_TARGETS`. Its compiler-messages: from the lib target an
//! unused_variables warning at src/lib.rs:2:9 and a clippy::bool_comparison at src/lib.rs:3:5; the same two again from
//! the lib-test target; from the lib-test target an unused_variables warning at src/lib.rs:8:13 (in `#[cfg(test)]`
//! code); a codeless "3 warnings emitted" warning with no spans; then build-finished success true.
//! unknown_lint.jsonl and dashed_and_bare_group.jsonl are hand-built in the reported clippy 0.1.95 shape.
//! renamed_allowed_check.jsonl is captured from clippy 0.1.95, exit 0, with
//! `cargo clippy --all-targets --message-format=json --quiet -- --warn=clippy::blocks_in_conditions --allow=clippy::block_in_if_condition_expr`.
//! Its `/private/tmp/engines-kong-2bf3-probe` path prefix is replaced by `/work/project`.
//! The other captured fixtures' home-folder paths are replaced by `/work/project`, so those paths, `CAPTURED`,
//! `ALL_TARGETS` and the absolute paths in `all_targets_warnings.jsonl` all start with it.

use super::parse_clippy;
use crate::types::Hit;
use procio::Output;
use rules::RelPath;
use std::path::Path;

const CAPTURED: &str = "/work/project/clippy-capture/a";
const LINT_AND_OTHER: &[u8] = include_bytes!("fixtures/lint_and_other.jsonl");
const CLEAN: &[u8] = include_bytes!("fixtures/clean.jsonl");
const BUILD_FAILED: &[u8] = include_bytes!("fixtures/build_failed.jsonl");
const ALL_TARGETS: &str = "/work/project/clippy-capture-all-targets";
const ALL_TARGETS_WARNINGS: &[u8] = include_bytes!("fixtures/all_targets_warnings.jsonl");

fn output(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

fn hit(path: &str, line: u32, col: u32, what: &str) -> Hit {
    Hit { path: RelPath::new(path), line: Some(line), col: Some(col), what: what.to_string() }
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
    assert_eq!(why, r#"clippy: the build failed: "error: could not compile `cap_c` (lib) due to 1 previous error\n""#);
}

#[test]
fn output_with_no_build_finished_record_is_a_parse_error() {
    let cut: Vec<u8> = LINT_AND_OTHER
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.windows(16).any(|w| w == b"\"build-finished\""))
        .flat_map(|line| line.iter().copied().chain([b'\n']))
        .collect();
    let got = parse_clippy(&output(0, &cut, b""), "clippy::bool_comparison", Path::new(CAPTURED), Path::new(CAPTURED));
    assert_eq!(got.unwrap_err().why, r##"clippy: no build-finished record: "{\"reason\":\"compiler-message\",\"package_id\":\"path+file:///work/project/clippy-capture/a#cap_a@0.1.0\",\"manifest_path\":\"/work/project/clippy-capture/a/Cargo.toml\",\"target\":{\"kind\":[\"lib\"],\"crate_types\":[\"""##);
}

#[test]
fn a_non_json_line_is_a_parse_error_quoting_stdout_when_stderr_is_empty() {
    let stdout = [b"warning: not json\n", CLEAN].concat();
    let got = parse_clippy(&output(0, &stdout, b""), "clippy::bool_comparison", Path::new(CAPTURED), Path::new(CAPTURED));
    let why = got.unwrap_err().why;
    assert_eq!(why, r##"clippy: output is not JSON lines: "warning: not json\n{\"reason\":\"compiler-artifact\",\"package_id\":\"path+file:///work/project/clippy-capture/b#cap_b@0.1.0\",\"manifest_path\":\"/work/project/clippy-capture/b/Cargo.toml\",\"target\":{\"kind\":[\"lib""##);
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

#[test]
fn warnings_keeps_every_coded_warning_once_with_its_code_in_front() {
    let got = parse_clippy(&output(0, ALL_TARGETS_WARNINGS, b""), "warnings", Path::new(ALL_TARGETS), Path::new(ALL_TARGETS));
    assert_eq!(
        got,
        Ok(vec![
            hit("src/lib.rs", 2, 9, "unused_variables: unused variable: `unused`"),
            hit("src/lib.rs", 3, 5, "clippy::bool_comparison: equality checks against true are unnecessary"),
            hit("src/lib.rs", 8, 13, "unused_variables: unused variable: `test_unused`"),
        ])
    );
}

#[test]
fn one_lint_over_all_targets_is_one_hit_with_the_message_unchanged() {
    let got = parse_clippy(&output(0, ALL_TARGETS_WARNINGS, b""), "clippy::bool_comparison", Path::new(ALL_TARGETS), Path::new(ALL_TARGETS));
    assert_eq!(got, Ok(vec![hit("src/lib.rs", 3, 5, "equality checks against true are unnecessary")]));
}

const ERROR_AND_NOTE: &str = concat!(
    r#"{"reason":"compiler-message","message":{"message":"cannot find value","level":"error","code":{"code":"E0425","explanation":null},"spans":[{"file_name":"a.rs","line_start":1,"column_start":2,"is_primary":true}]}}"#,
    "\n",
    r#"{"reason":"compiler-message","message":{"message":"a note","level":"note","code":{"code":"unused_variables","explanation":null},"spans":[{"file_name":"a.rs","line_start":3,"column_start":4,"is_primary":true}]}}"#,
    "\n",
    r#"{"reason":"build-finished","success":true}"#,
    "\n"
);

#[test]
fn warnings_skips_error_and_note_levels_even_with_a_code() {
    let got = parse_clippy(&output(0, ERROR_AND_NOTE.as_bytes(), b""), "warnings", Path::new("/w/crate"), Path::new("/w/crate"));
    assert_eq!(got, Ok(vec![]));
}

#[test]
fn one_lint_keeps_its_code_at_error_level() {
    let got = parse_clippy(&output(0, ERROR_AND_NOTE.as_bytes(), b""), "E0425", Path::new("/w/crate"), Path::new("/w/crate"));
    assert_eq!(got, Ok(vec![hit("a.rs", 1, 2, "cannot find value")]));
}

#[test]
fn warnings_skips_a_codeless_warning_that_has_a_primary_span() {
    let stdout = concat!(
        r#"{"reason":"compiler-message","message":{"message":"no code here","level":"warning","code":null,"spans":[{"file_name":"a.rs","line_start":5,"column_start":6,"is_primary":true}]}}"#,
        "\n",
        r#"{"reason":"build-finished","success":true}"#,
        "\n"
    );
    let got = parse_clippy(&output(0, stdout.as_bytes(), b""), "warnings", Path::new("/w/crate"), Path::new("/w/crate"));
    assert_eq!(got, Ok(vec![]));
}

#[test]
fn d1_clippy_unknown_requested_lint_is_an_error() {
    let dir = Path::new("/private/tmp/d1-clippy-probe");
    let out = output(0, include_bytes!("fixtures/unknown_lint.jsonl"), b"");
    let got = parse_clippy(&out, "clippy::bool_comparisn", dir, dir);
    assert_eq!(got.unwrap_err().why, "clippy did not take check `clippy::bool_comparisn` whole: unknown lint: `clippy::bool_comparisn`");
}

#[test]
fn d1_clippy_dashed_lint_matches_underscored_diagnostics() {
    let dir = Path::new("/private/tmp/d1-clippy-probe");
    let out = output(0, include_bytes!("fixtures/dashed_and_bare_group.jsonl"), b"");
    assert_eq!(parse_clippy(&out, "clippy::bool-comparison", dir, dir), Ok(vec![hit("src/lib.rs", 1, 29, "equality checks against true are unnecessary")]));
}

#[test]
fn d1_clippy_deprecated_bare_group_is_an_error() {
    let dir = Path::new("/private/tmp/d1-clippy-probe");
    let out = output(0, include_bytes!("fixtures/dashed_and_bare_group.jsonl"), b"");
    let got = parse_clippy(&out, "style", dir, dir);
    assert_eq!(got.unwrap_err().why, "clippy did not take check `style` whole: lint name `style` is deprecated and may not have an effect in the future. Use `clippy::style` instead");
}

#[test]
fn d1_clippy_unknown_other_lint_does_not_hide_requested_lint() {
    let dir = Path::new("/private/tmp/d1-clippy-probe");
    let out = output(0, include_bytes!("fixtures/unknown_lint.jsonl"), b"");
    assert_eq!(parse_clippy(&out, "clippy::bool_comparison", dir, dir), Ok(vec![hit("src/lib.rs", 1, 29, "equality checks against true are unnecessary")]));
}

// Hand-built compiler messages in the shapes of unknown_lint.jsonl and renamed_allowed_check.jsonl.
#[test]
fn dashed_unknown_check_is_refused_when_diagnostic_uses_underscores() {
    let stdout = concat!(
        r#"{"reason":"compiler-message","message":{"message":"unknown lint: `clippy::bool_comparisn`","level":"warning","code":{"code":"E0602"},"spans":[]}}"#,
        "\n",
        r#"{"reason":"build-finished","success":true}"#,
        "\n"
    );
    let dir = Path::new(CAPTURED);
    let got = parse_clippy(&output(0, stdout.as_bytes(), b""), "clippy::bool-comparisn", dir, dir);
    assert_eq!(got.unwrap_err().why, "clippy did not take check `clippy::bool-comparisn` whole: unknown lint: `clippy::bool_comparisn`");
}

#[test]
fn dashed_renamed_check_is_refused_when_diagnostic_uses_underscores() {
    let stdout = concat!(
        r#"{"reason":"compiler-message","message":{"message":"lint `clippy::block_in_if_condition_expr` has been renamed to `clippy::blocks_in_conditions`","level":"warning","code":{"code":"renamed_and_removed_lints"},"spans":[]}}"#,
        "\n",
        r#"{"reason":"build-finished","success":true}"#,
        "\n"
    );
    let dir = Path::new(CAPTURED);
    let got = parse_clippy(&output(0, stdout.as_bytes(), b""), "clippy::block-in-if-condition-expr", dir, dir);
    assert_eq!(got.unwrap_err().why, "clippy did not take check `clippy::block-in-if-condition-expr` whole: lint `clippy::block_in_if_condition_expr` has been renamed to `clippy::blocks_in_conditions`");
}

#[test]
fn renamed_allowed_lint_does_not_refuse_its_new_name() {
    let dir = Path::new("/work/project");
    let out = output(0, include_bytes!("fixtures/renamed_allowed_check.jsonl"), b"");
    assert_eq!(parse_clippy(&out, "clippy::blocks_in_conditions", dir, dir), Ok(vec![]));
}

#[test]
fn renamed_allowed_lint_refuses_its_old_name() {
    let dir = Path::new("/work/project");
    let out = output(0, include_bytes!("fixtures/renamed_allowed_check.jsonl"), b"");
    let got = parse_clippy(&out, "clippy::block_in_if_condition_expr", dir, dir);
    assert_eq!(got.unwrap_err().why, "clippy did not take check `clippy::block_in_if_condition_expr` whole: lint `clippy::block_in_if_condition_expr` has been renamed to `clippy::blocks_in_conditions`");
}

// Hand-built from the message clippy 0.1.95 prints for `--allow=clippy::should_assert_eq` (a removed lint), cut to the fields `parse_clippy` reads.
const REMOVED_LINT: &str = concat!(
    r#"{"reason":"compiler-message","message":{"message":"lint `clippy::should_assert_eq` has been removed: `assert!(a == b)` can now print the values the same way `assert_eq!(a, b) can","level":"warning","code":{"code":"renamed_and_removed_lints","explanation":null},"spans":[]}}"#,
    "\n",
    r#"{"reason":"build-finished","success":true}"#,
    "\n"
);

#[test]
fn removed_lint_reason_does_not_refuse_later_quoted_text() {
    let dir = Path::new("/work/project");
    let got = parse_clippy(&output(0, REMOVED_LINT.as_bytes(), b""), "assert!(a == b)", dir, dir);
    assert_eq!(got, Ok(vec![]));
}

#[test]
fn removed_lint_refuses_its_own_name() {
    let dir = Path::new("/work/project");
    let got = parse_clippy(&output(0, REMOVED_LINT.as_bytes(), b""), "clippy::should_assert_eq", dir, dir);
    assert_eq!(got.unwrap_err().why, "clippy did not take check `clippy::should_assert_eq` whole: lint `clippy::should_assert_eq` has been removed: `assert!(a == b)` can now print the values the same way `assert_eq!(a, b) can");
}
