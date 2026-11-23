//! Unit tests for golangci_lint_entry.rs: `GOLANGCI_LINT`'s fields, the command `entry_cmd` builds from it with and without
//! a config, and the version read from real `golangci-lint --version` output. Nothing runs.

use super::GOLANGCI_LINT;
use crate::entry::entry_cmd;
use procio::Output;
use rules::Engine;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

const PINNED: [&str; 12] = [
    "run",
    "--output.json.path=stdout",
    "--output.text.path=stderr",
    "--show-stats=false",
    "--issues-exit-code=1",
    "--max-issues-per-linter=0",
    "--max-same-issues=0",
    "--uniq-by-line=false",
    "--allow-parallel-runners",
    "--path-mode=abs",
    "--path-prefix=",
    "--fix=false",
];

#[test]
fn the_entry_is_golangci_lint_needing_golangci_lint_and_go() {
    assert_eq!(
        (GOLANGCI_LINT.name, GOLANGCI_LINT.engine, GOLANGCI_LINT.program),
        ("golangci-lint", Engine::GolangciLint, "golangci-lint")
    );
    assert_eq!(GOLANGCI_LINT.needs, ["golangci-lint", "go"]);
    assert_eq!(GOLANGCI_LINT.never, None);
}

#[test]
fn entry_cmd_runs_the_pinned_flags_with_the_linters_and_any_config_over_every_package() {
    let rows: [(&str, Option<&str>, &[&str]); 3] = [
        ("staticcheck", None, &["--enable-only=staticcheck", "./..."]),
        ("staticcheck,govet", Some("lint/.golangci.yml"), &["--enable-only=staticcheck,govet", "--config=lint/.golangci.yml", "./..."]),
        ("--fix", Some("--no-config"), &["--enable-only=--fix", "--config=--no-config", "./..."]),
    ];
    for (check, config, tail) in rows {
        let cmd = entry_cmd(&GOLANGCI_LINT, OsStr::new(check), &[], config, Path::new("/w/go"));
        let want: Vec<OsString> = PINNED.iter().chain(tail).map(OsString::from).collect();
        assert_eq!(cmd.program, OsString::from("golangci-lint"), "{check}");
        assert_eq!(cmd.args, want, "{check}");
        assert_eq!(cmd.cwd, Some(PathBuf::from("/w/go")), "{check}");
        assert!(cmd.env.is_empty() && cmd.env_remove.is_empty() && cmd.timeout.is_none(), "{check}");
    }
}

#[test]
fn the_version_is_read_from_golangci_lint_dash_dash_version() {
    let probe = GOLANGCI_LINT.version.unwrap();
    assert_eq!(probe.args, ["--version"]);
    let line = b"golangci-lint has version 2.15.0 built with go1.27.2 from 25c5369 on 2026-10-16T10:51:54Z\n";
    let out = Output { code: Some(0), stdout: line.to_vec(), stderr: Vec::new() };
    assert_eq!((probe.read)(&out), Some("2.15.0".to_string()));
}
