//! Zone integration tests for lint discovery: `hits_clippy` with the real `parse_lints` and `known_lint`, the captured clippy-driver
//! help and a fake process host. An unlisted, renamed or rustdoc check is refused after the help and before
//! locate-project; a listed lint runs help, locate-project and clippy.

use super::hits_clippy_groups_tests::{assert_help_only, tree};
use crate::clippy::hits_clippy_tests::{output, run_rules_with_help, RULES};
use crate::types::Stop;
use std::fs;

const HELP: &[u8] = include_bytes!("../fixtures/lint_groups_help.txt");
const CLEAN: &[u8] = include_bytes!("../fixtures/clean.jsonl");

fn run_check(name: &str, check: &str, params: &str) -> (Result<Vec<crate::types::Hit>, Stop>, Vec<procio::Cmd>, std::path::PathBuf) {
    let root = tree(name);
    let rules = format!("{}{params}", RULES.replace("clippy::bool_comparison", check));
    let manifest = format!("{}\n", root.join("Cargo.toml").display());
    let (got, seen) = run_rules_with_help(&rules, &root, output(0, HELP, b""), output(0, manifest.as_bytes(), b""), output(0, CLEAN, b""));
    fs::remove_dir_all(&root).unwrap();
    (got, seen, root)
}

fn assert_unknown(name: &str, check: &str, params: &str) {
    let (got, seen, root) = run_check(name, check, params);
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: format!("clippy check `{check}` is not a lint this toolchain lists (clippy-driver -W help): tm cannot tell it was run, so it cannot report it; name a lint it lists, or use `warnings`") }));
    assert_help_only(&seen, &root);
}

#[test]
fn unknown_lint_is_refused_before_locate() {
    assert_unknown("unknown", "clippy::bool_comparisn", "");
}

#[test]
fn renamed_lint_not_in_help_is_refused_before_locate() {
    assert_unknown("renamed", "clippy::stutter", "");
}

#[test]
fn rustdoc_lint_is_refused_before_locate() {
    let (got, seen, root) = run_check("rustdoc", "rustdoc::broken_intra_doc_links", "");
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: "clippy check `rustdoc::broken_intra_doc_links` is a rustdoc lint: clippy never runs rustdoc lints, so tm cannot report it".to_string() }));
    assert_help_only(&seen, &root);
}

#[test]
fn allowing_unknown_lints_cannot_hide_an_unknown_check() {
    assert_unknown("unknown-allowed", "clippy::bool_comparisn", "params = { allow = [\"unknown_lints\"] }\n");
}

#[test]
fn listed_dashed_lint_runs_help_locate_and_clippy() {
    let (got, seen, root) = run_check("listed-dashed", "clippy::bool-comparison", "");
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
