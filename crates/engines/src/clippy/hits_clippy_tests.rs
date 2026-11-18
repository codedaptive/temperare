//! ZUT for hits_clippy.rs. The fake host answers clippy-driver help from a capture and cargo commands by first argument.
//! The help fixture is stdout from `clippy-driver -W help`, clippy 0.1.95 via rustup in /tmp, exit 0, empty stderr.
//! These tests reach no real Cargo tree: each stops before any command runs (no `Cargo.toml` or invalid params).
//! Shared helpers are `pub(super)` for the real-tree integration tests.

use super::hits_clippy;
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output, ProcError};
use rules::RuleFile;
use std::cell::RefCell;
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
    let got = hits_clippy(&file.rules[0], folder, &Host { run: &run, on_path: &on_path });
    (got, seen.into_inner())
}

#[test]
fn no_cargo_toml_at_or_above_the_folder_is_not_available_and_runs_nothing() {
    let (got, seen) = run(Path::new("/engines-zut-no-such-folder/a"), output(0, b"", b""), output(0, b"", b""));
    assert_eq!(got, Err(Stop::NotAvailable { why: "no Cargo.toml at or above the rule file's folder".to_string() }));
    assert!(seen.is_empty());
}

#[test]
fn a_param_other_than_allow_is_failed_with_the_reason_and_nothing_runs() {
    let rules = rules_with(r#"{ alow = ["clippy::module_inception"] }"#);
    let (got, seen) = run_rules(&rules, Path::new("/engines-zut-no-such-folder/a"), output(0, b"", b""), output(0, b"", b""));
    let want = "clippy param alow: a clippy rule reads only the `allow` param".to_string();
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

