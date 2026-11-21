//! Unit tests for run.rs: help, version and usage errors (install's and uninstall's included). The Rules, Check and Baseline
//! arms reach real git, disk and PATH (the Check arm also HOME and the clock), so run_rules_tests.rs,
//! run_check_tests.rs and run_baseline_test_tests.rs (seams) cover them, and the validation target,
//! tests/validation/ (cli_tm.rs, cli_check.rs, baseline_test_cmd.rs), covers the runs of the built
//! `temperare`. The Install and Uninstall arms read the real HOME, so no test here reaches them: the zone integration
//! target tests/install covers run_install and run_uninstall on a temporary HOME.

use super::run;
use crate::USAGE;
use std::io::{self, Write};
use std::path::Path;

fn run_with(args: &[&str]) -> (u8, String, String) {
    let args: Vec<String> = args.iter().map(|a| a.to_string()).collect();
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = run(&args, Path::new("/nowhere"), &mut out, &mut err);
    (code, String::from_utf8(out).unwrap(), String::from_utf8(err).unwrap())
}

#[test]
fn help_prints_the_usage_and_exits_0() {
    let (code, out, err) = run_with(&["help"]);
    assert_eq!(code, 0);
    assert_eq!(out, USAGE);
    assert!(out.contains("tm rules [--rules <path>] [--json]\n"), "{out}");
    assert_eq!(err, "");
}

#[test]
fn no_arguments_is_help() {
    let (code, out, _) = run_with(&[]);
    assert_eq!(code, 0);
    assert!(out.contains("tm --version\n"), "{out}");
}

#[test]
fn version_prints_temperare_the_workspace_version_and_the_commit() {
    let (code, out, err) = run_with(&["--version"]);
    assert_eq!(code, 0);
    let commit = env!("TEMPERARE_COMMIT");
    assert_eq!(out, format!("temperare 0.3.0 ({commit})\n"));
    if env!("TEMPERARE_CHECKOUT") == "yes" {
        assert_eq!(commit.len(), 12, "{commit}");
        assert!(commit.chars().all(|ch| matches!(ch, '0'..='9' | 'a'..='f')), "{commit}");
    }
    assert_eq!(err, "");
}

#[test]
fn a_usage_error_goes_to_stderr_with_exit_2() {
    let (code, out, err) = run_with(&["frobnicate"]);
    assert_eq!(code, 2);
    assert_eq!(out, "");
    assert_eq!(err, "tm: unknown command 'frobnicate'\nnext: Run tm help to see the commands.\n");
}

#[test]
fn an_install_usage_error_exits_2_on_stderr_before_any_disk_step() {
    let (code, out, err) = run_with(&["install", "--mode", "mcp"]);
    assert_eq!(code, 2);
    assert_eq!(out, "");
    assert_eq!(
        err,
        "tm: --mode mcp is not supported\nnext: Write --mode skill: Temperare wires a skill only, no MCP.\n"
    );
}

#[test]
fn an_argument_after_uninstall_exits_2_on_stderr_before_any_disk_step() {
    let (code, out, err) = run_with(&["uninstall", "now"]);
    assert_eq!(code, 2);
    assert_eq!(out, "");
    assert_eq!(err, "tm: unexpected argument 'now' after uninstall\nnext: Run tm uninstall with nothing after it.\n");
}

/// A writer whose every write fails, like a closed pipe.
struct Closed;

impl Write for Closed {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::Error::from(io::ErrorKind::BrokenPipe))
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn a_closed_output_does_not_change_the_exit_code() {
    let args = vec!["--version".to_string()];
    assert_eq!(run(&args, Path::new("/nowhere"), &mut Closed, &mut Closed), 0);
}
