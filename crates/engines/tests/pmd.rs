#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use engines::{no_file_rules, run_rules_versioned_at, NoFileRule, NotAvailableCause};
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

#[path = "support/git.rs"]
mod git_support;

static LOCK: Mutex<()> = Mutex::new(());
const CHECK: &str = "category/java/bestpractices.xml/UnusedPrivateField";
const VIOLATION: &str = "class Foo {\n    private int unused;\n}\n";
const NOPMD: &str = "class Foo {\n    private int unused; // NOPMD\n}\n";
const CLEAN: &str = "class Foo {\n    int value;\n}\n";

struct Repo(PathBuf);

impl Repo {
    fn new(name: &str, check: &str, source: Option<&str>, ignored: bool, tracked: bool) -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-pmd-{name}", std::process::id()));
        fs::create_dir(&path).unwrap();
        let repo = Self(fs::canonicalize(path).unwrap());
        git_support::git(&repo.0, &["init", "-q"], b"");
        fs::create_dir_all(repo.0.join("kit/src")).unwrap();
        fs::write(repo.0.join("kit/temperare.toml"), format!(
            "version = 1\n[[rule]]\nid = \"unused-field\"\nengine = \"pmd\"\ncheck = \"{check}\"\nfix = \"Remove the field.\"\n"
        )).unwrap();
        if let Some(source) = source {
            fs::write(repo.0.join("kit/src/Foo.java"), source).unwrap();
            if tracked {
                git_support::git(&repo.0, &["add", "kit/src/Foo.java"], b"");
            }
        } else {
            fs::write(repo.0.join("kit/README.txt"), "No Java files.\n").unwrap();
        }
        if ignored {
            fs::write(repo.0.join(".gitignore"), "kit/src/Ignored.java\n").unwrap();
            fs::write(repo.0.join("kit/src/Ignored.java"), VIOLATION).unwrap();
        }
        repo
    }
}

impl Drop for Repo {
    fn drop(&mut self) { fs::remove_dir_all(&self.0).unwrap(); }
}

struct PathGuard(Option<OsString>);

