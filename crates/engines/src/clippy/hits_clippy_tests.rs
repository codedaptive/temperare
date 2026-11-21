//! Unit tests for hits_clippy.rs. The fake host answers clippy-driver help from a capture and cargo commands by first argument.
//! The help fixture is stdout from `clippy-driver -W help`, clippy 0.1.95 via rustup in /tmp, exit 0, empty stderr.
//! These tests reach no real Cargo tree: each stops on invalid params before any command runs.
//! Shared helpers are `pub(super)` for the real-tree integration tests.

use super::{hits_clippy, CLIPPY};
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output, ProcError};
use rules::RuleFile;
use std::cell::{Cell, RefCell};
use std::path::Path;

pub(super) const RULES: &str = "version = 1\n[[rule]]\nid = \"r\"\nengine = \"clippy\"\ncheck = \"clippy::bool_comparison\"\nfix = \"f\"\n";
/// `RULES` with a `params` line, for the tests that read params.
pub(super) fn rules_with(params: &str) -> String {
    format!("{RULES}params = {params}\n")
}

pub(super) fn output(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

/// `hits_clippy` on `folder` with a host answering clippy-driver help with the captured fixture, locate-project with
/// `located` and clippy with `linted`; returns
/// the result and every command the host was asked to run.
pub(super) fn run(folder: &Path, located: Output, linted: Output) -> (Result<Vec<Hit>, Stop>, Vec<Cmd>) {
    run_rules(RULES, folder, located, linted)
}

/// `run` with the rule file `rules` in place of `RULES`.
pub(super) fn run_rules(rules: &str, folder: &Path, located: Output, linted: Output) -> (Result<Vec<Hit>, Stop>, Vec<Cmd>) {
    run_rules_with_help(rules, folder, output(0, include_bytes!("fixtures/lint_groups_help.txt"), b""), located, linted)
}

pub(super) fn run_rules_with_help(rules: &str, folder: &Path, help: Output, located: Output, linted: Output) -> (Result<Vec<Hit>, Stop>, Vec<Cmd>) {
    let file = RuleFile::parse(rules, folder).unwrap();
    let seen = RefCell::new(Vec::new());
    let run = |cmd: &Cmd| -> Result<Output, ProcError> {
        seen.borrow_mut().push(cmd.clone());
        match cmd.program.to_str() {
            Some("clippy-driver") => Ok(help.clone()),
            Some("cargo") => match cmd.args[0].to_str() {
                Some("locate-project") => Ok(located.clone()),
                Some("clippy") => Ok(linted.clone()),
                other => panic!("unexpected cargo command {other:?}"),
            },
            other => panic!("unexpected program {other:?}"),
        }
    };
    let on_path = |_: &str| -> bool { panic!("hits_clippy looks nothing up on PATH") };
    let got = hits_clippy(&CLIPPY, &file.rules[0], folder, &Host { run: &run, on_path: &on_path, raw_chars: &Cell::new(None), base: None });
    (got, seen.into_inner())
}

#[test]
fn a_param_other_than_allow_is_failed_with_the_reason_and_nothing_runs() {
    let rules = rules_with(r#"{ alow = ["clippy::module_inception"] }"#);
    let (got, seen) = run_rules(&rules, Path::new("/engines-zut-no-such-folder/a"), output(0, b"", b""), output(0, b"", b""));
    let want = "clippy param alow: a clippy rule reads only the `allow`, `max_nesting` param".to_string();
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: want }));
    assert!(seen.is_empty());
}

#[test]
fn an_allow_that_is_not_a_list_is_failed_with_the_reason_and_nothing_runs() {
    let rules = rules_with(r#"{ allow = "clippy::module_inception" }"#);
    let (got, seen) = run_rules(&rules, Path::new("/engines-zut-no-such-folder/a"), output(0, b"", b""), output(0, b"", b""));
    let want = "clippy param allow: must be a list of strings".to_string();
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: want }));
    assert!(seen.is_empty());
}


/// `RULES` with check `check` and a `params` line.
fn rules_for(check: &str, params: &str) -> String {
    rules_with(params).replace("clippy::bool_comparison", check)
}

/// `hits_clippy` on a folder that does not exist, whose rule must stop before anything runs.
fn stopped(rules: &str) -> (Result<Vec<Hit>, Stop>, Vec<Cmd>) {
    run_rules(rules, Path::new("/engines-zut-no-such-folder/a"), output(0, b"", b""), output(0, b"", b""))
}

#[test]
fn a_max_nesting_of_0_is_failed_with_the_range_and_nothing_runs() {
    let (got, seen) = stopped(&rules_for("clippy::excessive_nesting", "{ max_nesting = 0 }"));
    let want = "clippy param max_nesting: must be a whole number from 1 to 1000".to_string();
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: want }));
    assert!(seen.is_empty());
}

#[test]
fn a_max_nesting_on_another_check_is_failed_with_its_check_and_nothing_runs() {
    let (got, seen) = stopped(&rules_for("clippy::bool_comparison", "{ max_nesting = 3 }"));
    let want = "clippy param max_nesting: read only with check `clippy::excessive_nesting`".to_string();
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: want }));
    assert!(seen.is_empty());
}

#[test]
fn an_unknown_param_beside_max_nesting_is_failed_naming_both_params_and_nothing_runs() {
    let (got, seen) = stopped(&rules_for("clippy::excessive_nesting", "{ max_nesting = 3, foo = 1 }"));
    let want = "clippy param foo: a clippy rule reads only the `allow`, `max_nesting` param".to_string();
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: want }));
    assert!(seen.is_empty());
}
