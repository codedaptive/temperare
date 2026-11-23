#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};

use engines::{no_file_rules, run_rules_versioned_at, NoFileRule, NotAvailableCause};
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

mod support {
    pub mod git;
}

static LOCK: Mutex<()> = Mutex::new(());
const FIX: &str = "Use a single space.";
const DIALECT: &str = "[sqlfluff]\ndialect = ansi\n";
const VIOLATION: &str = "SELECT a  FROM t\n";
/// 25,500 bytes: over sqlfluff's default `large_file_skip_byte_limit` (20,000), so sqlfluff skips the file unreported.
static LARGE: LazyLock<String> = LazyLock::new(|| "SELECT a FROM t;\n".repeat(1500));

struct Repo(PathBuf);

impl Repo {
    fn new(name: &str, filename: &str, source: &str, check: &str) -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-sqlfluff-{name}", std::process::id()));
        fs::create_dir(&path).unwrap();
        let repo = Self(fs::canonicalize(path).unwrap());
        support::git::git(&repo.0, &["init", "-q"], b"");
        fs::write(repo.0.join(filename), source).unwrap();
        fs::write(repo.0.join("temperare.toml"), format!(
            "version = 1\n[[rule]]\nid = \"spacing\"\nengine = \"sqlfluff\"\ncheck = \"{check}\"\nkind = \"quality\"\nseverity = \"warning\"\nfix = \"{FIX}\"\npaths = [\"**\"]\n"
        )).unwrap();
        repo
    }
}

