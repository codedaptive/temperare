#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use engines::run_rules_versioned_at;
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

#[path = "support/git.rs"]
mod git_support;

struct TempFolder(PathBuf);

impl TempFolder {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-actionlint-{name}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(fs::canonicalize(path).unwrap())
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) { fs::remove_dir_all(&self.0).unwrap(); }
}

struct PathGuard(Option<OsString>);

impl Drop for PathGuard {
    fn drop(&mut self) {
        match &self.0 {
            Some(path) => std::env::set_var("PATH", path),
            None => std::env::remove_var("PATH"),
        }
    }
}

struct ShellcheckOptsGuard(Option<OsString>);

impl Drop for ShellcheckOptsGuard {
    fn drop(&mut self) {
        match &self.0 {
            Some(opts) => std::env::set_var("SHELLCHECK_OPTS", opts),
            None => std::env::remove_var("SHELLCHECK_OPTS"),
        }
    }
}

#[test]
fn actionlint_rule_reports_workflows_suppressions_and_availability() {
    let original = PathGuard(std::env::var_os("PATH"));
    std::env::set_var("PATH", std::env::join_paths(
        std::iter::once(Path::new("/opt/homebrew/bin").to_path_buf())
            .chain(std::env::split_paths(original.0.as_ref().unwrap()))
    ).unwrap());
    let violation = "name: CI\non: push\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v4\n        with:\n          nonexistent-input: value\n      - run: echo \"${{ unknown.x }}\"\n";
    let clean = "name: CI\non: push\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      - run: echo hello\n";
    let shell = "name: CI\non: push\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      - run: echo $FOO\n";
    let cases = [
        ("violation", "expression"),
        ("clean", "expression"),
        ("ignored", "expression"),
        ("merged-ignore", "expression"),
        ("unknown-kind", "expresion"),
        ("malformed", "Expression"),
        ("shellcheck", "shellcheck"),
        ("shellcheck-absent", "shellcheck"),
        ("pyflakes-absent", "pyflakes"),
        ("no-workflow", "expression"),
        ("no-workflow-absent", "expression"),
        ("absent", "expression"),
    ];
    let mut failed = Vec::new();
    for (name, check) in cases {
        let result = std::panic::catch_unwind(|| {
            let folder = TempFolder::new(name);
            git_support::git(&folder.0, &["init", "-q"], b"");
            let no_workflow = name == "no-workflow" || name == "no-workflow-absent";
            if no_workflow {
                fs::write(folder.0.join("notes.txt"), "no workflows here\n").unwrap();
            } else {
                fs::create_dir_all(folder.0.join(".github/workflows")).unwrap();
                let source = match name {
                    "clean" => clean,
                    "shellcheck" | "shellcheck-absent" => shell,
                    _ => violation,
                };
                fs::write(folder.0.join(".github/workflows/ci.yml"), source).unwrap();
            }
            if name == "violation" {
                fs::write(folder.0.join("other.yml"), violation).unwrap();
            }
            match name {
                "ignored" => fs::write(folder.0.join(".github/actionlint.yaml"),
                    "paths:\n  \"**\":\n    ignore: ['undefined variable']\n").unwrap(),
                "merged-ignore" => fs::write(folder.0.join(".github/actionlint.yaml"),
                    "base: &b {ignore: ['undefined variable']}\npaths:\n  \"**\": {<<: *b}\n").unwrap(),
                _ => {}
            }
            fs::write(folder.0.join("temperare.toml"), format!(
                "version = 1\n[[rule]]\nid = \"undefined-variable\"\nengine = \"actionlint\"\ncheck = \"{check}\"\nfix = \"Define the variable.\"\n"
            )).unwrap();
            let file = RuleFile::read(&folder.0.join("temperare.toml")).unwrap();
            let tools = if name == "shellcheck-absent" || name == "pyflakes-absent" {
                let tools = TempFolder::new(&format!("{name}-bin"));
                for program in ["actionlint", "git"] {
                    let executable = std::env::split_paths(&std::env::var_os("PATH").unwrap())
                        .map(|dir| dir.join(program))
                        .find(|path| path.metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0))
                        .unwrap();
                    std::os::unix::fs::symlink(executable, tools.0.join(program)).unwrap();
                }
                Some(tools)
            } else { None };
            let _path_guard = if let Some(tools) = &tools {
                let guard = PathGuard(std::env::var_os("PATH"));
                std::env::set_var("PATH", &tools.0);
                Some(guard)
            } else if name == "absent" || name == "no-workflow-absent" {
                let guard = PathGuard(std::env::var_os("PATH"));
                let filtered = std::env::join_paths(std::env::split_paths(guard.0.as_ref().unwrap()).filter(|dir| {
                    !dir.join("actionlint").metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
                })).unwrap();
                std::env::set_var("PATH", filtered);
                Some(guard)
            } else { None };
            let _opts_guard = if name == "shellcheck" {
                let guard = ShellcheckOptsGuard(std::env::var_os("SHELLCHECK_OPTS"));
                std::env::set_var("SHELLCHECK_OPTS", "-e SC2086");
                Some(guard)
            } else { None };
            let runs = run_rules_versioned_at(&file, &folder.0, &[], None).runs;
            assert_eq!(runs.len(), 1, "{name}: {runs:?}");
            let run = &runs[0];
            if no_workflow {
                assert_eq!(run.no_files.as_deref(), Some("no file its engine checks"), "{name}: {run:?}");
                assert_eq!(run.outcome, RuleOutcome::Findings(vec![]), "{name}");
                return;
            }
            assert_eq!(run.no_files, None, "{name}: {run:?}");
            match name {
                "unknown-kind" | "absent" | "shellcheck-absent" | "pyflakes-absent" => {
                    let RuleOutcome::NotAvailable { rule, engine, why } = &run.outcome else {
                        panic!("{name}: expected NotAvailable, got {:?}", run.outcome);
                    };
                    assert_eq!(rule, "undefined-variable", "{name}");
                    assert_eq!(*engine, Engine::Actionlint, "{name}");
                    match name {
                        "unknown-kind" => {
                            assert!(why.starts_with("actionlint 1.7.12"), "{name}: {why}");
                            assert!(why.contains("expresion"), "{name}: {why}");
                        }
                        "shellcheck-absent" => assert_eq!(why, "shellcheck not found on PATH", "{name}"),
                        "pyflakes-absent" => assert_eq!(why, "pyflakes not found on PATH", "{name}"),
                        _ => assert_eq!(why, "actionlint not found on PATH", "{name}"),
                    }
                }
                "ignored" | "merged-ignore" | "malformed" => {
                    let RuleOutcome::Failed { stderr, .. } = &run.outcome else {
                        panic!("{name}: expected Failed, got {:?}", run.outcome);
                    };
                    match name {
                        "ignored" => {
                            assert!(stderr.contains("actionlint.yaml"), "{name}: {stderr}");
                            assert!(stderr.contains("tm cannot turn it off; put exceptions in the rule file"), "{name}: {stderr}");
                            assert!(!stderr.contains("S-67"), "{name}: {stderr}");
                        }
                        "merged-ignore" => {
                            assert!(stderr.contains("actionlint.yaml"), "{name}: {stderr}");
                            assert!(stderr.contains("merge key"), "{name}: {stderr}");
                        }
                        "malformed" => assert!(stderr.contains("is not an actionlint check kind"), "{name}: {stderr}"),
                        _ => unreachable!(),
                    }
                }
                _ => {
                    let RuleOutcome::Findings(findings) = &run.outcome else {
                        panic!("{name}: expected Findings, got {:?}", run.outcome);
                    };
                    assert_eq!(findings.len(), usize::from(name != "clean"), "{name}: {findings:?}");
                    if name == "violation" {
                        let finding = &findings[0];
                        assert_eq!(finding.rule, "undefined-variable", "{name}");
                        assert_eq!(finding.path, RelPath::new(".github/workflows/ci.yml"), "{name}");
                        assert_eq!(finding.line(), Some(10), "{name}");
                        assert_eq!(finding.col, Some(24), "{name}");
                        assert_eq!(finding.what, "undefined variable \"unknown\". available variables are \"env\", \"github\", \"inputs\", \"job\", \"matrix\", \"needs\", \"runner\", \"secrets\", \"steps\", \"strategy\", \"vars\"", "{name}");
                        assert_eq!(finding.fix, "Define the variable.", "{name}");
                    }
                    if name == "shellcheck" {
                        assert!(findings[0].what.contains("SC2086"), "{name}: {findings:?}");
                    }
                }
            }
            if name == "violation" {
                let mut entries: Vec<_> = fs::read_dir(&folder.0).unwrap().map(|e| e.unwrap().file_name()).collect();
                entries.sort();
                assert_eq!(entries, [".git", ".github", "other.yml", "temperare.toml"]);
                let mut github: Vec<_> = fs::read_dir(folder.0.join(".github")).unwrap().map(|e| e.unwrap().file_name()).collect();
                github.sort();
                assert_eq!(github, ["workflows"]);
                let mut workflows: Vec<_> = fs::read_dir(folder.0.join(".github/workflows")).unwrap().map(|e| e.unwrap().file_name()).collect();
                workflows.sort();
                assert_eq!(workflows, ["ci.yml"]);
            }
        });
        if result.is_err() { failed.push(name); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
