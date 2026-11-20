//! ZUT for pick_at.rs: the index of the check's argument for each kind of pick, and None for a helper command.

use crate::ast_grep::AST_GREP;
use crate::clippy::CLIPPY;
use crate::pick_at::pick_at;
use crate::ruff::RUFF;
use crate::tessera::TESSERA;
use procio::Cmd;
use std::ffi::OsString;

fn cmd(program: &str, args: &[&str]) -> Cmd {
    Cmd { args: args.iter().map(OsString::from).collect(), ..Cmd::new(program) }
}

#[test]
fn an_arg_pick_is_the_first_argument_joining_flag_and_check() {
    let run = cmd("ruff", &["check", "--no-cache", "--select=E711", "--config=x.toml", "--select=E711", "."]);
    assert_eq!(pick_at(&RUFF, "E711", &run), Some(2));
}

#[test]
fn an_arg_pick_does_not_match_another_check_or_a_prefix_of_it() {
    let run = cmd("ruff", &["check", "--select=E7", "--select=E7111", "."]);
    assert_eq!(pick_at(&RUFF, "E711", &run), None);
}

#[test]
fn a_helper_command_has_no_pick() {
    let help = cmd("clippy-driver", &["-W", "help"]);
    assert_eq!(pick_at(&CLIPPY, "needless_return", &help), None);
}

#[test]
fn clippy_picks_its_warn_argument() {
    let run = cmd("cargo", &["clippy", "--all-targets", "--", "--warn=needless_return", "--allow=dead_code"]);
    assert_eq!(pick_at(&CLIPPY, "needless_return", &run), Some(3));
}

#[test]
fn a_file_pick_is_the_argument_after_the_option_whatever_the_check() {
    let run = cmd("ast-grep", &["scan", "--rule", "/tmp/ast-grep-1.yml", "--json=stream"]);
    assert_eq!(pick_at(&AST_GREP, "rules/no-eval.yml", &run), Some(2));
}

#[test]
fn a_file_pick_with_nothing_after_the_option_is_none() {
    let run = cmd("ast-grep", &["scan", "--rule"]);
    assert_eq!(pick_at(&AST_GREP, "rules/no-eval.yml", &run), None);
}

#[test]
fn a_file_pick_without_the_option_is_none() {
    let run = cmd("ast-grep", &["--version"]);
    assert_eq!(pick_at(&AST_GREP, "rules/no-eval.yml", &run), None);
}

#[test]
fn no_pick_never_has_an_index() {
    let run = cmd("ts", &["slop", "--json", "slop"]);
    assert_eq!(pick_at(&TESSERA, "slop", &run), None);
}
