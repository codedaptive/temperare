//! ZUT for hits_clippy.rs. The host is a fake: `cargo locate-project` and `cargo clippy` are answered by matching
//! the command's first argument, clippy from the captured `lint_and_other.jsonl` / `build_failed.jsonl` (see
//! parse_clippy_tests.rs) or a hand-built message. `cargo_root` reads the filesystem, so each test makes its own
//! canonical temp folder of empty `Cargo.toml` files and removes it before asserting.

use super::hits_clippy;
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output, ProcError};
use rules::{RelPath, RuleFile};
use std::cell::RefCell;
use std::fs;
use std::path::{Path, PathBuf};

const LINT_AND_OTHER: &[u8] = include_bytes!("fixtures/lint_and_other.jsonl");
const BUILD_FAILED: &[u8] = include_bytes!("fixtures/build_failed.jsonl");
const RULES: &str = "version = 1\n[[rule]]\nid = \"r\"\nengine = \"clippy\"\ncheck = \"clippy::bool_comparison\"\nfix = \"f\"\n";
const FOUND: &str = "equality checks against true are unnecessary";

/// Hand-built: a bool_comparison message as cargo prints it from a workspace root for its `member/` crate.
const MEMBER_LINT: &str = concat!(
    r#"{"reason":"compiler-message","message":{"message":"equality checks against true are unnecessary","code":{"code":"clippy::bool_comparison","explanation":null},"spans":[{"file_name":"member/src/lib.rs","line_start":3,"column_start":5,"is_primary":true}]}}"#,
    "\n",
    r#"{"reason":"build-finished","success":true}"#,
    "\n"
);

/// A fresh, canonical temp folder holding an empty `Cargo.toml` at each of `manifests` (folders relative to it).
fn tree(name: &str, manifests: &[&str]) -> PathBuf {
    let root = std::env::temp_dir().join(format!("engines-zut-{}-hits-clippy-{name}", std::process::id()));
    fs::create_dir(&root).unwrap();
    let root = fs::canonicalize(root).unwrap();
    for dir in manifests {
        fs::create_dir_all(root.join(dir)).unwrap();
        fs::write(root.join(dir).join("Cargo.toml"), "").unwrap();
    }
    root
}

fn output(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

/// `hits_clippy` on `folder` with a host answering locate-project with `located` and clippy with `linted`; returns
/// the result and every command the host was asked to run.
fn run(folder: &Path, located: Output, linted: Output) -> (Result<Vec<Hit>, Stop>, Vec<Cmd>) {
    let file = RuleFile::parse(RULES, folder).unwrap();
    let seen = RefCell::new(Vec::new());
    let run = |cmd: &Cmd| -> Result<Output, ProcError> {
        seen.borrow_mut().push(cmd.clone());
        match cmd.args[0].to_str() {
            Some("locate-project") => Ok(located.clone()),
            Some("clippy") => Ok(linted.clone()),
            other => panic!("unexpected cargo command {other:?}"),
        }
    };
    let on_path = |_: &str| -> bool { panic!("hits_clippy looks nothing up on PATH") };
    let got = hits_clippy(&file.rules[0], folder, &Host { run: &run, on_path: &on_path });
    (got, seen.into_inner())
}

fn manifest(root: &Path) -> Vec<u8> {
    format!("{}\n", root.join("Cargo.toml").display()).into_bytes()
}

#[test]
fn a_single_crate_is_linted_in_its_own_folder_with_paths_from_there() {
    let root = tree("single", &["."]);
    let (got, seen) = run(&root, output(0, &manifest(&root), b""), output(0, LINT_AND_OTHER, b""));
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Ok(vec![Hit { path: RelPath::new("src/lib.rs"), line: 3, col: Some(5), what: FOUND.to_string() }]));
    let args: Vec<_> = seen.iter().map(|cmd| cmd.args[0].clone()).collect();
    assert_eq!(args, ["locate-project", "clippy"]);
    assert!(seen.iter().all(|cmd| cmd.cwd.as_deref() == Some(root.as_path())));
    assert!(seen[1].env.is_empty(), "no CARGO_TARGET_DIR or other env is set");
}

#[test]
fn a_member_crate_is_linted_alone_and_its_paths_are_read_from_the_workspace_root() {
    let root = tree("member", &[".", "member"]);
    let folder = root.join("member");
    let (got, seen) = run(&folder, output(0, &manifest(&root), b""), output(0, MEMBER_LINT.as_bytes(), b""));
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Ok(vec![Hit { path: RelPath::new("src/lib.rs"), line: 3, col: Some(5), what: FOUND.to_string() }]));
    assert_eq!(seen[0].cwd.as_deref(), Some(folder.as_path()));
    assert_eq!(seen[1].cwd.as_deref(), Some(folder.as_path()));
}

#[test]
fn a_rule_folder_below_the_cargo_root_runs_clippy_in_the_cargo_root() {
    let root = tree("below", &["."]);
    let folder = root.join("src");
    fs::create_dir(&folder).unwrap();
    let (got, seen) = run(&folder, output(0, &manifest(&root), b""), output(0, LINT_AND_OTHER, b""));
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Ok(vec![Hit { path: RelPath::new("lib.rs"), line: 3, col: Some(5), what: FOUND.to_string() }]));
    assert_eq!(seen[1].cwd.as_deref(), Some(root.as_path()));
}

#[test]
fn no_cargo_toml_at_or_above_the_folder_is_not_available_and_runs_nothing() {
    let (got, seen) = run(Path::new("/engines-zut-no-such-folder/a"), output(0, b"", b""), output(0, b"", b""));
    assert_eq!(got, Err(Stop::NotAvailable { why: "no Cargo.toml at or above the rule file's folder".to_string() }));
    assert!(seen.is_empty());
}

#[test]
fn a_failed_locate_project_is_failed_with_its_code_and_stderr_and_clippy_is_not_run() {
    let root = tree("locate-fails", &["."]);
    let stderr = b"error: failed to parse manifest\n";
    let (got, seen) = run(&root, output(101, b"", stderr), output(0, LINT_AND_OTHER, b""));
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Err(Stop::Failed { code: Some(101), stderr: "error: failed to parse manifest\n".to_string() }));
    assert_eq!(seen.len(), 1);
}

#[test]
fn a_failed_build_is_failed_with_code_101_and_cargos_stderr() {
    let root = tree("build-fails", &["."]);
    let stderr = b"error: could not compile `cap_c` (lib) due to 1 previous error\n";
    let (got, _) = run(&root, output(0, &manifest(&root), b""), output(101, BUILD_FAILED, stderr));
    fs::remove_dir_all(&root).unwrap();
    let want = "error: could not compile `cap_c` (lib) due to 1 previous error\n".to_string();
    assert_eq!(got, Err(Stop::Failed { code: Some(101), stderr: want }));
}
