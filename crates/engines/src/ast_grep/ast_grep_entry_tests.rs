//! Unit tests for ast_grep_entry.rs: what `AST_GREP` needs, the scan command `entry_cmd` builds from it, its reader on a fixed
//! match line, and the version read from fixed output. Nothing runs.

use super::AST_GREP;
use crate::entry::entry_cmd;
use crate::Hit;
use procio::Output;
use rules::{Engine, RelPath};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[test]
fn the_entry_is_ast_grep_needing_ast_grep() {
    assert_eq!((AST_GREP.name, AST_GREP.engine, AST_GREP.program), ("ast-grep", Engine::AstGrep, "ast-grep"));
    assert_eq!(AST_GREP.needs, ["ast-grep"]);
    assert_eq!(AST_GREP.never, None);
}

#[test]
fn the_command_scans_with_the_rule_file_in_stream_json_from_the_folder() {
    let cmd = entry_cmd(&AST_GREP, OsStr::new("/tmp/tm-ast-grep-1-0.yml"), &[], None, Path::new("/w/repo"));
    assert_eq!(cmd.program, "ast-grep");
    assert_eq!(cmd.args, ["scan", "--rule", "/tmp/tm-ast-grep-1-0.yml", "--json=stream"]);
    assert_eq!(cmd.cwd, Some(PathBuf::from("/w/repo")));
}

#[test]
fn the_command_sets_nothing_else_and_passes_no_config() {
    let cmd = entry_cmd(&AST_GREP, OsStr::new("r.yml"), &[], Some("sgconfig.yml"), Path::new("/w"));
    assert_eq!(cmd.args, ["scan", "--rule", "r.yml", "--json=stream"]);
    assert!(cmd.env.is_empty() && cmd.env_remove.is_empty() && cmd.stdin.is_empty());
    assert_eq!(cmd.timeout, None);
}

#[test]
fn the_reader_reads_match_lines_whatever_the_check() {
    // Hand-built in ast-grep's --json=stream shape; its line and column are 0-based.
    let line = br#"{"file":"src/a.js","range":{"start":{"line":0,"column":12}},"message":"matched name","ruleId":"names"}"#;
    let out = Output { code: Some(0), stdout: line.to_vec(), stderr: Vec::new() };
    let want = Hit { path: RelPath::new("src/a.js"), line: Some(1), col: Some(13), what: "matched name".to_string() };
    let got = (AST_GREP.read)(&out, "rules/names.yml", Path::new("/w/repo"), Path::new("/w/repo"));
    assert_eq!(got, Ok(vec![want]));
}

#[test]
fn the_version_is_read_from_ast_grep_dash_dash_version() {
    let probe = AST_GREP.version.unwrap();
    assert_eq!(probe.args, ["--version"]);
    let out = Output { code: Some(0), stdout: b"ast-grep 0.39.6\n".to_vec(), stderr: Vec::new() };
    assert_eq!((probe.read)(&out), Some("0.39.6".to_string()));
}
