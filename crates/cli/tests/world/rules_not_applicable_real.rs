//! The built binary lists set-aside and no-file baseline rules in a throwaway git repository.

use serde_json::{json, Value};
use std::ffi::{OsStr, OsString};
use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct TempFolder(PathBuf);

impl TempFolder {
    fn new() -> Self {
        loop {
            let n = NEXT.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!("tm-rules-na-{}-{n}", std::process::id()));
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

fn clean_command(program: impl Into<OsString>, repo: &Path, home: &Path, temp: &Path, path: &OsStr) -> procio::Cmd {
    procio::Cmd {
        cwd: Some(repo.to_path_buf()),
        env_remove: std::env::vars_os().map(|(name, _)| name).collect(),
        env: vec![
            ("PATH".into(), path.into()),
            ("HOME".into(), home.as_os_str().into()),
            ("TMPDIR".into(), temp.as_os_str().into()),
            ("GIT_CONFIG_NOSYSTEM".into(), "1".into()),
        ],
        ..procio::Cmd::new(program)
    }
}

fn git(repo: &Path, home: &Path, temp: &Path, args: &[&str]) -> String {
    let path = std::env::var_os("PATH").unwrap_or_else(|| "/usr/bin:/bin".into());
    let mut cmd = clean_command("git", repo, home, temp, &path);
    cmd.args = args.iter().map(OsString::from).collect();
    let output = procio::run(&cmd).unwrap();
    assert_eq!(output.code, Some(0), "git {args:?}: {}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

/// ZIT: the binary reaches cli's run_rules, rules_world_real, rules_json and rules_text, and run_check.
#[test]
fn rules_not_applicable_matches_check_and_lists_literal_reasons() {
    let temp = TempFolder::new();
    let repo = temp.0.join("repo");
    let home = temp.0.join("home");
    fs::create_dir(&repo).unwrap();
    fs::create_dir(&home).unwrap();
    git(&repo, &home, &temp.0, &["init", "-q"]);
    fs::write(repo.join("temperare.toml"), r#"version = 1
owner = "team"
baseline = ["standard@1", "house@1"]
[[rule]]
id = "own-no-file"
engine = "ruff"
check = "F401"
paths = ["missing/**/*.py"]
fix = "Remove the import."
"#).unwrap();
    git(&repo, &home, &temp.0, &["add", "-A"]);
    git(&repo, &home, &temp.0, &["-c", "user.name=Test", "-c", "user.email=test@example.invalid", "-c", "commit.gpgsign=false", "commit", "-q", "-m", "base"]);
    let base = git(&repo, &home, &temp.0, &["rev-parse", "HEAD"]);

    let path = std::env::var_os("PATH").expect("PATH must contain an executable git");
    let git = std::env::split_paths(&path)
        .map(|dir| dir.join("git"))
        .find(|candidate| fs::metadata(candidate).is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0))
        .expect("PATH must contain an executable git");
    let bin = temp.0.join("bin");
    fs::create_dir(&bin).unwrap();
    // Only git is on PATH so no engine is found, probed, or run.
    symlink(fs::canonicalize(git).unwrap(), bin.join("git")).unwrap();

    let run = |args: &[&str]| {
        let mut cmd = clean_command(env!("CARGO_BIN_EXE_temperare"), &repo, &home, &temp.0, bin.as_os_str());
        cmd.args = args.iter().map(OsString::from).collect();
        procio::run(&cmd).unwrap()
    };
    let listing = run(&["rules", "--json"]);
    assert_eq!(listing.code, Some(0), "{}", String::from_utf8_lossy(&listing.stderr));
    let listing: Value = serde_json::from_slice(&listing.stdout).unwrap();
    let check = run(&["check", "--json", "--base", &base]);
    assert_eq!(check.code, Some(6), "{}", String::from_utf8_lossy(&check.stderr));
    let check: Value = serde_json::from_slice(&check.stdout).unwrap();
    assert_eq!(listing["not_applicable"], check["not_applicable"]);
    let entries = listing["not_applicable"].as_array().unwrap();
    assert_eq!(listing["rules_not_applicable"], json!(entries.len()));

    let no_file = "its paths match no file in the rule file's folder";
    let expected = [
        json!({"rule": "tm-rust-dead-code", "engine": "clippy", "why": no_file}),
        json!({"rule": "tm-change-adds-sleep-rust", "engine": "ast-grep", "why": no_file}),
        json!({"rule": "tm-house-no-stored-bool-swift", "engine": "ast-grep", "why": "required params not given: entity_types; give them in the params of an [[override]] for \"tm-house-no-stored-bool-swift\""}),
    ];
    let positions: Vec<usize> = expected.iter().map(|entry| {
        entries.iter().position(|actual| actual == entry).unwrap_or_else(|| panic!("missing {entry}: {entries:?}"))
    }).collect();
    assert!(positions.windows(2).all(|pair| pair[0] < pair[1]), "{positions:?}");
    assert!(entries.iter().all(|entry| entry["rule"] != "own-no-file"));

    let text = run(&["rules"]);
    assert_eq!(text.code, Some(0), "{}", String::from_utf8_lossy(&text.stderr));
    let text = String::from_utf8(text.stdout).unwrap();
    assert!(text.lines().any(|line| line == format!("not applicable ({}):", entries.len())), "{text}");
    assert!(text.lines().any(|line| line == format!("  tm-rust-dead-code  clippy  {no_file}")), "{text}");
}
