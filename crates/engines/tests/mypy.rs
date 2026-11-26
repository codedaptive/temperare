#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::PathBuf;
use std::sync::Mutex;

use engines::{no_file_rules, run_rules_versioned_at, NoFileRule, NotAvailableCause, RuleRun};
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

mod support {
    pub mod git;
}

static LOCK: Mutex<()> = Mutex::new(());
const VIOLATION: &str = "def f(x: int) -> int:\n    return x\n\nf(\"s\")\n";
const FIX: &str = "Pass an argument of the expected type.";
/// Set on the child process that runs one git-only case: the name of that case's test.
const GIT_ONLY_CASE: &str = "ENGINES_MYPY_GIT_ONLY_CASE";

/// Runs `case` when this process is the child started for `test`; otherwise starts that child and asserts it passed.
fn in_own_process(test: &str, case: impl FnOnce()) {
    if std::env::var_os(GIT_ONLY_CASE).is_some_and(|name| name == test) {
        return case();
    }
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    // ccvcslib finds git once per process and keeps it, and this case's git lives in a temp folder that is removed,
    // so it runs in its own process.
    let out = procio::run(&procio::Cmd {
        args: vec!["--exact".into(), test.into(), "--test-threads=1".into()],
        env: vec![(GIT_ONLY_CASE.into(), test.into())],
        ..procio::Cmd::new(std::env::current_exe().unwrap())
    }).unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(out.code, Some(0), "{test}: {stdout}{}", String::from_utf8_lossy(&out.stderr));
    assert!(stdout.contains("1 passed"), "{test}: {stdout}");
}

struct Repo(PathBuf);

impl Repo {
    fn new(case: &str, filename: &str, source: &str, check: &str) -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-mypy-{case}", std::process::id()));
        fs::create_dir(&path).unwrap();
        let repo = Self(fs::canonicalize(path).unwrap());
        support::git::git(&repo.0, &["init", "-q"], b"");
        fs::write(repo.0.join(filename), source).unwrap();
        fs::write(repo.0.join("temperare.toml"), format!(
            "version = 1\n[[rule]]\nid = \"types\"\nengine = \"mypy\"\ncheck = \"{check}\"\nkind = \"quality\"\nseverity = \"warning\"\nfix = \"{FIX}\"\n"
        )).unwrap();
        repo
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

impl PathGuard {
    fn with_mypy() -> Self {
        let guard = Self(std::env::var_os("PATH"));
        let path = std::env::join_paths(
            std::iter::once(PathBuf::from("/opt/homebrew/bin"))
                .chain(std::env::split_paths(guard.0.as_ref().unwrap())),
        ).unwrap();
        std::env::set_var("PATH", path);
        guard
    }

    fn git_only(repo: &Repo) -> Self {
        let guard = Self(std::env::var_os("PATH"));
        let git = std::env::split_paths(guard.0.as_ref().unwrap()).map(|dir| dir.join("git"))
            .find(|p| p.metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)).unwrap();
        let bin = repo.0.join("bin");
        fs::create_dir(&bin).unwrap();
        symlink(fs::canonicalize(git).unwrap(), bin.join("git")).unwrap();
        std::env::set_var("PATH", bin);
        guard
    }
}

impl Drop for PathGuard {
    fn drop(&mut self) {
        match &self.0 {
            Some(path) => std::env::set_var("PATH", path),
            None => std::env::remove_var("PATH"),
        }
    }
}

fn assert_finding(runs: Vec<RuleRun>) {
    assert_eq!(runs.len(), 1, "{runs:?}");
    assert_eq!(runs[0].rule, "types");
    assert_eq!(runs[0].engine, Engine::Mypy);
    assert_eq!(runs[0].not_available_cause, None);
    assert!(matches!(&runs[0].outcome, RuleOutcome::Findings(_)), "{runs:?}");
    let RuleOutcome::Findings(findings) = &runs[0].outcome else { unreachable!() };
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(findings[0].rule, "types");
    assert_eq!(findings[0].engine, Engine::Mypy);
    assert_eq!(findings[0].path, RelPath::new("a.py"));
    assert_eq!(findings[0].line(), Some(4));
    assert_eq!(findings[0].col, Some(3));
    assert_eq!(findings[0].what, "Argument 1 to \"f\" has incompatible type \"str\"; expected \"int\"");
    assert_eq!(findings[0].fix, FIX);
}

fn assert_unavailable(runs: Vec<RuleRun>, expected: &str, cause: NotAvailableCause, exact: bool) {
    assert_eq!(runs.len(), 1, "{runs:?}");
    assert_eq!(runs[0].rule, "types");
    assert_eq!(runs[0].engine, Engine::Mypy);
    assert_eq!(runs[0].not_available_cause, Some(cause));
    assert!(matches!(&runs[0].outcome, RuleOutcome::NotAvailable { .. }), "{runs:?}");
    let RuleOutcome::NotAvailable { rule, engine, why } = &runs[0].outcome else { unreachable!() };
    assert_eq!(rule, "types");
    assert_eq!(*engine, Engine::Mypy);
    if exact { assert_eq!(why, expected); } else { assert!(why.contains(expected), "{why}"); }
}

