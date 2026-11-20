//! ZUT for hits_ruff.rs: the command the host is given, findings read from captured output, garbage output, an
//! absent ruff, and the characters of the check run counted into the host. The host is a fake; nothing runs. `DIR` is the folder path the captured fixture's
//! absolute paths start with (see parse_ruff_tests.rs); no folder is read, so it need not exist.

use super::hits_ruff;
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output, ProcError};
use rules::{RelPath, RuleFile};
use std::cell::{Cell, RefCell};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

const DIR: &str = "/work/project/cap-ruff";

const WITH_CONFIG: &str = "version = 1\n[[rule]]\nid = \"no-none-compare\"\nengine = \"ruff\"\ncheck = \"E711\"\nconfig = \"ruff.toml\"\nfix = \"f\"\n";
const NO_CONFIG: &str = "version = 1\n[[rule]]\nid = \"no-none-compare\"\nengine = \"ruff\"\ncheck = \"E711\"\nfix = \"f\"\n";

/// `hits_ruff` for the one rule in `text`, over `DIR`, with a host whose run answers `answer`; also the command the
/// run was given, if it was asked.
fn run_rule(text: &str, answer: impl Fn() -> Result<Output, ProcError>) -> (Result<Vec<Hit>, Stop>, Option<Cmd>) {
    let file = RuleFile::parse(text, Path::new(DIR)).unwrap();
    let seen = RefCell::new(None);
    let run = |cmd: &Cmd| {
        *seen.borrow_mut() = Some(cmd.clone());
        answer()
    };
    let on_path = |_: &str| -> bool { panic!("hits_ruff looks nothing up on PATH") };
    let got = hits_ruff(&file.rules[0], Path::new(DIR), &Host { run: &run, on_path: &on_path, raw_chars: &Cell::new(None) });
    (got, seen.into_inner())
}

/// `hits_ruff` for the one rule in `text`, as `run_rule` runs it; also the count the host holds after it and how many
/// runs the host was asked for.
fn run_counted(
    text: &str,
    answer: impl Fn() -> Result<Output, ProcError>,
) -> (Result<Vec<Hit>, Stop>, Option<usize>, usize) {
    let file = RuleFile::parse(text, Path::new(DIR)).unwrap();
    let runs = Cell::new(0);
    let run = |_: &Cmd| {
        runs.set(runs.get() + 1);
        answer()
    };
    let on_path = |_: &str| -> bool { panic!("hits_ruff looks nothing up on PATH") };
    let raw_chars = Cell::new(None);
    let got = hits_ruff(&file.rules[0], Path::new(DIR), &Host { run: &run, on_path: &on_path, raw_chars: &raw_chars });
    (got, raw_chars.get(), runs.get())
}

fn captured() -> Result<Output, ProcError> {
    let stdout = include_bytes!("fixtures/e711_and_f401.json").to_vec();
    Ok(Output { code: Some(1), stdout, stderr: Vec::new() })
}

fn os(items: &[&str]) -> Vec<OsString> {
    items.iter().map(OsString::from).collect()
}

#[test]
fn the_checked_rules_findings_come_through() {
    let (got, _) = run_rule(WITH_CONFIG, captured);
    let want = Hit {
        path: RelPath::new("example.py"),
        line: 3,
        col: Some(13),
        what: "Comparison to `None` should be `cond is None`".to_string(),
    };
    assert_eq!(got, Ok(vec![want]));
}

#[test]
fn the_rules_config_is_passed_and_ruff_runs_in_the_folder() {
    let cmd = run_rule(WITH_CONFIG, captured).1.unwrap();
    assert_eq!(
        cmd.args,
        os(&["check", "--no-cache", "--output-format", "json", "--select=E711", "--config=ruff.toml", "."])
    );
    assert_eq!(cmd.cwd, Some(PathBuf::from(DIR)));
}

#[test]
fn a_rule_with_no_config_passes_none() {
    let cmd = run_rule(NO_CONFIG, captured).1.unwrap();
    assert_eq!(cmd.args, os(&["check", "--no-cache", "--output-format", "json", "--select=E711", "."]));
}

#[test]
fn garbage_output_is_failed_with_the_exit_code_and_stderr() {
    // Hand-built, not captured.
    let (got, _) = run_rule(NO_CONFIG, || {
        Ok(Output { code: Some(2), stdout: b"garbage".to_vec(), stderr: b"ruff failed: bad config".to_vec() })
    });
    assert_eq!(got, Err(Stop::Failed { code: Some(2), stderr: "ruff failed: bad config".to_string() }));
}

#[test]
fn ruff_not_found_is_not_available() {
    let (got, _) = run_rule(NO_CONFIG, || Err(ProcError::NotFound { program: "ruff".into() }));
    assert_eq!(got, Err(Stop::NotAvailable { why: "ruff not found on PATH".to_string() }));
}

#[test]
fn d1_ruff_named_category_is_refused_without_running() {
    let rules = "version = 1\n[[rule]]\nid = \"r\"\nengine = \"ruff\"\ncheck = \"flake8-bugbear\"\nfix = \"f\"\n";
    let (got, seen) = run_rule(rules, captured);
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: "ruff check `flake8-bugbear`: `flake8-bugbear` is not a rule code, a code prefix or ALL, so tm cannot tell which findings it selects".to_string() }));
    assert!(seen.is_none());
}

#[test]
fn d1_ruff_mixed_invalid_selector_is_refused_without_running() {
    let rules = "version = 1\n[[rule]]\nid = \"r\"\nengine = \"ruff\"\ncheck = \"E7,correctness\"\nfix = \"f\"\n";
    let (got, seen) = run_rule(rules, captured);
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: "ruff check `E7,correctness`: `correctness` is not a rule code, a code prefix or ALL, so tm cannot tell which findings it selects".to_string() }));
    assert!(seen.is_none());
}

#[test]
fn the_check_run_counts_once_with_the_characters_of_stdout_and_stderr() {
    // "[]" is 2 characters; "warning: café\n" is 14 characters in 15 bytes.
    let (got, chars, runs) = run_counted(NO_CONFIG, || {
        Ok(Output { code: Some(0), stdout: b"[]".to_vec(), stderr: "warning: café\n".as_bytes().to_vec() })
    });
    assert_eq!(got, Ok(Vec::new()));
    assert_eq!((chars, runs), (Some(16), 1));
}

#[test]
fn a_run_failed_on_its_output_still_counts() {
    // "garbage" is 7 characters, "ruff failed: bad config" 23.
    let (got, chars, _) = run_counted(NO_CONFIG, || {
        Ok(Output { code: Some(2), stdout: b"garbage".to_vec(), stderr: b"ruff failed: bad config".to_vec() })
    });
    assert_eq!(got, Err(Stop::Failed { code: Some(2), stderr: "ruff failed: bad config".to_string() }));
    assert_eq!(chars, Some(30));
}

#[test]
fn a_check_refused_before_its_run_counts_nothing() {
    let rules = "version = 1\n[[rule]]\nid = \"r\"\nengine = \"ruff\"\ncheck = \"flake8-bugbear\"\nfix = \"f\"\n";
    let (_, chars, runs) = run_counted(rules, captured);
    assert_eq!((chars, runs), (None, 0));
}

#[test]
fn ruff_not_found_counts_nothing() {
    let (_, chars, _) = run_counted(NO_CONFIG, || Err(ProcError::NotFound { program: "ruff".into() }));
    assert_eq!(chars, None);
}
