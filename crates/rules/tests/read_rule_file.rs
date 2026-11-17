//! ZIT (rule 11): `RuleFile::read` on a real `temperare.toml` in a throwaway folder. Proves the pieces of the rules
//! crate work together on a file read from disk: the folder it exposes resolves the rule's `check` file, the read
//! rules scope paths by include, exclude and kit, the read exceptions match findings, and a broken file reports
//! every problem with its line (SPEC-rules WO-rules-1..4, Tests).

use rules::{Engine, Finding, Kind, RelPath, RuleFile, Severity};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const RULE_FILE: &str = r#"version = 1

[kits.core]
paths = ["crates/core/**"]

[[rule]]
id = "no-bool"
engine = "ast-grep"
check = "rules/no-bool.yml"
paths = ["**/*.rs"]
exclude = ["**/generated/**"]
kits = ["core"]
severity = "warning"
fix = "Store a bitmap field."

[[rule]]
id = "lint-all"
engine = "clippy"
check = "clippy::all"
fix = "Do what clippy says."

[[exception]]
rule = "no-bool"
path = "crates/core/src/a.rs"
what = "isArchived"
reason = "Legacy schema."
approved = "bob 2026-11-17"
"#;

const CHECK_FILE: &str = "id: no-bool\nlanguage: rust\nrule:\n  pattern: bool\n";

/// Line 1: `version` is not 1. Line 3: an unknown top-level key.
const BROKEN_FILE: &str = "version = 2\n\ncolour = \"blue\"\n";

/// A unique folder under the system temp folder, removed on drop (also when an assertion fails).
struct TempFolder(PathBuf);

impl TempFolder {
    fn new(tag: &str) -> TempFolder {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let path = std::env::temp_dir().join(format!("tm-rules-zit-{}-{}-{}", std::process::id(), nanos, tag));
        fs::create_dir_all(&path).unwrap();
        TempFolder(path)
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Writes the rule file and its check file into `dir`, then reads the rule file back.
fn write_and_read(dir: &Path) -> RuleFile {
    fs::create_dir_all(dir.join("rules")).unwrap();
    fs::write(dir.join("rules/no-bool.yml"), CHECK_FILE).unwrap();
    let path = dir.join("temperare.toml");
    fs::write(&path, RULE_FILE).unwrap();
    RuleFile::read(&path).unwrap()
}

fn finding(path: &str, what: &str) -> Finding {
    Finding {
        rule: "no-bool".to_string(),
        engine: Engine::AstGrep,
        kind: Kind::Quality,
        severity: Severity::Warning,
        owner: None,
        path: RelPath::new(path),
        line: 7,
        col: Some(3),
        what: what.to_string(),
        fix: "Store a bitmap field.".to_string(),
    }
}

#[test]
fn folder_is_the_rule_files_folder_and_resolves_the_check_file() {
    let tmp = TempFolder::new("folder");
    let file = write_and_read(&tmp.0);
    assert_eq!(file.folder, tmp.0);
    let rule = &file.rules[0];
    assert_eq!(rule.id, "no-bool");
    assert_eq!(rule.engine, Engine::AstGrep);
    let check = file.folder.join(&rule.check);
    assert_eq!(check, tmp.0.join("rules").join("no-bool.yml"));
    assert_eq!(fs::read_to_string(check).unwrap(), CHECK_FILE);
}

#[test]
fn read_rule_scopes_by_paths_exclude_and_kit() {
    let tmp = TempFolder::new("scope");
    let file = write_and_read(&tmp.0);
    let rule = &file.rules[0];
    assert!(rule.applies(&RelPath::new("crates/core/src/a.rs")));
    assert!(!rule.applies(&RelPath::new("crates/other/a.rs")), "outside the kit");
    assert!(!rule.applies(&RelPath::new("crates/core/generated/x.rs")), "excluded");
}

#[test]
fn rule_without_paths_applies_to_every_file() {
    let tmp = TempFolder::new("all");
    let file = write_and_read(&tmp.0);
    let rule = &file.rules[1];
    assert_eq!(rule.id, "lint-all");
    assert_eq!(rule.engine, Engine::Clippy);
    for path in ["crates/core/src/a.rs", "README.md", "Sources/App/Main.swift", "x/generated/y.rs"] {
        assert!(rule.applies(&RelPath::new(path)), "{path}");
    }
}

#[test]
fn exception_for_matches_rule_path_and_what() {
    let tmp = TempFolder::new("exception");
    let file = write_and_read(&tmp.0);
    let hit = file.exception_for(&finding("crates/core/src/a.rs", "stored Bool isArchived on Order"));
    let exception = hit.expect("same rule, same path, `what` contains isArchived");
    assert_eq!(exception.rule, "no-bool");
    assert_eq!(exception.path, RelPath::new("crates/core/src/a.rs"));
    assert_eq!(exception.what.as_deref(), Some("isArchived"));
    assert_eq!(exception.reason, "Legacy schema.");
    assert_eq!(exception.approved, "bob 2026-11-17");
    assert!(file.exception_for(&finding("crates/core/src/a.rs", "stored Bool isPaid on Order")).is_none());
    assert!(file.exception_for(&finding("crates/core/src/b.rs", "stored Bool isArchived on Order")).is_none());
    assert_eq!(file.exceptions_in_force().len(), 1);
}

#[test]
fn broken_file_reports_every_problem_with_its_line() {
    let tmp = TempFolder::new("broken");
    let dir = tmp.0.join("broken");
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("temperare.toml");
    fs::write(&path, BROKEN_FILE).unwrap();
    let problems = RuleFile::read(&path).expect_err("version 2 and an unknown key");
    let mut lines: Vec<usize> = problems.iter().map(|p| p.line).collect();
    lines.sort();
    assert_eq!(lines, vec![1, 3]);
}