fn assert_failed(runs: Vec<RuleRun>, expected: &str, exact: bool) {
    assert_eq!(runs.len(), 1, "{runs:?}");
    assert_eq!(runs[0].rule, "types");
    assert_eq!(runs[0].engine, Engine::Mypy);
    assert_eq!(runs[0].not_available_cause, None);
    assert!(matches!(&runs[0].outcome, RuleOutcome::Failed { .. }), "{runs:?}");
    let RuleOutcome::Failed { rule, engine, stderr, .. } = &runs[0].outcome else { unreachable!() };
    assert_eq!(rule, "types");
    assert_eq!(*engine, Engine::Mypy);
    if exact { assert_eq!(stderr, expected); } else { assert!(stderr.contains(expected), "{stderr}"); }
}

#[test]
fn violation_is_found() {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let _path = PathGuard::with_mypy();
    let repo = Repo::new("violation", "a.py", VIOLATION, "arg-type");
    assert_finding(repo.run());
}

#[test]
fn clean_file_passes() {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let _path = PathGuard::with_mypy();
    let repo = Repo::new("clean", "a.py", "def f(x: int) -> int:\n    return x\n\nf(1)\n", "arg-type");
    let runs = repo.run();
    assert_eq!(runs.len(), 1, "{runs:?}");
    assert_eq!(runs[0].rule, "types");
    assert_eq!(runs[0].engine, Engine::Mypy);
    assert_eq!(runs[0].not_available_cause, None);
    assert_eq!(runs[0].outcome, RuleOutcome::Findings(vec![]));
}

#[test]
fn missing_mypy_is_not_available() {
    in_own_process("missing_mypy_is_not_available", || {
        let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let repo = Repo::new("missing", "a.py", VIOLATION, "arg-type");
        let _path = PathGuard::git_only(&repo);
        assert_unavailable(repo.run(), "mypy not found on PATH", NotAvailableCause::EngineNotOnPath, true);
    });
}

#[test]
fn no_python_file_precedes_missing_engine() {
    in_own_process("no_python_file_precedes_missing_engine", no_python_file_case);
}

fn no_python_file_case() {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let repo = Repo::new("no-python", "README.txt", "tracked\n", "arg-type");
    support::git::git(&repo.0, &["add", "README.txt"], b"");
    let rule_path = repo.0.join("temperare.toml");
    let mut rule_text = fs::read_to_string(&rule_path).unwrap();
    rule_text.push_str("paths = [\"**\"]\n");
    fs::write(&rule_path, rule_text).unwrap();
    let _path = PathGuard::git_only(&repo);
    let file = RuleFile::read(&rule_path).unwrap();
    let runs = run_rules_versioned_at(&file, &repo.0, &[], None).runs;
    let no_files = no_file_rules(&file, &repo.0);
    assert_eq!(runs.len(), 1, "{runs:?}");
    assert_eq!(runs[0].rule, "types");
    assert_eq!(runs[0].engine, Engine::Mypy);
    assert_eq!(runs[0].outcome, RuleOutcome::Findings(vec![]));
    assert_eq!(runs[0].not_available_cause, None);
    assert_eq!(runs[0].no_files.as_deref(), Some("no file its engine checks"));
    assert_eq!(no_files, vec![NoFileRule { rule: "types".into(), engine: Engine::Mypy, why: "no file its engine checks".into() }]);
}

#[test]
fn unknown_code_is_not_available() {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let _path = PathGuard::with_mypy();
    let repo = Repo::new("unknown", "a.py", VIOLATION, "nonsense-code");
    assert_unavailable(repo.run(), "Invalid error code(s): nonsense-code", NotAvailableCause::UnknownCheck, false);
}

#[test]
fn config_ignore_errors_fails_the_rule() {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let _path = PathGuard::with_mypy();
    let repo = Repo::new("ignore-errors", "a.py", VIOLATION, "arg-type");
    fs::write(repo.0.join("mypy.ini"), "[mypy]\n[mypy-a]\nignore_errors = True\n").unwrap();
    assert_failed(repo.run(), "mypy config mypy.ini: [mypy-a] ignore_errors silences mypy findings outside the rule file; exceptions go in the rule file", true);
}

#[test]
fn config_exclude_does_not_hide() {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let _path = PathGuard::with_mypy();
    let repo = Repo::new("exclude", "a.py", VIOLATION, "arg-type");
    fs::write(repo.0.join("mypy.ini"), "[mypy]\nexclude = ^a\\.py$\n").unwrap();
    assert_finding(repo.run());
}

#[test]
fn blocking_syntax_error_fails_the_rule() {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let _path = PathGuard::with_mypy();
    let repo = Repo::new("syntax", "a.py", VIOLATION, "arg-type");
    fs::write(repo.0.join("b.py"), "def g(:\n").unwrap();
    assert_failed(repo.run(), "mypy stopped at a blocking error, so it checked nothing for `arg-type`: b.py:1: Expected a parameter or the end of the parameter list", true);
}
