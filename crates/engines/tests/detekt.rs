#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};

use engines::{run_rules_versioned_at, NotAvailableCause, RuleRun};
use rules::{Engine, Finding, RelPath, RuleFile, RuleOutcome};

#[path = "support/git.rs"]
mod git_support;

static LOCK: Mutex<()> = Mutex::new(());
const DEMO: &str = "package demo\n\nfun area(r: Double): Double {\n    return r * r * 3.14159\n}\n";
const FIX: &str = "Replace the magic number with a named constant.";
const MESSAGE: &str = "This expression contains a magic number. Consider defining it to a well named constant.";

struct PathGuard(Option<OsString>);

impl Drop for PathGuard {
    fn drop(&mut self) {
        match &self.0 {
            Some(path) => std::env::set_var("PATH", path),
            None => std::env::remove_var("PATH"),
        }
    }
}

struct Repo {
    path: PathBuf,
    _path_guard: PathGuard,
    _lock: MutexGuard<'static, ()>,
}

impl Repo {
    fn new(case: &str, check: &str) -> Self {
        let lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let path_guard = PathGuard(std::env::var_os("PATH"));
        std::env::set_var("PATH", std::env::join_paths(
            std::iter::once(PathBuf::from("/opt/homebrew/bin"))
                .chain(std::env::split_paths(path_guard.0.as_deref().unwrap_or_default())),
        ).unwrap());
        let path = std::env::temp_dir().join(format!("engines-zit-{}-detekt-{case}", std::process::id()));
        fs::create_dir(&path).unwrap();
        let repo = Self { path: fs::canonicalize(path).unwrap(), _path_guard: path_guard, _lock: lock };
        git_support::git(&repo.path, &["init", "-q"], b"");
        repo.write("temperare.toml", &format!(
            "version = 1\n[[rule]]\nid = \"magic-number\"\nengine = \"detekt\"\ncheck = \"{check}\"\npaths = [\"**\"]\nfix = \"{FIX}\"\n"
        ));
        repo
    }

    fn write(&self, name: &str, contents: &str) {
        let path = self.path.join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, contents).unwrap();
    }

    fn without_detekt(&self) {
        let path = std::env::var_os("PATH").unwrap();
        let filtered = std::env::join_paths(std::env::split_paths(&path).filter(|dir| {
            !dir.join("detekt").metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
        })).unwrap();
        std::env::set_var("PATH", filtered);
    }

    fn run(&self) -> RuleRun {
        let file = RuleFile::read(&self.path.join("temperare.toml")).unwrap();
        let mut runs = run_rules_versioned_at(&file, &self.path, &[], None).runs;
        assert_eq!(runs.len(), 1, "{runs:?}");
        runs.remove(0)
    }
}

impl Drop for Repo {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).unwrap();
    }
}

fn is_magic_number(finding: &Finding) -> bool {
    finding.rule == "magic-number"
        && finding.engine == Engine::Detekt
        && finding.path == RelPath::new("src/Demo.kt")
        && finding.line() == Some(4)
        && finding.col == Some(20)
        && finding.what == MESSAGE
        && finding.fix == FIX
}

fn assert_demo(run: RuleRun, exact: bool) {
    assert!(matches!(&run.outcome, RuleOutcome::Findings(_)), "{run:?}");
    let RuleOutcome::Findings(findings) = &run.outcome else { unreachable!() };
    assert_eq!(run.no_files, None);
    if exact {
        assert_eq!(findings.len(), 1, "{findings:?}");
    }
    assert!(findings.iter().any(is_magic_number), "{findings:?}");
}

#[test]
fn magic_number_violation() {
    let repo = Repo::new("violation", "MagicNumber");
    repo.write("src/Demo.kt", DEMO);
    assert_demo(repo.run(), true);
}

#[test]
fn clean_kotlin_file() {
    let repo = Repo::new("clean", "MagicNumber");
    repo.write("src/Clean.kt", "package demo\n\nconst val ONE = 1\n");
    let run = repo.run();
    assert_eq!(run.outcome, RuleOutcome::Findings(vec![]), "{run:?}");
    assert_eq!(run.no_files, None);
}

#[test]
fn rule_set_with_no_typed_rule_runs() {
    let repo = Repo::new("untyped-rule-set", "empty-blocks");
    repo.write("src/Empty.kt", "package demo\n\nfun nothing() {\n}\n");
    let run = repo.run();
    assert!(matches!(&run.outcome, RuleOutcome::Findings(_)), "{run:?}");
    let RuleOutcome::Findings(findings) = &run.outcome else { unreachable!() };
    assert_eq!(run.no_files, None);
    assert_eq!(findings.len(), 1, "{findings:?}");
    let finding = &findings[0];
    assert_eq!(finding.rule, "magic-number");
    assert_eq!(finding.engine, Engine::Detekt);
    assert_eq!(finding.path, RelPath::new("src/Empty.kt"));
    assert_eq!(finding.line(), Some(3));
    assert_eq!(finding.col, Some(15));
    assert_eq!(finding.what, "This empty block of code can be removed.");
    assert_eq!(finding.fix, FIX);
}

