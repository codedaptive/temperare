#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use engines::{no_file_rules, run_rules_versioned_at, NoFileRule, NotAvailableCause};
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

#[path = "support/git.rs"]
mod git_support;
#[path = "support/env_guard.rs"]
mod env_guard;

use env_guard::EnvGuard;

static LOCK: Mutex<()> = Mutex::new(());
const FIX: &str = "Tag the image.";
const VIOLATION: &str = "FROM ubuntu\n";
const MESSAGE: &str = "Always tag the version of an image explicitly";
/// Set on the child process that runs one git-only case: the name of that case's test.
const GIT_ONLY_CASE: &str = "ENGINES_HADOLINT_GIT_ONLY_CASE";

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
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-hadolint-{name}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
        fs::create_dir(&path).unwrap();
        let repo = Self(fs::canonicalize(path).unwrap());
        git_support::git(&repo.0, &["init", "-q"], b"");
        repo
    }

    fn write(&self, name: &str, contents: &str) {
        let path = self.0.join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, contents).unwrap();
    }
}

impl Drop for Repo {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

struct PathGuard(Option<OsString>);

impl PathGuard {
    fn with_hadolint() -> Self {
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
    Hit { line: Option<u32>, message: Option<&'static str> },
    Clean,
    Missing,
    NoFiles,
    Failed(Option<&'static str>),
    Refused,
    NotListed { shellcheck: bool },
}

struct Case {
    file: &'static str,
    source: &'static str,
    check: &'static str,
    config: Option<(&'static str, &'static str)>,
    extra: Option<(&'static str, &'static str)>,
    env: Option<(&'static str, &'static str)>,
    expected: Expected,
}

fn run_case(name: &str, case: Case) {
    if matches!(case.expected, Expected::Missing | Expected::NoFiles) {
        return in_own_process(name, || check_case(name, case));
    }
    check_case(name, case);
}

fn check_case(name: &str, case: Case) {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let _env = case.env.map(|(name, value)| EnvGuard::set(name, value));
    let repo = Repo::new(name);
    let _path = if matches!(case.expected, Expected::Missing | Expected::NoFiles) {
        PathGuard::git_only(&repo)
    } else {
        PathGuard::with_hadolint()
    };
    repo.write(case.file, case.source);
    if let Some((path, source)) = case.config { repo.write(path, source); }
    if let Some((path, source)) = case.extra { repo.write(path, source); }
    repo.write("tao.toml", &format!(
        "version = 1\n[[rule]]\nid = \"docker\"\nengine = \"hadolint\"\ncheck = \"{}\"\npaths = [\"**\"]\nfix = \"{FIX}\"\n", case.check
    ));
    let file = RuleFile::read(&repo.0.join("tao.toml")).unwrap();
    let runs = run_rules_versioned_at(&file, &repo.0, &[], None).runs;
    assert_eq!(runs.len(), 1, "{name}: {runs:?}");
    let run = &runs[0];
    assert_eq!(run.rule, "docker", "{name}");
    assert_eq!(run.engine, Engine::Hadolint, "{name}");

    match case.expected {
        Expected::Hit { line, message } => {
            assert_eq!(run.no_files, None, "{name}: {run:?}");
            assert_eq!(run.not_available_cause, None, "{name}: {run:?}");
            let RuleOutcome::Findings(findings) = &run.outcome else { panic!("{name}: {run:?}") };
            assert_eq!(findings.len(), 1, "{name}: {findings:?}");
            let hit = &findings[0];
            assert_eq!(hit.rule, "docker");
            assert_eq!(hit.engine, Engine::Hadolint);
            assert_eq!(hit.path, RelPath::new(case.file));
            assert_eq!(hit.fix, FIX);
            if let Some(line) = line { assert_eq!(hit.line(), Some(line), "{name}"); }
            if let Some(message) = message { assert_eq!(hit.what, message, "{name}"); }
        }
        Expected::Clean => {
            assert_eq!(run.no_files, None, "{name}: {run:?}");
            assert_eq!(run.not_available_cause, None, "{name}: {run:?}");
            assert!(run.raw_chars.is_some(), "{name}: {run:?}");
            assert_eq!(run.outcome, RuleOutcome::Findings(vec![]), "{name}");
        }
        Expected::Missing => {
            assert_eq!(run.no_files, None, "{name}: {run:?}");
            assert_eq!(run.not_available_cause, Some(NotAvailableCause::EngineNotOnPath), "{name}: {run:?}");
            let RuleOutcome::NotAvailable { why, .. } = &run.outcome else { panic!("{name}: {run:?}") };
            assert!(why.contains("hadolint") && why.contains("PATH"), "{name}: {why}");
        }
        Expected::NoFiles => {
            assert_eq!(run.no_files.as_deref(), Some("no file its engine checks"), "{name}: {run:?}");
            assert_eq!(run.not_available_cause, None, "{name}: {run:?}");
            assert_eq!(run.outcome, RuleOutcome::Findings(vec![]), "{name}");
            assert_eq!(no_file_rules(&file, &repo.0), vec![NoFileRule {
                rule: "docker".into(), engine: Engine::Hadolint, why: "no file its engine checks".into(),
            }]);
        }
        Expected::Failed(token) => {
            assert_eq!(run.not_available_cause, None, "{name}: {run:?}");
            let RuleOutcome::Failed { stderr, .. } = &run.outcome else { panic!("{name}: {run:?}") };
            if let Some(token) = token { assert!(stderr.contains(token), "{name}: {stderr}"); }
        }
        Expected::Refused => {
            assert_eq!(run.not_available_cause, None, "{name}: {run:?}");
            assert_eq!(run.raw_chars, None, "{name}: {run:?}");
            assert!(matches!(&run.outcome, RuleOutcome::Failed { .. }), "{name}: {run:?}");
        }
        Expected::NotListed { shellcheck } => {
            assert_eq!(run.not_available_cause, Some(NotAvailableCause::Other), "{name}: {run:?}");
            let RuleOutcome::NotAvailable { why, .. } = &run.outcome else { panic!("{name}: {run:?}") };
            assert!(why.contains(case.check), "{name}: {why}");
            assert!(why.to_lowercase().contains("list"), "{name}: {why}");
            if shellcheck {
                assert!(why.to_lowercase().contains("shellcheck") && why.to_lowercase().contains("optional"), "{name}: {why}");
            }
        }
    }
}

macro_rules! cases {
    ($($name:ident => $case:expr),+ $(,)?) => {
        $(#[test] fn $name() { run_case(stringify!($name), $case); })+
    };
}

const HIT: Expected = Expected::Hit { line: Some(1), message: Some(MESSAGE) };

macro_rules! case {
    ($file:expr, $source:expr, $check:expr, $config:expr, $extra:expr, $expected:expr) => {
        Case { file: $file, source: $source, check: $check, config: $config, extra: $extra, env: None, expected: $expected }
    };
    ($file:expr, $source:expr, $check:expr, $config:expr, $extra:expr, $expected:expr, $env:expr) => {
        Case { env: Some($env), ..case!($file, $source, $check, $config, $extra, $expected) }
    };
}

#[test]
fn old_hadolint_no_file_before_floor() {
    in_own_process("old_hadolint_no_file_before_floor", || old_hadolint_row("no-file", "README", false));
}

#[test]
fn old_hadolint_matching_file_before_floor() {
    in_own_process("old_hadolint_matching_file_before_floor", || old_hadolint_row("matching-file", "Dockerfile", true));
}

/// One row with hadolint 2.14.0, below the floor, with PATH a folder holding only git and the hadolint stub; `file` is the one source file the row adds.
fn old_hadolint_row(name: &str, file: &str, old_floor: bool) {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let repo = Repo::new(name);
    let _path = PathGuard::git_only(&repo);
    repo.write("bin/hadolint", "#!/bin/sh\necho 'Haskell Dockerfile Linter 2.14.0'\n");
    fs::set_permissions(repo.0.join("bin/hadolint"), fs::Permissions::from_mode(0o755)).unwrap();
    repo.write(file, "FROM ubuntu\n");
    repo.write("tao.toml", "version = 1\n[[rule]]\nid = \"docker\"\nengine = \"hadolint\"\ncheck = \"DL3006\"\npaths = [\"**\"]\nfix = \"Tag the image.\"\n");
    let rules = RuleFile::read(&repo.0.join("tao.toml")).unwrap();
    let checked = run_rules_versioned_at(&rules, &repo.0, &[], None);
    assert_eq!(checked.engines.len(), 1, "{name}: {:?}", checked.engines);
    assert_eq!(checked.engines[0].engine, Engine::Hadolint, "{name}");
    assert_eq!(checked.engines[0].version.as_deref(), Some("2.14.0"), "{name}");
    assert_eq!(checked.runs.len(), 1, "{name}: {:?}", checked.runs);
    let run = &checked.runs[0];
    if old_floor {
        assert_eq!(run.no_files, None, "{name}: {run:?}");
        assert_eq!(run.not_available_cause, Some(NotAvailableCause::Other), "{name}: {run:?}");
        let RuleOutcome::NotAvailable { why, .. } = &run.outcome else { panic!("{name}: {run:?}") };
        assert_eq!(why, "hadolint 2.14.0 is older than hadolint 2.15.1, the release cctao's list of hadolint codes was taken from", "{name}");
    } else {
        assert_eq!(run.outcome, RuleOutcome::Findings(vec![]), "{name}: {run:?}");
        assert_eq!(run.no_files.as_deref(), Some("no file its engine checks"), "{name}");
        assert_eq!(run.not_available_cause, None, "{name}");
    }
}

cases! {
    dockerfile => case!("Dockerfile", VIOLATION, "DL3006", None, None, HIT),
    environment_ignore => case!("Dockerfile", VIOLATION, "DL3006", None, None, HIT, ("HADOLINT_IGNORE", "DL3006")),
    containerfile => case!("Containerfile", VIOLATION, "DL3006", None, None, HIT),
    named_dockerfile => case!("web.Dockerfile", VIOLATION, "DL3006", None, None, HIT),
    space_path => case!("sub dir/Dockerfile", VIOLATION, "DL3006", None, None, HIT),
    unicode_path => case!("é.Dockerfile", VIOLATION, "DL3006", None, None, HIT),
    clean => case!("Dockerfile", "FROM ubuntu:22.04\n", "DL3006", None, None, Expected::Clean),
    missing_engine => case!("Dockerfile", VIOLATION, "DL3006", None, None, Expected::Missing),
    no_files_before_missing_engine => case!("README", "readme\n", "DL3006", None, None, Expected::NoFiles),
    inline_ignore => case!("Dockerfile", "# hadolint ignore=DL3006\nFROM ubuntu\n", "DL3006", None, None, Expected::Hit { line: Some(2), message: Some(MESSAGE) }),
    global_ignore => case!("Dockerfile", "# hadolint global ignore=DL3006\nFROM ubuntu\n", "DL3006", None, None, Expected::Hit { line: Some(2), message: Some(MESSAGE) }),
    yaml_ignored => case!("Dockerfile", VIOLATION, "DL3006", Some((".hadolint.yaml", "ignored: [DL3006]\n")), None, Expected::Failed(Some(".hadolint.yaml"))),
    yml_ignored => case!("Dockerfile", VIOLATION, "DL3006", Some((".hadolint.yml", "ignored:\n  - DL3006\n")), None, Expected::Failed(Some(".hadolint.yml"))),
    config_override => case!("Dockerfile", VIOLATION, "DL3006", Some((".hadolint.yaml", "override:\n  error: [DL3006]\n")), None, HIT),
    engine_rejects_config => case!("Dockerfile", VIOLATION, "DL3006", Some((".hadolint.yaml", "strict-labels: yes\n")), None, Expected::Failed(Some("Error parsing your config file"))),
    invalid_dockerfile => case!("Dockerfile", VIOLATION, "DL3006", None, Some(("bad.Dockerfile", "FROM ubuntu\nFROOM x\n")), Expected::Failed(Some("could not parse bad.Dockerfile"))),
    shellcheck_code => case!("Dockerfile", VIOLATION, "SC2086", None, None, Expected::NotListed { shellcheck: true }),
    unknown_dl_code => case!("Dockerfile", VIOLATION, "DL9999", None, None, Expected::NotListed { shellcheck: false }),
    malformed_check => case!("Dockerfile", VIOLATION, "DLx", None, None, Expected::Refused),
    multiple_checks => case!("Dockerfile", VIOLATION, "DL3006,DL3008", None, None, Expected::Refused),
    default_off_rule => case!("Dockerfile", "FROM ubuntu:22.04\n# hadolint ignore=DL3008\nRUN apt-get update\n", "DL1001", None, None, Expected::Hit { line: None, message: None }),
}
