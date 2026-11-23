//! Cross-crate integration test: the built binary checks a git test-ratio rule with and without a base in a throwaway repository.

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct TempFolder(PathBuf);

impl TempFolder {
    fn new() -> Self {
        loop {
            let n = NEXT.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!("tm-check-base-{}-{n}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => return Self(fs::canonicalize(path).unwrap()),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => panic!("cannot make {}: {e}", path.display()),
            }
        }
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn clean_command(program: impl Into<OsString>, repo: &Path, home: &Path, temp: &Path) -> procio::Cmd {
    procio::Cmd {
        cwd: Some(repo.to_path_buf()),
        env_remove: std::env::vars_os().map(|(name, _)| name).collect(),
        env: vec![
            ("PATH".into(), std::env::var_os("PATH").unwrap_or_else(|| "/usr/bin:/bin".into())),
            ("HOME".into(), home.as_os_str().into()),
            ("TMPDIR".into(), temp.as_os_str().into()),
            ("GIT_CONFIG_NOSYSTEM".into(), "1".into()),
        ],
        ..procio::Cmd::new(program)
    }
}

fn git(repo: &Path, home: &Path, temp: &Path, args: &[&str]) -> String {
    let mut cmd = clean_command("git", repo, home, temp);
    cmd.args = args.iter().map(OsString::from).collect();
    let output = procio::run(&cmd).unwrap();
    assert_eq!(output.code, Some(0), "git {args:?}: {}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

fn check_base(with_base: bool) {
    let temp = TempFolder::new();
    let repo = temp.0.join("repo");
    let home = temp.0.join("home");
    fs::create_dir(&repo).unwrap();
    fs::create_dir(&home).unwrap();
    fs::create_dir(repo.join("src")).unwrap();
    fs::create_dir(repo.join("tests")).unwrap();
    git(&repo, &home, &temp.0, &["init", "-q"]);
    fs::write(repo.join("src/lib.rs"), "base code\n").unwrap();
    fs::write(repo.join("tests/a_tests.rs"), "base test\n").unwrap();
    fs::write(repo.join("temperare.toml"), r#"version = 1
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
only_with_base = true
[rule.params]
max_ratio = 1.5
floor = 1
test_paths = ["tests/**"]
"#).unwrap();
    git(&repo, &home, &temp.0, &["add", "-A"]);
    git(&repo, &home, &temp.0, &["-c", "user.name=Test", "-c", "user.email=test@example.invalid", "-c", "commit.gpgsign=false", "commit", "-q", "-m", "base"]);
    let base = git(&repo, &home, &temp.0, &["rev-parse", "HEAD"]);
    fs::write(repo.join("src/lib.rs"), "base code\nadded code\n").unwrap();
    fs::write(repo.join("tests/a_tests.rs"), "base test\none\ntwo\nthree\nfour\nfive\nsix\n").unwrap();

    let mut cmd = clean_command(env!("CARGO_BIN_EXE_temperare"), &repo, &home, &temp.0);
    cmd.args = if with_base { vec!["check".into(), "--base".into(), base.into()] } else { vec!["check".into()] };
    let output = procio::run(&cmd).unwrap();
    let text = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    if with_base {
        assert!(text.lines().any(|line| line.starts_with("tests/a_tests.rs:2  ") && line.split_whitespace().any(|word| word == "test-ratio")), "stdout:\n{text}\nstderr:\n{stderr}");
        assert!(text.lines().any(|line| line.starts_with("1 findings ")), "stdout:\n{text}\nstderr:\n{stderr}");
        assert!(!text.contains("not available: test-ratio"), "stdout:\n{text}");
        assert!(!text.contains("not applicable: test-ratio"), "stdout:\n{text}");
        assert_eq!(output.code, Some(1), "stdout:\n{text}\nstderr:\n{stderr}");
    } else {
        assert!(text.lines().any(|line| line.starts_with("0 findings (") && line.contains(", 1 not applicable, ")), "stdout:\n{text}\nstderr:\n{stderr}");
        assert!(!text.lines().any(|line| line.contains(" quality test-ratio") || line.contains(" security test-ratio")), "stdout:\n{text}");
        assert_eq!(output.code, Some(0), "stdout:\n{text}\nstderr:\n{stderr}");
    }
}

macro_rules! case {
    ($name:ident, $with_base:expr) => {
        #[test]
        fn $name() {
            check_base($with_base);
        }
    };
}

case!(with_base_reports_added_test_ratio, true);
case!(without_base_reports_not_applicable, false);
