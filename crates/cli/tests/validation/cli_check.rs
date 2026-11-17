//! Validation (S-28): check runs in an isolated repository with engine programs stood in by stubs.

use procio::{Cmd, Output};
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::ErrorKind;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_FOLDER: AtomicU64 = AtomicU64::new(0);

const QUALITY: &str = r#"version = 1
[[rule]]
id = "python-lint"
engine = "ruff"
check = "F401"
fix = "Remove the unused import."
"#;
const SECURITY: &str = r#"version = 1
owner = "security-team"
[[rule]]
id = "python-lint"
engine = "ruff"
kind = "security"
check = "F401"
fix = "Remove the unused import."
"#;
const WARNING: &str = r#"version = 1
[[rule]]
id = "python-lint"
engine = "ruff"
check = "F401"
severity = "warning"
fix = "Remove the unused import."
"#;
const SWIFT: &str = r#"version = 1
[[rule]]
id = "swift-lint"
engine = "swiftlint"
check = "line_length"
fix = "Fix Swift lint."
"#;

struct Repo {
    dir: PathBuf,
    git: PathBuf,
}

impl Repo {
    fn new(rules: &str) -> Self {
        let parent = fs::canonicalize(std::env::temp_dir()).unwrap();
        let dir = loop {
            let n = NEXT_FOLDER.fetch_add(1, Ordering::Relaxed);
            let path = parent.join(format!("tm-check-zit-{}-{n}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => break path,
                Err(error) if error.kind() == ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("create test repository: {error}"),
            }
        };
        let exec_path = run("git".as_ref(), &["--exec-path"], &dir, &std::env::var_os("PATH").unwrap(), true);
        assert_eq!(exec_path.code, Some(0), "git --exec-path failed");
        let git = PathBuf::from(String::from_utf8(exec_path.stdout).unwrap().trim()).join("git");
        let repo = Self { dir, git };
        fs::create_dir(repo.dir.join("stubs")).unwrap();
        symlink(&repo.git, repo.dir.join("stubs/git")).unwrap();
        repo.git(&["init", "-q"]);
        fs::write(repo.dir.join("temperare.toml"), rules).unwrap();
        fs::write(repo.dir.join("a.py"), "import os\n").unwrap();
        repo
    }

    fn git(&self, args: &[&str]) {
        let output = run(self.git.as_os_str(), args, &self.dir, &std::env::var_os("PATH").unwrap(), true);
        assert_eq!(output.code, Some(0), "git {args:?}: {}", String::from_utf8_lossy(&output.stderr));
    }

    fn stub(&self, body: &str) {
        let path = self.dir.join("stubs/ruff");
        fs::write(&path, format!("#!/bin/sh\n{body}\n")).unwrap();
        fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
    }

    fn diagnostic(&self, row: u32) {
        self.stub(&format!(
            "printf '[{{\"code\":\"F401\",\"filename\":\"%s/a.py\",\"location\":{{\"row\":{row},\"column\":8}},\"message\":\"unused import\"}}]\\n' \"$(pwd -P)\"\nexit 1"
        ));
    }

    fn check(&self, args: &[&str]) -> Output {
        let mut check_args = vec!["check"];
        check_args.extend_from_slice(args);
        run(env!("CARGO_BIN_EXE_tm").as_ref(), &check_args, &self.dir, self.dir.join("stubs").as_os_str(), false)
    }
}

impl Drop for Repo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

fn run(program: &OsStr, args: &[&str], dir: &Path, path: &OsStr, git_setup: bool) -> Output {
    let mut env: Vec<(OsString, OsString)> = vec![
        ("PATH".into(), path.into()),
        ("GIT_CONFIG_NOSYSTEM".into(), "1".into()),
        ("GIT_CONFIG_GLOBAL".into(), "/dev/null".into()),
        ("GIT_CEILING_DIRECTORIES".into(), dir.parent().unwrap().as_os_str().into()),
    ];
    if git_setup {
        env.extend([
            ("GIT_AUTHOR_NAME".into(), "Test Author".into()),
            ("GIT_AUTHOR_EMAIL".into(), "author@example.invalid".into()),
            ("GIT_COMMITTER_NAME".into(), "Test Committer".into()),
            ("GIT_COMMITTER_EMAIL".into(), "committer@example.invalid".into()),
        ]);
    }
    procio::run(&Cmd {
        args: args.iter().map(OsString::from).collect(),
        cwd: Some(dir.to_path_buf()),
        env,
        env_remove: std::env::vars_os().map(|(name, _)| name).collect(),
        ..Cmd::new(program)
    }).unwrap()
}

fn text(output: &Output, code: i32) -> String {
    assert_eq!(output.code, Some(code), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(output.stderr.is_empty(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8(output.stdout.clone()).unwrap()
}

#[test]
fn clean_check_exits_zero() {
    let repo = Repo::new(QUALITY);
    repo.stub("printf '[]\\n'\nexit 0");
    let stdout = text(&repo.check(&[]), 0);
    assert_eq!(stdout.lines().last(), Some("next: nothing to do"));
    assert!(stdout.contains("0 findings"), "{stdout}");
}

#[test]
fn quality_error_names_rule_place_fix_and_exception() {
    let repo = Repo::new(QUALITY);
    repo.diagnostic(1);
    let stdout = text(&repo.check(&[]), 1);
    assert!(stdout.contains("a.py:1:8  error quality python-lint"), "{stdout}");
    assert!(stdout.contains("  fix: Remove the unused import."), "{stdout}");
    assert_eq!(stdout.lines().last(), Some(format!("next: fix the findings above, or add an approved [[exception]] to {}", repo.dir.join("temperare.toml").display()).as_str()));
}

#[test]
fn failing_engine_exits_five() {
    let repo = Repo::new(QUALITY);
    repo.stub("printf 'ruff failed\\n' >&2\nexit 2");
    let stdout = text(&repo.check(&[]), 5);
    assert!(stdout.contains("failed: python-lint (ruff), exit 2"), "{stdout}");
    assert_eq!(stdout.lines().last(), Some("next: see the engine output above"));
}

#[test]
fn absent_engine_exits_three_and_names_install() {
    let repo = Repo::new(SWIFT);
    let stdout = text(&repo.check(&[]), 3);
    assert!(stdout.contains("swiftlint not found on PATH"), "{stdout}");
    assert_eq!(stdout.lines().last(), Some("next: install swiftlint (or run without those rules)"));
}

#[test]
fn security_error_alone_exits_four() {
    let repo = Repo::new(SECURITY);
    repo.diagnostic(1);
    let stdout = text(&repo.check(&[]), 4);
    assert!(stdout.contains("a.py:1:8  error security python-lint  owner security-team"), "{stdout}");
    assert_eq!(stdout.lines().last(), Some("next: raise the security findings with their owner; work goes on"));
}

#[test]
fn warnings_only_exit_zero() {
    let repo = Repo::new(WARNING);
    repo.diagnostic(1);
    let stdout = text(&repo.check(&[]), 0);
    assert!(stdout.contains("a.py:1:8  warning quality python-lint"), "{stdout}");
    assert_eq!(stdout.lines().last(), Some("next: nothing to do"));
}

#[test]
fn base_head_discards_finding_on_unchanged_line() {
    let repo = Repo::new(QUALITY);
    repo.git(&["add", "a.py"]);
    repo.git(&["commit", "-qm", "base"]);
    repo.diagnostic(1);
    let stdout = text(&repo.check(&["--base", "HEAD"]), 0);
    assert!(!stdout.contains("a.py:1:8"), "{stdout}");
    assert!(stdout.contains("0 findings"), "{stdout}");
    assert_eq!(stdout.lines().last(), Some("next: nothing to do"));
}

#[test]
fn base_head_keeps_finding_on_added_line() {
    let repo = Repo::new(QUALITY);
    repo.git(&["add", "a.py"]);
    repo.git(&["commit", "-qm", "base"]);
    fs::write(repo.dir.join("a.py"), "import os\nimport sys\n").unwrap();
    repo.diagnostic(2);
    let stdout = text(&repo.check(&["--base", "HEAD"]), 1);
    assert!(stdout.contains("a.py:2:8  error quality python-lint"), "{stdout}");
    assert_eq!(stdout.lines().last(), Some(format!("next: fix the findings above, or add an approved [[exception]] to {}", repo.dir.join("temperare.toml").display()).as_str()));
}

#[test]
fn json_is_one_object_with_report_exit_and_next() {
    let repo = Repo::new(QUALITY);
    repo.diagnostic(1);
    let output = repo.check(&["--json"]);
    let stdout = text(&output, 1);
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(value.is_object());
    assert_eq!(value["exit"], 1);
    assert_eq!(value["next"], format!("fix the findings above, or add an approved [[exception]] to {}", repo.dir.join("temperare.toml").display()));
    assert_eq!(value["findings"].as_array().unwrap().len(), 1);
    assert_eq!(value["findings"][0]["rule"], "python-lint");
    assert_eq!(value["counts"]["findings"], 1);
}

#[test]
fn bad_base_reports_usage_error() {
    let repo = Repo::new(QUALITY);
    let output = repo.check(&["--base", "nosuchrev"]);
    assert_eq!(output.code, Some(2));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.lines().any(|line| line == "tm: base 'nosuchrev' does not name a commit"), "{stderr}");
}
