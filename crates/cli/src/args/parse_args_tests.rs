//! Unit tests for parse_args.rs: help and version forms, dispatch to each command (install, uninstall and baseline
//! included), flags given twice, unknown commands.

use std::path::PathBuf;

use super::{parse_args, BaselineArgs, CheckArgs, Command, InstallArgs, RulesArgs, UsageError};
use crate::Harness;

fn parse(args: &[&str]) -> Result<Command, UsageError> {
    parse_args(&args.iter().map(|a| a.to_string()).collect::<Vec<_>>())
}

fn error(what: &str, fix: &str) -> UsageError {
    UsageError { what: what.to_string(), fix: fix.to_string() }
}

#[test]
fn no_arguments_is_help() {
    assert_eq!(parse(&[]), Ok(Command::Help));
}

#[test]
fn each_help_form_is_help() {
    assert_eq!(parse(&["help"]), Ok(Command::Help));
    assert_eq!(parse(&["--help"]), Ok(Command::Help));
    assert_eq!(parse(&["-h"]), Ok(Command::Help));
}

#[test]
fn version() {
    assert_eq!(parse(&["--version"]), Ok(Command::Version));
}

#[test]
fn an_argument_after_help_is_refused() {
    assert_eq!(
        parse(&["help", "rules"]),
        Err(error("unexpected argument 'rules' after help", "Run tm help with nothing after it."))
    );
    assert_eq!(
        parse(&["-h", "x"]),
        Err(error("unexpected argument 'x' after -h", "Run tm -h with nothing after it."))
    );
}

#[test]
fn an_argument_after_version_is_refused() {
    assert_eq!(
        parse(&["--version", "--json"]),
        Err(error("unexpected argument '--json' after --version", "Run tm --version with nothing after it."))
    );
}

#[test]
fn rules_command() {
    assert_eq!(
        parse(&["rules", "--rules", "t.toml", "--json"]),
        Ok(Command::Rules(RulesArgs { rules: Some(PathBuf::from("t.toml")), json: true, ..RulesArgs::default() }))
    );
}

#[test]
fn check_command() {
    assert_eq!(
        parse(&["check", "--kind", "quality", "src"]),
        Ok(Command::Check(CheckArgs {
            kind: Some(rules::Kind::Quality),
            paths: vec![PathBuf::from("src")],
            ..CheckArgs::default()
        }))
    );
}

#[test]
fn baseline_command() {
    assert_eq!(
        parse(&["baseline", "test", "--set", "house@1", "--json"]),
        Ok(Command::Baseline(BaselineArgs { set: Some("house@1".to_string()), json: true }))
    );
    assert_eq!(parse(&["baseline"]), Err(error("tm baseline needs a subcommand", "Run tm baseline test.")));
    assert_eq!(
        parse(&["baseline", "test", "--json", "--json"]),
        Err(error("--json given twice", "Give --json once."))
    );
}

#[test]
fn sub_parser_errors_come_through() {
    assert_eq!(
        parse(&["rules", "--base", "main"]),
        Err(error("--base is only for tm rules --loosened", "Add --loosened, or leave --base out."))
    );
}

#[test]
fn a_flag_given_twice() {
    assert_eq!(
        parse(&["rules", "--rules", "a.toml", "--rules", "b.toml"]),
        Err(error("--rules given twice", "Give --rules once."))
    );
    assert_eq!(
        parse(&["rules", "--loosened", "--base", "a", "--base", "b"]),
        Err(error("--base given twice", "Give --base once."))
    );
    assert_eq!(
        parse(&["check", "--json", "src", "--json"]),
        Err(error("--json given twice", "Give --json once."))
    );
}

#[test]
fn the_same_path_twice_is_not_a_repeated_flag() {
    assert_eq!(
        parse(&["check", "src", "src"]),
        Ok(Command::Check(CheckArgs {
            paths: vec![PathBuf::from("src"), PathBuf::from("src")],
            ..CheckArgs::default()
        }))
    );
}

#[test]
fn an_unknown_command() {
    assert_eq!(
        parse(&["frobnicate"]),
        Err(error("unknown command 'frobnicate'", "Run tm help to see the commands."))
    );
}

#[test]
fn a_flag_before_the_command_is_an_unknown_command() {
    assert_eq!(
        parse(&["--json", "rules"]),
        Err(error("unknown command '--json'", "Run tm help to see the commands."))
    );
}

#[test]
fn install_command() {
    assert_eq!(
        parse(&["install", "--no-place", "--target", "cursor,claude", "--mode", "skill", "-y"]),
        Ok(Command::Install(InstallArgs {
            no_place: true,
            targets: Some(vec![Harness::Cursor, Harness::Claude]),
            yes: true,
            ..InstallArgs::default()
        }))
    );
}

#[test]
fn install_alone_is_install_with_no_flags() {
    assert_eq!(parse(&["install"]), Ok(Command::Install(InstallArgs::default())));
}

#[test]
fn install_errors_come_through() {
    assert_eq!(
        parse(&["install", "--mode", "mcp"]),
        Err(error("--mode mcp is not supported", "Write --mode skill: Temperare wires a skill only, no MCP."))
    );
}

#[test]
fn an_install_flag_given_twice() {
    assert_eq!(parse(&["install", "-y", "-y"]), Err(error("-y given twice", "Give -y once.")));
}

#[test]
fn uninstall_command() {
    assert_eq!(parse(&["uninstall"]), Ok(Command::Uninstall));
}

#[test]
fn an_argument_after_uninstall_is_refused() {
    assert_eq!(
        parse(&["uninstall", "-y"]),
        Err(error("unexpected argument '-y' after uninstall", "Run tm uninstall with nothing after it."))
    );
}
