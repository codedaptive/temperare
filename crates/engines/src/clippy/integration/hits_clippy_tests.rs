//! ZIT for hits_clippy.rs. These are integration, not unit: the real `cargo_root` reads real folders, a canonical temp
//! folder of empty `Cargo.toml` files or, for the no-`Cargo.toml` test, the real ancestors of a missing path. The others compose `hits_clippy` with the real `clippy_allows`,
//! `parse_lint_groups`, `lint_group`, `parse_lints`, `known_lint` and `parse_clippy`. The fake host answers clippy-driver help with the captured
//! `lint_groups_help.txt`, and cargo by its first argument (see hits_clippy_tests.rs); clippy's output is the captured
//! `lint_and_other.jsonl` / `build_failed.jsonl` (see parse_clippy_tests.rs) or a hand-built message. A test that makes a folder removes it before asserting.

use crate::clippy::hits_clippy;
use crate::clippy::hits_clippy_tests::{output, rules_with, run, run_rules, RULES};
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output, ProcError};
use rules::{RelPath, RuleFile};
use std::cell::{Cell, RefCell};
use std::fs;
use std::path::{Path, PathBuf};

const LINT_AND_OTHER: &[u8] = include_bytes!("../fixtures/lint_and_other.jsonl");
const BUILD_FAILED: &[u8] = include_bytes!("../fixtures/build_failed.jsonl");
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

fn manifest(root: &Path) -> Vec<u8> {
    format!("{}\n", root.join("Cargo.toml").display()).into_bytes()
}

/// `hits_clippy` for `check` over `root`, the fake host answering help and locate-project with non-empty output and
/// the clippy check run with `clippy`; also the count the host holds after it and the commands it was given.
fn counted_clippy(
    check: &str,
    root: &Path,
    clippy: &dyn Fn() -> Result<Output, ProcError>,
) -> (Result<Vec<Hit>, Stop>, Option<usize>, Vec<Cmd>) {
    let file = RuleFile::parse(&RULES.replace("clippy::bool_comparison", check), root).unwrap();
    let seen = RefCell::new(Vec::new());
    let raw_chars = Cell::new(None);
    let run = |cmd: &Cmd| -> Result<Output, ProcError> {
        seen.borrow_mut().push(cmd.clone());
        match (cmd.program.to_str(), cmd.args[0].to_str()) {
            (Some("clippy-driver"), Some("-W")) => Ok(output(0, include_bytes!("../fixtures/lint_groups_help.txt"), b"help stderr")),
            (Some("cargo"), Some("locate-project")) => Ok(output(0, &manifest(root), b"locate stderr")),
            (Some("cargo"), Some("clippy")) => clippy(),
            _ => panic!("unexpected command: {cmd:?}"),
        }
    };
    let host = Host { run: &run, on_path: &|_| panic!("unexpected PATH lookup"), raw_chars: &raw_chars };
    let got = hits_clippy(&file.rules[0], root, &host);
    (got, raw_chars.get(), seen.into_inner())
}

/// A clean clippy check run: 42 characters of stdout and "é!", 2 characters, of stderr.
fn finished() -> Result<Output, ProcError> {
    Ok(output(0, br#"{"reason":"build-finished","success":true}"#, "é!".as_bytes()))
}

#[test]
fn counted_clippy_excludes_help_and_locate_and_counts_check_streams() {
    let root = tree("counted", &["."]);
    let (got, chars, seen) = counted_clippy("clippy::bool_comparison", &root, &finished);
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Ok(vec![]));
    assert_eq!(chars, Some(44));
    assert_eq!(seen.len(), 3);
    assert_eq!(seen[0].program, "clippy-driver");
    assert_eq!(seen[1].args[0], "locate-project");
    assert_eq!(seen[2].args[0], "clippy");
}

#[test]
fn counted_clippy_check_run_that_cannot_start_counts_nothing_though_help_and_locate_printed() {
    let root = tree("counted-no-start", &["."]);
    let cannot_start = || Err(ProcError::Spawn { program: "cargo".into(), io: std::io::Error::other("permission denied") });
    let (got, chars, seen) = counted_clippy("clippy::bool_comparison", &root, &cannot_start);
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: "cargo: could not run: permission denied".to_string() }));
    assert_eq!(chars, None);
    assert_eq!(seen.len(), 3);
    assert_eq!(seen[2].args[0], "clippy");
}

#[test]
fn counted_clippy_check_run_not_found_counts_nothing_though_help_and_locate_printed() {
    let root = tree("counted-not-found", &["."]);
    let (got, chars, seen) =
        counted_clippy("clippy::bool_comparison", &root, &|| Err(ProcError::NotFound { program: "cargo".into() }));
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Err(Stop::NotAvailable { why: "cargo not found on PATH".to_string() }));
    assert_eq!(chars, None);
    assert_eq!(seen.len(), 3);
    assert_eq!(seen[2].args[0], "clippy");
}

