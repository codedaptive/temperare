#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use engines::{no_file_rules, run_rules_versioned_at, NoFileRule, NotAvailableCause, RuleRun};
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

mod support {
    pub mod git;
}

static LOCK: Mutex<()> = Mutex::new(());
const VIOLATION: &str = "int f(int a) { if (a) return 1; return 0; }\n";
const FIX: &str = "Add braces around the statement.";

struct Repo(PathBuf);

impl Repo {
    fn new(case: &str, source: &str, check: &str) -> Self {
        Self::with_file(case, "a.c", source, check)
    }

    fn with_file(case: &str, filename: &str, source: &str, check: &str) -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-clang-tidy-{case}", std::process::id()));
        fs::create_dir(&path).unwrap();
        let repo = Self(fs::canonicalize(path).unwrap());
        support::git::git(&repo.0, &["init", "-q"], b"");
        fs::write(repo.0.join(filename), source).unwrap();
        fs::write(repo.0.join("temperare.toml"), format!(
            "version = 1\n[[rule]]\nid = \"braces\"\nengine = \"clang-tidy\"\ncheck = \"{check}\"\nkind = \"quality\"\nseverity = \"warning\"\nfix = \"{FIX}\"\n"
        )).unwrap();
        repo
    }

    fn database(&self) {
        let source = self.0.join("a.c");
        let database = serde_json::json!([{
            "directory": self.0,
            "file": source,
            "arguments": ["clang", "-c", source]
        }]);
        fs::write(self.0.join("compile_commands.json"), database.to_string()).unwrap();
    }

    fn run(&self) -> Vec<RuleRun> {
        let file = RuleFile::read(&self.0.join("temperare.toml")).unwrap();
        run_rules_versioned_at(&file, &self.0, &[], None).runs
    }
}

impl Drop for Repo {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
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

/// Set on the child process that runs one temp-git test: that test's name.
const TEMP_GIT_CASE: &str = "ENGINES_CLANG_TIDY_TEMP_GIT_CASE";

/// True in the child that runs `test`; anywhere else, runs `test` in a child of this test binary, asserts it passed
/// and returns false.
fn in_own_process(test: &str) -> bool {
    if std::env::var_os(TEMP_GIT_CASE).is_some_and(|case| case == test) {
        return true;
    }
    // ccvcslib finds git once per process and keeps it, and this case's git lives in a temp folder that is removed,
    // so it runs in its own process.
    let out = procio::run(&procio::Cmd {
        args: vec!["--exact".into(), test.into(), "--test-threads=1".into()],
        env: vec![(TEMP_GIT_CASE.into(), test.into())],
        ..procio::Cmd::new(std::env::current_exe().unwrap())
    }).unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(out.code, Some(0), "{test}: {stdout}{}", String::from_utf8_lossy(&out.stderr));
    assert!(stdout.contains("1 passed"), "{test}: {stdout}");
    false
}

fn assert_finding(runs: Vec<RuleRun>) {
    assert_eq!(runs.len(), 1, "{runs:?}");
    assert_eq!(runs[0].rule, "braces");
    assert_eq!(runs[0].engine, Engine::ClangTidy);
    assert_eq!(runs[0].not_available_cause, None);
    let RuleOutcome::Findings(findings) = &runs[0].outcome else {
        panic!("expected findings, got {:?}", runs[0]);
    };
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(findings[0].rule, "braces");
    assert_eq!(findings[0].engine, Engine::ClangTidy);
    assert_eq!(findings[0].path, RelPath::new("a.c"));
    assert_eq!(findings[0].line(), Some(1));
    assert_eq!(findings[0].col, Some(22));
    assert_eq!(findings[0].what, "statement should be inside braces");
    assert_eq!(findings[0].fix, FIX);
}

fn assert_unavailable(runs: Vec<RuleRun>, expected: &str, cause: NotAvailableCause, exact: bool) {
    assert_eq!(runs.len(), 1, "{runs:?}");
    assert_eq!(runs[0].rule, "braces");
    assert_eq!(runs[0].engine, Engine::ClangTidy);
    assert_eq!(runs[0].not_available_cause, Some(cause));
    let RuleOutcome::NotAvailable { rule, engine, why } = &runs[0].outcome else {
        panic!("expected not available, got {:?}", runs[0]);
    };
    assert_eq!(rule, "braces");
    assert_eq!(*engine, Engine::ClangTidy);
    if exact {
        assert_eq!(why, expected);
    } else {
        assert!(why.contains(expected), "{why}");
    }
}

#[test]
fn violation_with_compile_database() {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let repo = Repo::new("violation", VIOLATION, "readability-braces-around-statements");
    repo.database();
    let runs = repo.run();
    drop(repo);
    assert_finding(runs);
}

#[test]
fn clean_file_with_compile_database() {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let repo = Repo::new("clean", "int f(int a) { if (a) { return 1; } return 0; }\n", "readability-braces-around-statements");
    repo.database();
    let runs = repo.run();
    drop(repo);
    assert_eq!(runs.len(), 1, "{runs:?}");
    assert_eq!(runs[0].rule, "braces");
    assert_eq!(runs[0].engine, Engine::ClangTidy);
    assert_eq!(runs[0].not_available_cause, None);
    assert_eq!(runs[0].outcome, RuleOutcome::Findings(vec![]));
}

#[test]
fn missing_compile_database_is_not_available() {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let repo = Repo::new("no-database", VIOLATION, "readability-braces-around-statements");
    let why = format!("no compile database (compile_commands.json) or compile flags (compile_flags.txt) for {} or any folder above it, so clang-tidy would check it without flags", repo.0.join("a.c").display());
    let runs = repo.run();
    drop(repo);
    assert_unavailable(runs, &why, NotAvailableCause::Other, true);
}

#[test]
fn violation_with_compile_flags() {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let repo = Repo::new("flags", VIOLATION, "readability-braces-around-statements");
    fs::write(repo.0.join("compile_flags.txt"), "-std=c11\n").unwrap();
    let runs = repo.run();
    drop(repo);
    assert_finding(runs);
}

#[test]
fn missing_clang_tidy_is_not_available() {
    if !in_own_process("missing_clang_tidy_is_not_available") {
        return;
    }
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let repo = Repo::new("missing-engine", VIOLATION, "readability-braces-around-statements");
    repo.database();
    let path = PathGuard(std::env::var_os("PATH"));
    let git = std::env::split_paths(path.0.as_ref().unwrap()).map(|dir| dir.join("git"))
        .find(|p| p.metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)).unwrap();
    let bin = repo.0.join("bin");
    fs::create_dir(&bin).unwrap();
    symlink(fs::canonicalize(Path::new(&git)).unwrap(), bin.join("git")).unwrap();
    std::env::set_var("PATH", &bin);
    let runs = repo.run();
    drop(path);
    drop(repo);
    assert_unavailable(runs, "clang-tidy not found on PATH", NotAvailableCause::EngineNotOnPath, true);
}

