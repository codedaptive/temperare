//! Unit tests for shared_cmd.rs: an arg pick replaced by the union, a file pick run as it is and keyed without its path.

use crate::ast_grep::AST_GREP;
use crate::clippy::CLIPPY;
use crate::ruff::RUFF;
use crate::shared_cmd::shared_cmd;
use procio::Cmd;
use std::ffi::OsString;
use std::path::PathBuf;

fn cmd(program: &str, list: &[&str]) -> Cmd {
    Cmd { args: args(list), cwd: Some(PathBuf::from("/repo")), ..Cmd::new(program) }
}

fn args(of: &[&str]) -> Vec<OsString> {
    of.iter().map(OsString::from).collect()
}

#[test]
fn ruff_runs_with_the_union_in_place_of_its_select() {
    let own = cmd("ruff", &["check", "--select=E711", "--config=r.toml", "."]);
    let (key, run) = shared_cmd(&RUFF, &own, 1, &args(&["--select=E711,F401"]));
    let want = cmd("ruff", &["check", "--select=E711,F401", "--config=r.toml", "."]);
    assert_eq!((key, run), (want.clone(), want));
}

#[test]
fn clippy_runs_with_each_warn_of_the_union_where_its_own_was() {
    let own = cmd("cargo", &["clippy", "--", "--warn=dead_code", "--allow=x"]);
    let (key, run) = shared_cmd(&CLIPPY, &own, 2, &args(&["--warn=needless_return", "--warn=dead_code"]));
    let want = cmd("cargo", &["clippy", "--", "--warn=needless_return", "--warn=dead_code", "--allow=x"]);
    assert_eq!((key, run), (want.clone(), want));
}

#[test]
fn ast_grep_runs_its_own_command_keyed_without_the_copy_path() {
    let own = cmd("ast-grep", &["scan", "--rule", "/tmp/ast-grep-7.yml", "--json=stream"]);
    let (key, run) = shared_cmd(&AST_GREP, &own, 2, &[]);
    assert_eq!(key, cmd("ast-grep", &["scan", "--rule", "", "--json=stream"]));
    assert_eq!(run, cmd("ast-grep", &["scan", "--rule", "/tmp/ast-grep-7.yml", "--json=stream"]));
}

#[test]
fn two_ast_grep_copies_share_one_key() {
    let one = cmd("ast-grep", &["scan", "--rule", "/tmp/ast-grep-1.yml", "--json=stream"]);
    let two = cmd("ast-grep", &["scan", "--rule", "/tmp/ast-grep-2.yml", "--json=stream"]);
    assert_eq!(shared_cmd(&AST_GREP, &one, 2, &[]).0, shared_cmd(&AST_GREP, &two, 2, &[]).0);
}
