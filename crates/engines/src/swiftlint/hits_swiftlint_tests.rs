//! ZUT for hits_swiftlint.rs: the command the host is given, findings read from captured output, garbage output and
//! an absent swiftlint. The host is a fake; nothing runs. `DIR` is the folder path the captured fixture's
//! absolute paths start with (see parse_swiftlint_tests.rs); no folder is read, so it need not exist.

use super::hits_swiftlint;
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output, ProcError};
use rules::{RelPath, RuleFile};
use std::cell::RefCell;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

const DIR: &str = "/Users/bob/devlop/team-temperare/workcrews/engines/scratchpad/cap-swiftlint";

const WITH_CONFIG: &str = "version = 1\n[[rule]]\nid = \"no-force-cast\"\nengine = \"swiftlint\"\ncheck = \"force_cast\"\nconfig = \"two.yml\"\nfix = \"f\"\n";
const NO_CONFIG: &str = "version = 1\n[[rule]]\nid = \"no-force-cast\"\nengine = \"swiftlint\"\ncheck = \"force_cast\"\nfix = \"f\"\n";

/// `hits_swiftlint` for the one rule in `text`, over `DIR`, with a host whose run answers `answer`; also the command
/// the run was given, if it was asked.
fn run_rule(text: &str, answer: impl Fn() -> Result<Output, ProcError>) -> (Result<Vec<Hit>, Stop>, Option<Cmd>) {
    let file = RuleFile::parse(text, Path::new(DIR)).unwrap();
    let seen = RefCell::new(None);
    let run = |cmd: &Cmd| {
        *seen.borrow_mut() = Some(cmd.clone());
        answer()
    };
    let on_path = |_: &str| -> bool { panic!("hits_swiftlint looks nothing up on PATH") };
    let got = hits_swiftlint(&file.rules[0], Path::new(DIR), &Host { run: &run, on_path: &on_path });
    (got, seen.into_inner())
}

fn captured() -> Result<Output, ProcError> {
    let stdout = include_bytes!("fixtures/force_cast_and_others.json").to_vec();
    Ok(Output { code: Some(2), stdout, stderr: Vec::new() })
}

fn os(items: &[&str]) -> Vec<OsString> {
    items.iter().map(OsString::from).collect()
}

#[test]
fn the_checked_rules_findings_come_through() {
    let (got, _) = run_rule(WITH_CONFIG, captured);
    let hit = |path: &str, line: u32, col: u32| Hit {
        path: RelPath::new(path),
        line,
        col: Some(col),
        what: "Force casts should be avoided".to_string(),
    };
    assert_eq!(got, Ok(vec![hit("Example.swift", 2, 22), hit("NoNewline.swift", 1, 11)]));
}

#[test]
fn the_rules_config_is_passed_and_swiftlint_runs_in_the_folder() {
    let cmd = run_rule(WITH_CONFIG, captured).1.unwrap();
    assert_eq!(
        cmd.args,
        os(&["lint", "--reporter", "json", "--quiet", "--only-rule", "force_cast", "--config", "two.yml"])
    );
    assert_eq!(cmd.cwd, Some(PathBuf::from(DIR)));
}

#[test]
fn a_rule_with_no_config_passes_none() {
    let cmd = run_rule(NO_CONFIG, captured).1.unwrap();
    assert_eq!(cmd.args, os(&["lint", "--reporter", "json", "--quiet", "--only-rule", "force_cast"]));
}

#[test]
fn garbage_output_is_failed_with_the_exit_code_and_stderr() {
    // Hand-built, not captured.
    let (got, _) = run_rule(NO_CONFIG, || {
        Ok(Output { code: Some(1), stdout: b"garbage".to_vec(), stderr: b"Error: bad config".to_vec() })
    });
    assert_eq!(got, Err(Stop::Failed { code: Some(1), stderr: "Error: bad config".to_string() }));
}

#[test]
fn swiftlint_not_found_is_not_available() {
    let (got, _) = run_rule(NO_CONFIG, || Err(ProcError::NotFound { program: "swiftlint".into() }));
    assert_eq!(got, Err(Stop::NotAvailable { why: "swiftlint not found on PATH".to_string() }));
}
