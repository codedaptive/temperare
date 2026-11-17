//! ZUT for run.rs: help, version and a usage error. The Rules and Check arms reach real git, disk and PATH, so
//! run_rules_tests.rs and run_check_tests.rs (seams) cover them, and the validation target, tests/validation/
//! (cli_tm.rs, cli_check.rs, S-28), covers the runs of the built tm.

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
fn version_prints_tm_and_the_workspace_version() {
    assert_eq!(run_with(&["--version"]), (0, "tm 0.1.0\n".to_string(), String::new()));
}

#[test]
fn a_usage_error_goes_to_stderr_with_exit_2() {
    let (code, out, err) = run_with(&["frobnicate"]);
    assert_eq!(code, 2);
    assert_eq!(out, "");
    assert_eq!(err, "tm: unknown command 'frobnicate'\nnext: Run tm help to see the commands.\n");
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