#[test]
fn counted_clippy_group_refusal_excludes_help() {
    let root = tree("counted-group", &["."]);
    let (got, chars, seen) = counted_clippy("clippy::style", &root, &finished);
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: "clippy check `clippy::style` is a lint group: tm cannot tell its lints from other warnings, so it cannot report it; name each lint, or use `warnings`".into() }));
    assert_eq!(chars, None);
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0].program, "clippy-driver");
}

#[test]
fn no_cargo_toml_at_or_above_the_folder_is_not_available_and_runs_nothing() {
    let (got, seen) = run(Path::new("/engines-zut-no-such-folder/a"), output(0, b"", b""), output(0, b"", b""));
    assert_eq!(got, Err(Stop::NotAvailable { why: "no Cargo.toml at or above the rule file's folder".to_string() }));
    assert!(seen.is_empty());
}

#[test]
fn a_single_crate_is_linted_in_its_own_folder_with_paths_from_there() {
    let root = tree("single", &["."]);
    let (got, seen) = run(&root, output(0, &manifest(&root), b""), output(0, LINT_AND_OTHER, b""));
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Ok(vec![Hit { path: RelPath::new("src/lib.rs"), line: 3, col: Some(5), what: FOUND.to_string() }]));
    let args: Vec<_> = seen.iter().map(|cmd| cmd.args[0].clone()).collect();
    assert_eq!(args, ["-W", "locate-project", "clippy"]);
    assert_eq!(seen[0].program, "clippy-driver");
    assert_eq!(seen[0].args, ["-W", "help"]);
    let want = ["clippy", "--all-targets", "--message-format=json", "--quiet", "--", "--warn=clippy::bool_comparison"];
    assert_eq!(seen[2].args, want.map(std::ffi::OsString::from));
    assert!(seen.iter().all(|cmd| cmd.cwd.as_deref() == Some(root.as_path())));
    assert!(seen[2].env.is_empty(), "no CARGO_TARGET_DIR or other env is set");
}

#[test]
fn a_member_crate_is_linted_alone_and_its_paths_are_read_from_the_workspace_root() {
    let root = tree("member", &[".", "member"]);
    let folder = root.join("member");
    let (got, seen) = run(&folder, output(0, &manifest(&root), b""), output(0, MEMBER_LINT.as_bytes(), b""));
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Ok(vec![Hit { path: RelPath::new("src/lib.rs"), line: 3, col: Some(5), what: FOUND.to_string() }]));
    assert_eq!(seen.len(), 3);
    assert_eq!(seen[0].cwd.as_deref(), Some(folder.as_path()));
    assert_eq!(seen[1].cwd.as_deref(), Some(folder.as_path()));
    assert_eq!(seen[2].cwd.as_deref(), Some(folder.as_path()));
}

#[test]
fn a_rule_folder_below_the_cargo_root_runs_clippy_in_the_cargo_root() {
    let root = tree("below", &["."]);
    let folder = root.join("src");
    fs::create_dir(&folder).unwrap();
    let (got, seen) = run(&folder, output(0, &manifest(&root), b""), output(0, LINT_AND_OTHER, b""));
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Ok(vec![Hit { path: RelPath::new("lib.rs"), line: 3, col: Some(5), what: FOUND.to_string() }]));
    assert_eq!(seen.len(), 3);
    assert_eq!(seen[2].cwd.as_deref(), Some(root.as_path()));
}

#[test]
fn a_failed_locate_project_is_failed_with_its_code_and_stderr_and_clippy_is_not_run() {
    let root = tree("locate-fails", &["."]);
    let stderr = b"error: failed to parse manifest\n";
    let (got, seen) = run(&root, output(101, b"", stderr), output(0, LINT_AND_OTHER, b""));
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Err(Stop::Failed { code: Some(101), stderr: "error: failed to parse manifest\n".to_string() }));
    assert_eq!(seen.len(), 2);
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

#[test]
fn the_allowed_lints_reach_clippy_as_allow_values_after_the_check() {
    let root = tree("allows", &["."]);
    let rules = rules_with(r#"{ allow = ["clippy::result_large_err", "clippy::module_inception"] }"#);
    let (got, seen) = run_rules(&rules, &root, output(0, &manifest(&root), b""), output(0, LINT_AND_OTHER, b""));
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Ok(vec![Hit { path: RelPath::new("src/lib.rs"), line: 3, col: Some(5), what: FOUND.to_string() }]));
    assert_eq!(seen.len(), 3);
    let tail: Vec<_> = seen[2].args.iter().skip(5).filter_map(|arg| arg.to_str()).collect();
    assert_eq!(tail, ["--warn=clippy::bool_comparison", "--allow=clippy::result_large_err", "--allow=clippy::module_inception"]);
}