#[test]
fn rule_set_with_typed_rules_is_not_available() {
    let repo = Repo::new("typed-rule-set", "style");
    repo.write("src/Demo.kt", DEMO);
    let run = repo.run();
    assert!(matches!(&run.outcome, RuleOutcome::NotAvailable { .. }), "{run:?}");
    assert_eq!(run.not_available_cause, Some(NotAvailableCause::Other), "{run:?}");
    let RuleOutcome::NotAvailable { why, .. } = &run.outcome else { unreachable!() };
    assert!(why.contains("VarCouldBeVal"), "{why}");
}

#[test]
fn repo_config_cannot_switch_off_or_exclude_picked_rule() {
    let repo = Repo::new("config-suppression", "MagicNumber");
    repo.write("src/Demo.kt", DEMO);
    repo.write("detekt.yml", "style:\n  active: false\n  MagicNumber:\n    active: false\n    excludes: ['**/Demo.kt']\n");
    assert_demo(repo.run(), true);
}

#[test]
fn repo_baseline_is_not_applied() {
    let repo = Repo::new("baseline", "MagicNumber");
    repo.write("src/Demo.kt", DEMO);
    repo.write("detekt-baseline.xml", "<?xml version=\"1.0\" ?>\n<SmellBaseline>\n  <ManuallySuppressedIssues/>\n  <CurrentIssues>\n    <ID>MagicNumber:Demo.kt$3.14159</ID>\n  </CurrentIssues>\n</SmellBaseline>\n");
    assert_demo(repo.run(), true);
}

#[test]
fn git_ignored_kotlin_is_not_checked() {
    let repo = Repo::new("ignored", "MagicNumber");
    repo.write("src/Demo.kt", DEMO);
    repo.write(".gitignore", "build/\n");
    repo.write("build/Gen.kt", "package demo\n\nfun generated() = 3.14159\n");
    assert_demo(repo.run(), true);
}

#[test]
fn unlisted_check_is_not_available() {
    let repo = Repo::new("unlisted", "NoSuchRule");
    repo.write("src/Demo.kt", DEMO);
    let run = repo.run();
    assert!(matches!(&run.outcome, RuleOutcome::NotAvailable { .. }), "{run:?}");
    assert_eq!(run.not_available_cause, Some(NotAvailableCause::Other), "{run:?}");
    let RuleOutcome::NotAvailable { why, .. } = &run.outcome else { unreachable!() };
    let (prefix, _) = why.split_once(": ").unwrap();
    assert_eq!(prefix, "detekt 1.23.8", "{why}");
    assert!(why.contains("NoSuchRule is not in temperare's list of detekt codes (taken from detekt 1.23.8)"), "{why}");
}

#[test]
fn malformed_check_is_refused() {
    let repo = Repo::new("malformed", "detekt.style.MagicNumber");
    repo.write("src/Demo.kt", DEMO);
    let run = repo.run();
    assert!(matches!(run.outcome, RuleOutcome::Failed { .. }), "{run:?}");
}

#[test]
fn missing_detekt_is_not_available() {
    let repo = Repo::new("absent", "MagicNumber");
    repo.write("src/Demo.kt", DEMO);
    repo.without_detekt();
    let run = repo.run();
    assert!(matches!(run.outcome, RuleOutcome::NotAvailable { .. }), "{run:?}");
    assert_eq!(run.not_available_cause, Some(NotAvailableCause::EngineNotOnPath), "{run:?}");
}

#[test]
fn no_kotlin_file_precedes_missing_detekt() {
    let repo = Repo::new("no-kotlin", "MagicNumber");
    repo.write("README.txt", "no Kotlin here\n");
    git_support::git(&repo.path, &["add", "README.txt"], b"");
    repo.without_detekt();
    let run = repo.run();
    assert_eq!(run.outcome, RuleOutcome::Findings(vec![]), "{run:?}");
    assert_eq!(run.not_available_cause, None);
    assert_eq!(run.no_files.as_deref(), Some("no file its engine checks"));
}

#[test]
fn invalid_other_rule_in_repo_config_fails() {
    let repo = Repo::new("invalid-config", "MagicNumber");
    repo.write("src/Demo.kt", DEMO);
    repo.write("detekt.yml", "style:\n  NoSuchRuleXyz:\n    active: true\n");
    let run = repo.run();
    assert!(matches!(run.outcome, RuleOutcome::Failed { .. }), "{run:?}");
}

#[test]
fn type_resolution_rule_is_not_available() {
    let repo = Repo::new("type-resolution", "Deprecation");
    repo.write("src/Dep.kt", "package demo\n\n@Deprecated(\"use newer\")\nfun older(): Int = 1\n\nfun caller(): Int = older()\n");
    let run = repo.run();
    assert!(matches!(run.outcome, RuleOutcome::NotAvailable { .. }), "{run:?}");
    assert_eq!(run.not_available_cause, Some(NotAvailableCause::Other), "{run:?}");
}
