//! ZIT for clippy group discovery on real temporary Cargo trees with a fake host.
//! The help fixture is stdout from `clippy-driver -W help`, clippy 0.1.95 via rustup in /tmp, exit 0, empty stderr.

use crate::clippy::hits_clippy_tests::{output, run_rules_with_help, RULES};
use crate::types::Stop;
use procio::{Cmd, Output};
use std::fs;
use std::path::{Path, PathBuf};

const HELP: &str = include_str!("../fixtures/lint_groups_help.txt");

pub(super) fn tree(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("engines-zit-{}-clippy-groups-{name}", std::process::id()));
    fs::create_dir(&root).unwrap();
    let root = fs::canonicalize(root).unwrap();
    fs::write(root.join("Cargo.toml"), "").unwrap();
    root
}

fn rules(check: &str) -> String {
    RULES.replace("clippy::bool_comparison", check)
}

pub(super) fn assert_help_only(seen: &[Cmd], root: &Path) {
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0].program, "clippy-driver");
    assert_eq!(seen[0].args, ["-W", "help"]);
    assert_eq!(seen[0].cwd.as_deref(), Some(root));
}

fn refuses_group(name: &str, check: &str, help: Output) {
    let root = tree(name);
    let (got, seen) = run_rules_with_help(&rules(check), &root, help, output(0, b"", b""), output(0, b"", b""));
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: format!("clippy check `{check}` is a lint group: cctao cannot tell its lints from other warnings, so it cannot report it; name each lint, or use `warnings`") }));
    assert_help_only(&seen, &root);
}

fn future_help() -> Output {
    let (before, section) = HELP.split_once("Lint groups loaded by this crate:").unwrap();
    let section = section.replacen("----  ---------\n", "----  ---------\n                                  clippy::future_group  clippy::some-lint\n", 1);
    output(0, format!("{before}Lint groups loaded by this crate:{section}").as_bytes(), b"")
}

#[test]
fn clippy_style_group_is_refused_after_help() {
    refuses_group("style", "clippy::style", output(0, HELP.as_bytes(), b""));
}

#[test]
fn rustc_unused_group_is_refused_after_help() {
    refuses_group("unused", "unused", output(0, HELP.as_bytes(), b""));
}

#[test]
fn dashed_rustc_group_is_refused_after_help() {
    refuses_group("dashed", "nonstandard-style", output(0, HELP.as_bytes(), b""));
}

#[test]
fn future_group_is_discovered_from_help() {
    refuses_group("future", "clippy::future_group", future_help());
}

#[test]
fn dashed_future_group_is_discovered_from_help() {
    refuses_group("future-dashed", "clippy::future-group", future_help());
}

fn failed_help(name: &str, help: Output, code: Option<i32>) {
    let root = tree(name);
    let (got, seen) = run_rules_with_help(RULES, &root, help, output(0, b"", b""), output(0, b"", b""));
    fs::remove_dir_all(&root).unwrap();
    assert!(matches!(got, Err(Stop::Failed { code: actual, .. }) if actual == code), "{got:?}");
    assert_help_only(&seen, &root);
}

#[test]
fn failed_help_stops_before_clippy() {
    failed_help("failed-help", output(1, b"", b""), Some(1));
}

#[test]
fn help_without_groups_stops_before_clippy() {
    failed_help("no-groups", output(0, b"Available lint options:\n", b""), Some(0));
}

fn accepted_check(name: &str, check: &str) {
    let root = tree(name);
    let manifest = format!("{}\n", root.join("Cargo.toml").display());
    let (got, seen) = run_rules_with_help(&rules(check), &root, output(0, HELP.as_bytes(), b""), output(0, manifest.as_bytes(), b""), output(0, include_bytes!("../fixtures/clean.jsonl"), b""));
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Ok(vec![]));
    assert_eq!(seen.len(), 3);
    assert_eq!(seen[0].program, "clippy-driver");
    assert_eq!(seen[0].args, ["-W", "help"]);
    assert_eq!(seen[0].cwd.as_deref(), Some(root.as_path()));
    assert_eq!(seen[1].program, "cargo");
    assert_eq!(seen[1].args[0], "locate-project");
    assert_eq!(seen[2].program, "cargo");
    assert_eq!(seen[2].args[0], "clippy");
}

#[test]
fn single_lint_runs_help_locate_then_clippy() {
    accepted_check("single", "clippy::bool_comparison");
}

#[test]
fn warnings_is_not_rejected_as_a_group() {
    accepted_check("warnings", "warnings");
}