impl PathGuard {
    fn set(repo: &Repo, absent: bool) -> Self {
        let original = Self(std::env::var_os("PATH"));
        let path = original.0.as_deref().unwrap_or_default();
        if absent {
            let git = std::env::split_paths(path).map(|dir| dir.join("git"))
                .find(|p| p.metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)).unwrap();
            let bin = repo.0.join("bin");
            fs::create_dir(&bin).unwrap();
            symlink(fs::canonicalize(git).unwrap(), bin.join("git")).unwrap();
            std::env::set_var("PATH", bin);
        } else {
            std::env::set_var("PATH", std::env::join_paths(
                std::iter::once(Path::new("/opt/homebrew/bin").to_path_buf()).chain(std::env::split_paths(path))
            ).unwrap());
        }
        original
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

#[derive(Clone, Copy)]
enum Expected { Finding, CategoryFinding, Clean, Unknown, Failed, NoFiles, Absent }

struct Case {
    name: &'static str,
    check: &'static str,
    source: Option<&'static str>,
    ignored: bool,
    tracked: bool,
    absent: bool,
    expected: Expected,
}

/// Set on the child process that runs one PATH-narrowed case: the name of that case's test.
const GIT_ONLY_CASE: &str = "ENGINES_PMD_GIT_ONLY_CASE";

/// Runs the named case's test alone in a child of this test binary, with GIT_ONLY_CASE naming it.
fn run_in_child(name: &str) {
    let out = procio::run(&procio::Cmd {
        args: vec!["--exact".into(), name.into(), "--test-threads=1".into()],
        env: vec![(GIT_ONLY_CASE.into(), name.into())],
        ..procio::Cmd::new(std::env::current_exe().unwrap())
    }).unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(out.code, Some(0), "{name}: {stdout}{}", String::from_utf8_lossy(&out.stderr));
    assert!(stdout.contains("1 passed"), "{name}: {stdout}");
}

fn run_case(case: Case) {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    if case.absent && std::env::var(GIT_ONLY_CASE).as_deref() != Ok(case.name) {
        // ccvcslib finds git once per process and keeps it, and this case's git lives in a temp folder that is
        // removed, so it runs in its own process.
        return run_in_child(case.name);
    }
    let repo = Repo::new(case.name, case.check, case.source, case.ignored, case.tracked);
    let _path = PathGuard::set(&repo, case.absent);
    let file = RuleFile::read(&repo.0.join("kit/temperare.toml")).unwrap();
    let runs = run_rules_versioned_at(&file, &repo.0, &[], None).runs;
    let no_files = no_file_rules(&file, &repo.0);
    assert_eq!(runs.len(), 1, "{}: {runs:?}", case.name);
    let run = &runs[0];
    assert_eq!(run.rule, "unused-field");
    assert_eq!(run.engine, Engine::Pmd);
    match case.expected {
        Expected::Finding | Expected::CategoryFinding => {
            assert!(matches!(&run.outcome, RuleOutcome::Findings(_)), "{}: {:?}", case.name, run.outcome);
            let RuleOutcome::Findings(findings) = &run.outcome else { unreachable!() };
            if matches!(case.expected, Expected::Finding) { assert_eq!(findings.len(), 1, "{findings:?}"); }
            let finding = findings.iter().find(|f| f.what == "Avoid unused private fields such as 'unused'.");
            assert!(finding.is_some(), "{}: {findings:?}", case.name);
            let finding = finding.unwrap();
            assert_eq!(finding.rule, "unused-field");
            assert_eq!(finding.path, RelPath::new("src/Foo.java"));
            assert_eq!(finding.line(), Some(2));
            assert_eq!(finding.col, Some(17));
            assert_eq!(finding.what, "Avoid unused private fields such as 'unused'.");
            assert_eq!(finding.fix, "Remove the field.", "{}", case.name);
            assert_eq!(run.no_files, None);
        }
        Expected::Clean => {
            assert_eq!(run.outcome, RuleOutcome::Findings(vec![]));
            assert_eq!(run.no_files, None);
        }
        Expected::Unknown | Expected::Absent => {
            assert!(matches!(&run.outcome, RuleOutcome::NotAvailable { rule, engine: Engine::Pmd, .. } if rule == "unused-field"), "{}: {:?}", case.name, run.outcome);
            let cause = if matches!(case.expected, Expected::Unknown) { NotAvailableCause::UnknownCheck } else { NotAvailableCause::EngineNotOnPath };
            assert_eq!(run.not_available_cause, Some(cause));
        }
        Expected::Failed => assert!(matches!(&run.outcome, RuleOutcome::Failed { .. }), "{}: {:?}", case.name, run.outcome),
        Expected::NoFiles => {
            assert_eq!(run.outcome, RuleOutcome::Findings(vec![]));
            assert_eq!(run.not_available_cause, None);
            assert_eq!(run.no_files.as_deref(), Some("no file its engine checks"));
            assert_eq!(no_files, vec![NoFileRule { rule: "unused-field".into(), engine: Engine::Pmd, why: "no file its engine checks".into() }]);
        }
    }
}

macro_rules! cases {
    ($( $name:ident: ($check:expr, $source:expr, $ignored:expr, $tracked:expr, $absent:expr, $expected:ident) ),+ $(,)?) => {
        $(#[test] fn $name() {
            run_case(Case { name: stringify!($name), check: $check, source: $source, ignored: $ignored,
                tracked: $tracked, absent: $absent, expected: Expected::$expected });
        })+
    };
}

cases! {
    violation: (CHECK, Some(VIOLATION), false, true, false, Finding),
    clean: (CHECK, Some(CLEAN), false, false, false, Clean),
    nopmd_is_not_an_exception: (CHECK, Some(NOPMD), false, false, false, Finding),
    ignored_java_is_not_checked: (CHECK, Some(CLEAN), true, true, false, Clean),
    whole_category: ("category/java/bestpractices.xml", Some(VIOLATION), false, false, false, CategoryFinding),
    comma_separated_checks: ("category/java/bestpractices.xml/UnusedPrivateField,category/java/bestpractices.xml/UnusedLocalVariable", Some(VIOLATION), false, false, false, Finding),
    unknown_rule: ("category/java/bestpractices.xml/NoSuchRule", Some(CLEAN), false, false, false, Unknown),
    unknown_category: ("category/java/nosuch.xml/Foo", Some(CLEAN), false, false, false, Unknown),
    ruleset_path_refused: ("ruleset.xml", Some(CLEAN), false, false, false, Failed),
    no_java: (CHECK, None, false, false, false, NoFiles),
    no_java_before_missing_pmd: (CHECK, None, false, false, true, NoFiles),
    missing_pmd: (CHECK, Some(CLEAN), false, false, true, Absent),
}