#[test]
fn no_c_file_precedes_missing_engine() {
    if !in_own_process("no_c_file_precedes_missing_engine") {
        return;
    }
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let repo = Repo::with_file("no-c-file", "README.txt", "tracked\n", "readability-braces-around-statements");
    support::git::git(&repo.0, &["add", "README.txt"], b"");
    let rule_path = repo.0.join("temperare.toml");
    let mut rule_text = fs::read_to_string(&rule_path).unwrap();
    rule_text.push_str("paths = [\"**\"]\n");
    fs::write(&rule_path, rule_text).unwrap();
    let path = PathGuard(std::env::var_os("PATH"));
    let git = std::env::split_paths(path.0.as_ref().unwrap()).map(|dir| dir.join("git"))
        .find(|p| p.metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)).unwrap();
    let bin = repo.0.join("bin");
    fs::create_dir(&bin).unwrap();
    symlink(fs::canonicalize(Path::new(&git)).unwrap(), bin.join("git")).unwrap();
    std::env::set_var("PATH", &bin);
    let file = RuleFile::read(&rule_path).unwrap();
    let runs = run_rules_versioned_at(&file, &repo.0, &[], None).runs;
    let no_files = no_file_rules(&file, &repo.0);
    drop(path);
    drop(repo);

    assert_eq!(runs.len(), 1, "{runs:?}");
    assert_eq!(runs[0].rule, "braces");
    assert_eq!(runs[0].engine, Engine::ClangTidy);
    assert_eq!(runs[0].outcome, RuleOutcome::Findings(vec![]));
    assert_eq!(runs[0].not_available_cause, None);
    assert_eq!(runs[0].no_files.as_deref(), Some("no file its engine checks"));
    assert_eq!(no_files, vec![NoFileRule { rule: "braces".into(), engine: Engine::ClangTidy, why: "no file its engine checks".into() }]);
}

#[test]
fn unknown_check_is_not_available() {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let repo = Repo::new("unknown", VIOLATION, "nosuch-check");
    repo.database();
    let runs = repo.run();
    drop(repo);
    assert_unavailable(runs, "Error: no checks enabled.", NotAvailableCause::UnknownCheck, false);
}
