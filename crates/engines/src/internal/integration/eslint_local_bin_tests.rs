//! ZIT for local ESLint lookup: program_for through availability (`availability_in`, and its body
//! `availability_from` with PATH given), version probe and run.

use crate::availability::availability_from;
use crate::host::Host;
use crate::types::Availability;
use crate::run_rules_versioned_with::run_rules_versioned_with;
use procio::Cmd;
use rules::{Engine, RelPath, RuleFile, RuleOutcome};
use std::cell::{Cell, RefCell};
use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

#[path = "../../../tests/support/git.rs"]
mod git_support;

struct TempFolder(PathBuf);

impl TempFolder {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-eslint-{name}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(fs::canonicalize(path).unwrap())
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) { fs::remove_dir_all(&self.0).unwrap(); }
}

#[test]
fn availability_in_finds_local_eslint() {
    use std::os::unix::fs::PermissionsExt;

    let folder = TempFolder::new("availability");
    let root = &folder.0;
    fs::create_dir_all(root.join("node_modules/.bin")).unwrap();
    let eslint = root.join("node_modules/.bin/eslint");
    fs::write(&eslint, b"").unwrap();
    fs::set_permissions(&eslint, fs::Permissions::from_mode(0o755)).unwrap();
    // The public call, with the real PATH: it can tell the local program from PATH's only where eslint is not on PATH.
    // The proof of its folder wiring comes with cli's change that passes the rule file's folder, whose integration
    // test runs tm as a child with a PATH holding node but no eslint.
    assert_eq!(crate::availability_in::availability_in(Engine::Eslint, root), Availability::Available);
    // `availability_in`'s body with PATH given, saying no eslint, so the folder alone decides.
    let on_path = |n: &str| n != "eslint" && procio::which(n).is_some();
    assert_eq!(availability_from(Engine::Eslint, Some(root), on_path), Availability::Available);
    let directory_case = TempFolder::new("availability-dir");
    fs::create_dir_all(directory_case.0.join("node_modules/.bin/eslint")).unwrap();
    assert_eq!(
        availability_from(Engine::Eslint, Some(&directory_case.0), on_path),
        Availability::NotAvailable { why: "eslint not found on PATH".into() }
    );
}

#[test]
fn local_eslint_runs_rule_and_supplies_version() {
    let folder = TempFolder::new("local-bin");
    let root = &folder.0;
    git_support::git(root, &["init", "-q"], b"");
    fs::create_dir_all(root.join("node_modules/.bin")).unwrap();
    symlink("/opt/homebrew/lib/node_modules/eslint/bin/eslint.js", root.join("node_modules/.bin/eslint")).unwrap();
    symlink("/opt/homebrew/lib/node_modules/eslint", root.join("node_modules/eslint")).unwrap();
    symlink("/opt/homebrew/lib/node_modules/typescript-eslint", root.join("node_modules/typescript-eslint")).unwrap();
    fs::write(root.join("eslint.config.mjs"), "export default [];\n").unwrap();
    fs::write(root.join("example.js"), "var x = 1;\nconsole.log(x);\n").unwrap();
    git_support::git(root, &["add", "example.js"], b"");
    fs::write(root.join("temperare.toml"), r#"version = 1
[[rule]]
id = "eslint-test"
engine = "eslint"
check = "no-var"
fix = "Use the recommended form."
"#).unwrap();
    let file = RuleFile::read(&root.join("temperare.toml")).unwrap();
    let programs = RefCell::new(Vec::<OsString>::new());
    let run = |cmd: &Cmd| {
        programs.borrow_mut().push(cmd.program.clone());
        procio::run(cmd)
    };
    let on_path = |n: &str| n != "eslint" && procio::which(n).is_some();
    let host = Host { run: &run, on_path: &on_path, raw_chars: &Cell::new(None), base: None };
    let got = run_rules_versioned_with(&file, root, &[], &host);

    assert_eq!(got.runs.len(), 1, "{:?}", got.runs);
    let RuleOutcome::Findings(findings) = &got.runs[0].outcome else {
        panic!("expected findings, got {:?}", got.runs[0].outcome);
    };
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(findings[0].path, RelPath::new("example.js"));
    assert_eq!(got.engines.len(), 1, "{:?}", got.engines);
    assert_eq!(got.engines[0].engine, Engine::Eslint);
    let package: serde_json::Value =
        serde_json::from_str(&fs::read_to_string("/opt/homebrew/lib/node_modules/eslint/package.json").unwrap()).unwrap();
    assert_eq!(got.engines[0].version.as_deref(), Some(package["version"].as_str().unwrap()),"{:?}", got.engines);
    let programs = programs.borrow();
    let eslint: Vec<_> = programs.iter().filter(|p| Path::new(p).file_name().is_none_or(|n| n != "git")).collect();
    assert!(eslint.len() >= 2, "{eslint:?}");
    assert!(eslint.iter().all(|p| Path::new(p) == root.join("node_modules/.bin/eslint")), "{eslint:?}");
}
