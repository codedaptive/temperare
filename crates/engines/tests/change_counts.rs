use std::fs;
use std::path::{Path, PathBuf};
use std::ffi::OsString;
use std::sync::atomic::{AtomicUsize, Ordering};

use engines::{availability, run_rules_versioned_at, Availability};
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

static NEXT_FOLDER: AtomicUsize = AtomicUsize::new(0);

struct TempFolder(PathBuf);

impl TempFolder {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "engines-change-counts-{}-{}",
            std::process::id(),
            NEXT_FOLDER.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(fs::canonicalize(path).unwrap())
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn git(repo: &Path, args: &[&str]) -> String {
    let output = procio::run(&procio::git_cmd(repo, args.iter().map(OsString::from).collect())).unwrap();
    assert_eq!(output.code, Some(0), "git {args:?}: {}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

enum Expected {
    Finding,
    Clean,
    NoBase,
    Failed(&'static str),
}

fn check_change(code_lines: usize, tracked_test_lines: usize, untracked_test_lines: usize, params: &str, with_base: bool, expected: Expected, setup: impl FnOnce(&Path) -> Option<(PathBuf, fs::Permissions)>) {
    let repo = TempFolder::new();
    git(&repo.0, &["init", "-q"]);
    fs::create_dir(repo.0.join("src")).unwrap();
    fs::create_dir(repo.0.join("tests")).unwrap();
    fs::write(repo.0.join("src/lib.rs"), "base code\n").unwrap();
    fs::write(repo.0.join("tests/a_tests.rs"), "base test\n").unwrap();
    git(&repo.0, &["add", "src/lib.rs", "tests/a_tests.rs"]);
    git(&repo.0, &["-c", "user.name=Test", "-c", "user.email=test@example.invalid", "commit", "-q", "-m", "base"]);
    let base = git(&repo.0, &["rev-parse", "HEAD"]);
    fs::write(repo.0.join("src/lib.rs"), format!("base code\n{}", "added code\n".repeat(code_lines))).unwrap();
    fs::write(repo.0.join("tests/a_tests.rs"), format!("base test\n{}", "added test\n".repeat(tracked_test_lines))).unwrap();
    if untracked_test_lines > 0 {
        fs::write(repo.0.join("tests/new_tests.rs"), "new test\n".repeat(untracked_test_lines)).unwrap();
    }
    fs::write(repo.0.join("temperare.toml"), format!(r#"version = 1
owner = "team"
[kits.project]
paths = ["src/**", "tests/**"]
owner = "team"
[[rule]]
id = "test-ratio"
engine = "git"
check = "test_ratio"
paths = ["src/**", "tests/**"]
fix = "Balance the change."
[rule.params]
{params}
"#)).unwrap();
    let file = RuleFile::read(&repo.0.join("temperare.toml")).unwrap();
    let restore = setup(&repo.0);
    let mut runs = run_rules_versioned_at(&file, &repo.0, &[], if with_base { Some(&base) } else { None }).runs;
    if let Some((path, permissions)) = restore {
        fs::set_permissions(path, permissions).unwrap();
    }
    assert_eq!(runs.len(), 1, "{runs:?}");
    let run = runs.remove(0);
    assert_eq!(run.rule, "test-ratio");
    assert_eq!(run.engine, Engine::Git);
    assert_eq!(run.raw_chars, None);
    match expected {
        Expected::Finding => {
            let RuleOutcome::Findings(findings) = run.outcome else { panic!("expected findings, got {:?}", run.outcome) };
            assert_eq!(findings.len(), 1, "{findings:?}");
            assert_eq!(findings[0].path, RelPath::new("tests/new_tests.rs"));
            assert_eq!(findings[0].line(), Some(1));
            assert_eq!(findings[0].what, "this change adds 80 lines of test for 10 lines of code (8.0 to 1; the rule's limit is 1.5 past 40 lines)");
        }
        Expected::Clean => assert_eq!(run.outcome, RuleOutcome::Findings(vec![])),
        Expected::NoBase => assert_eq!(run.outcome, RuleOutcome::NotAvailable {
            rule: "test-ratio".into(), engine: Engine::Git, why: "reports only what a change adds; run with --base".into(),
        }),
        Expected::Failed(name) => {
            let RuleOutcome::Failed { rule, engine, stderr, .. } = run.outcome else { panic!("expected Failed, got {:?}", run.outcome) };
            assert_eq!(rule, "test-ratio");
            assert_eq!(engine, Engine::Git);
            assert!(stderr.contains(name), "{stderr}");
        }
    }
}

const PARAMS: &str = "max_ratio = 1.5\nfloor = 40\ntest_paths = [\"tests/**\"]";

macro_rules! change_case {
    ($name:ident, $code:expr, $tracked:expr, $untracked:expr, $params:expr, $base:expr, $expected:expr) => {
        #[test]
        fn $name() {
            assert_eq!(availability(Engine::Git), Availability::Available);
            check_change($code, $tracked, $untracked, $params, $base, $expected, |_| None);
        }
    };
}

change_case!(over_ratio_counts_untracked, 10, 30, 50, PARAMS, true, Expected::Finding);
change_case!(under_floor_is_clean, 2, 20, 0, PARAMS, true, Expected::Clean);
change_case!(without_base_is_not_available, 10, 30, 50, PARAMS, false, Expected::NoBase);
change_case!(zero_ratio_fails, 10, 30, 50, "max_ratio = 0\nfloor = 40\ntest_paths = [\"tests/**\"]", true, Expected::Failed("max_ratio"));
change_case!(unknown_param_fails, 10, 30, 50, "max_ratio = 1.5\nfloor = 40\ntest_paths = [\"tests/**\"]\nfoo = 1", true, Expected::Failed("foo"));

#[cfg(unix)]
#[test]
fn dangling_untracked_symlink_is_skipped() {
    check_change(10, 30, 50, PARAMS, true, Expected::Finding, |repo| {
        std::os::unix::fs::symlink("nowhere", repo.join("tests/gone.rs")).unwrap();
        None
    });
}

#[cfg(unix)]
#[test]
fn unreadable_untracked_file_outside_scope_is_skipped() {
    use std::os::unix::fs::PermissionsExt;

    check_change(2, 20, 0, PARAMS, true, Expected::Clean, |repo| {
        fs::create_dir(repo.join("other")).unwrap();
        let path = repo.join("other/secret.txt");
        fs::write(&path, "secret\n").unwrap();
        let original = fs::metadata(&path).unwrap().permissions();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o000)).unwrap();
        Some((path, original))
    });
}