impl Drop for Repo {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

struct PathGuard(Option<OsString>);

impl PathGuard {
    fn with_sqlfluff() -> Self {
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

#[derive(Clone, Copy)]
enum Expected {
    Finding,
    Clean,
    Failed,
    FailedSaying(&'static str),
    NoDialect,
    UnknownCheck,
    MissingEngine,
    NoFiles,
}

struct Case {
    name: &'static str,
    check: &'static str,
    source: &'static str,
    config: Option<&'static str>,
    ignore: bool,
    absent: bool,
    expected: Expected,
}

fn run_case(case: Case) {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let no_files = matches!(case.expected, Expected::NoFiles);
    let filename = if no_files { "README.txt" } else { "q.sql" };
    let repo = Repo::new(case.name, filename, case.source, case.check);
    if let Some(config) = case.config {
        fs::write(repo.0.join(".sqlfluff"), config).unwrap();
    }
    if case.ignore {
        fs::write(repo.0.join(".sqlfluffignore"), "q.sql\n").unwrap();
    }
    if no_files {
        support::git::git(&repo.0, &["add", "README.txt"], b"");
    }
    let _path = if case.absent { PathGuard::git_only(&repo) } else { PathGuard::with_sqlfluff() };
    let file = RuleFile::read(&repo.0.join("temperare.toml")).unwrap();
    let runs = run_rules_versioned_at(&file, &repo.0, &[], None).runs;
    assert_eq!(runs.len(), 1, "{}: {runs:?}", case.name);
    let run = &runs[0];
    assert_eq!(run.rule, "spacing", "{}", case.name);
    assert_eq!(run.engine, Engine::Sqlfluff, "{}", case.name);
    match case.expected {
        Expected::Finding => {
            assert_eq!(run.not_available_cause, None, "{}: {run:?}", case.name);
            let RuleOutcome::Findings(findings) = &run.outcome else { panic!("{}: {run:?}", case.name) };
            assert_eq!(findings.len(), 1, "{}: {findings:?}", case.name);
            let finding = &findings[0];
            assert_eq!(finding.rule, "spacing");
            assert_eq!(finding.engine, Engine::Sqlfluff);
            assert_eq!(finding.path, RelPath::new("q.sql"));
            assert_eq!(finding.line(), Some(1));
            assert_eq!(finding.col, Some(9));
            assert!(finding.what.contains("Expected only single space before 'FROM' keyword"), "{}", finding.what);
            assert_eq!(finding.fix, FIX);
        }
        Expected::Clean => {
            assert_eq!(run.not_available_cause, None, "{}: {run:?}", case.name);
            assert_eq!(run.outcome, RuleOutcome::Findings(vec![]), "{}", case.name);
        }
        Expected::Failed => {
            assert_eq!(run.not_available_cause, None, "{}: {run:?}", case.name);
            assert!(matches!(&run.outcome, RuleOutcome::Failed { .. }), "{}: {run:?}", case.name);
        }
        Expected::FailedSaying(text) => {
            assert_eq!(run.not_available_cause, None, "{}: {run:?}", case.name);
            let RuleOutcome::Failed { stderr, .. } = &run.outcome else { panic!("{}: {run:?}", case.name) };
            assert!(stderr.contains(text), "{}: {run:?}", case.name);
        }
        Expected::NoDialect | Expected::UnknownCheck | Expected::MissingEngine => {
            let RuleOutcome::NotAvailable { rule, engine, why } = &run.outcome else { panic!("{}: {run:?}", case.name) };
            assert_eq!(rule, "spacing");
            assert_eq!(*engine, Engine::Sqlfluff);
            match case.expected {
                Expected::NoDialect => {
                    assert!(why.contains(".sqlfluff") && why.contains("dialect"), "{why}");
                }
                Expected::UnknownCheck => {
                    assert_eq!(run.not_available_cause, Some(NotAvailableCause::UnknownCheck), "{run:?}");
                    assert!(why.contains("NOPE99"), "{why}");
                }
                Expected::MissingEngine => {
                    assert_eq!(run.not_available_cause, Some(NotAvailableCause::EngineNotOnPath), "{run:?}");
                    assert_eq!(why, "sqlfluff not found on PATH");
                }
                _ => unreachable!(),
            }
        }
        Expected::NoFiles => {
            assert_eq!(run.outcome, RuleOutcome::Findings(vec![]));
            assert_eq!(run.not_available_cause, None);
            assert_eq!(run.no_files.as_deref(), Some("no file its engine checks"));
            assert_eq!(no_file_rules(&file, &repo.0), vec![NoFileRule {
                rule: "spacing".into(), engine: Engine::Sqlfluff, why: "no file its engine checks".into(),
            }]);
        }
    }
}

macro_rules! cases {
    ($( $test:ident: ($name:expr, $check:expr, $source:expr, $config:expr,
        $ignore:expr, $absent:expr, $expected:ident $(($saying:expr))?) ),+ $(,)?) => {
        $(#[test]
        fn $test() {
            run_case(Case { name: $name, check: $check, source: $source, config: $config,
                ignore: $ignore, absent: $absent, expected: Expected::$expected $(($saying))? });
        })+
    };
}

cases! {
    code_finds_spacing: ("code", "LT01", VIOLATION, Some(DIALECT), false, false, Finding),
    name_finds_spacing: ("name", "layout.spacing", VIOLATION, Some(DIALECT), false, false, Finding),
    clean_file: ("clean", "LT01", "SELECT a FROM t\n", Some(DIALECT), false, false, Clean),
    inline_noqa_does_not_suppress: ("noqa", "LT01", "SELECT a  FROM t -- noqa: LT01\n", Some(DIALECT), false, false, Finding),
    ignore_file_does_not_suppress: ("ignore-file", "LT01", VIOLATION, Some(DIALECT), true, false, Finding),
    exclude_rules_does_not_suppress: ("exclude", "LT01", VIOLATION, Some("[sqlfluff]\ndialect = ansi\nexclude_rules = LT01\n"), false, false, Finding),
    ignored_parsing_error_fails: ("parsing", "LT01", "SELECT FROM WHERE (((\n", Some("[sqlfluff]\ndialect = ansi\nignore = parsing\n"), false, false, Failed),
    dbt_templater_fails: (
        "dbt", "LT01", VIOLATION, Some("[sqlfluff]\ndialect = ansi\ntemplater = dbt\n"),
        false, false, FailedSaying("sqlfluff-templater-dbt")
    ),
    missing_dialect_is_not_available: ("dialect", "LT01", VIOLATION, None, false, false, NoDialect),
    unknown_check_is_not_available: ("unknown", "NOPE99", VIOLATION, Some(DIALECT), false, false, UnknownCheck),
    large_file_fails: ("large", "LT01", LARGE.as_str(), Some(DIALECT), false, false, Failed),
    missing_engine_is_not_available: ("missing", "LT01", VIOLATION, Some(DIALECT), false, true, MissingEngine),
    no_sql_file_precedes_missing_engine: ("no-sql", "LT01", "tracked\n", Some(DIALECT), false, true, NoFiles),
}
