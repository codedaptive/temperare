//! Validation: check runs in isolated repositories, with stub engines except in the real_ruff_ cases.

use crate::temp_home::TempHome;
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
    home: TempHome,
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
        let exec_path = run("git".as_ref(), &["--exec-path"], &dir, &std::env::var_os("PATH").unwrap(), None, true);
        assert_eq!(exec_path.code, Some(0), "git --exec-path failed");
        let git = PathBuf::from(String::from_utf8(exec_path.stdout).unwrap().trim()).join("git");
        let repo = Self { dir, git, home: TempHome::new() };
        fs::create_dir(repo.dir.join("stubs")).unwrap();
        symlink(&repo.git, repo.dir.join("stubs/git")).unwrap();
        repo.git(&["init", "-q"]);
        fs::write(repo.dir.join("temperare.toml"), rules).unwrap();
        fs::write(repo.dir.join("a.py"), "import os\n").unwrap();
        repo
    }

    fn git(&self, args: &[&str]) {
        let output = run(self.git.as_os_str(), args, &self.dir, &std::env::var_os("PATH").unwrap(), None, true);
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

    /// Runs the built temperare with PATH = the stubs folder and HOME = the test's temp home.
    fn check(&self, args: &[&str]) -> Output {
        let mut check_args = vec!["check"];
        check_args.extend_from_slice(args);
        run(env!("CARGO_BIN_EXE_temperare").as_ref(), &check_args, &self.dir, self.dir.join("stubs").as_os_str(), Some(self.home.path()), false)
    }
}

impl Drop for Repo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

/// Runs `program` with only the variables set here; HOME is set when `home` is given.
fn run(program: &OsStr, args: &[&str], dir: &Path, path: &OsStr, home: Option<&Path>, git_setup: bool) -> Output {
    let mut env: Vec<(OsString, OsString)> = vec![
        ("PATH".into(), path.into()),
        ("GIT_CONFIG_NOSYSTEM".into(), "1".into()),
        ("GIT_CONFIG_GLOBAL".into(), "/dev/null".into()),
        ("GIT_CEILING_DIRECTORIES".into(), dir.parent().unwrap().as_os_str().into()),
    ];
    if let Some(home) = home {
        env.push(("HOME".into(), home.as_os_str().into()));
    }
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

// Four 91-character objects, three commas, two brackets and a newline: 370 characters.
const SAVINGS_RUFF: &str = concat!(
    "printf '[",
    "{\"code\":\"E501\",\"filename\":\"a.py\",\"location\":{\"row\":1,\"column\":8},\"message\":\"unused import\"},",
    "{\"code\":\"E501\",\"filename\":\"a.py\",\"location\":{\"row\":1,\"column\":8},\"message\":\"unused import\"},",
    "{\"code\":\"E501\",\"filename\":\"a.py\",\"location\":{\"row\":1,\"column\":8},\"message\":\"unused import\"},",
    "{\"code\":\"E501\",\"filename\":\"a.py\",\"location\":{\"row\":1,\"column\":8},\"message\":\"unused import\"}]\\n'\nexit 0"
);

fn output_tokens(chars: usize) -> usize { (chars + 2) / 4 }

fn percent(saved: usize, baseline: usize) -> usize {
    (0..=100).rev().find(|p| p * baseline <= saved * 100).unwrap()
}

/// The printed JSON line without its `"saved"` member, and that member parsed.
fn printed_body(stdout: &str) -> (String, serde_json::Value) {
    let line = stdout.strip_suffix('\n').unwrap();
    assert!(!line.contains('\n'), "stdout must end with exactly one newline");
    let mut value: serde_json::Value = serde_json::from_str(stdout).unwrap();
    let saved = value.as_object_mut().unwrap().remove("saved").unwrap();
    let needle = format!("\"saved\":{saved}");
    assert_eq!(line.matches(&needle).count(), 1, "the printed saved is not compact, or appears twice");
    let (before, after) = line.split_once(&needle).unwrap();
    let body = if let Some(before) = before.strip_suffix(',') {
        format!("{before}{after}")
    } else {
        format!("{before}{}", after.strip_prefix(',').unwrap())
    };
    assert_eq!(serde_json::from_str::<serde_json::Value>(&body).unwrap(), value);
    (body, saved)
}

fn savings_repo() -> (Repo, PathBuf) {
    let repo = Repo::new(QUALITY);
    repo.stub(SAVINGS_RUFF);
    let home = repo.dir.join("home");
    fs::create_dir(&home).unwrap();
    (repo, home)
}

fn savings_check(repo: &Repo, home: &Path, args: &[&str]) -> Output {
    let mut check_args = vec!["check"];
    check_args.extend_from_slice(args);
    procio::run(&Cmd {
        args: check_args.iter().map(OsString::from).collect(),
        cwd: Some(repo.dir.clone()),
        env: vec![
            ("PATH".into(), repo.dir.join("stubs").into_os_string()),
            ("HOME".into(), home.as_os_str().into()),
            ("GIT_CONFIG_NOSYSTEM".into(), "1".into()),
            ("GIT_CONFIG_GLOBAL".into(), "/dev/null".into()),
            ("GIT_CEILING_DIRECTORIES".into(), repo.dir.parent().unwrap().as_os_str().into()),
        ],
        env_remove: std::env::vars_os().map(|(name, _)| name).collect(),
        ..Cmd::new(env!("CARGO_BIN_EXE_temperare"))
    }).unwrap()
}

#[test]
fn savings_json_counts_raw_ruff_output_exactly() {
    let (repo, home) = savings_repo();
    let stdout = text(&savings_check(&repo, &home, &["--json"]), 0);
    let (body, saved) = printed_body(&stdout);
    let chars = body.chars().count();
    assert_eq!(saved["output_chars"], chars);
    let tokens = output_tokens(chars);
    let tokens_saved = 93usize.saturating_sub(tokens);
    assert_eq!(output_tokens(231), 58);
    assert_eq!(percent(35, 93), 37);
    assert_eq!(saved, serde_json::json!({
        "counted": [{"rule":"python-lint", "engine":"ruff", "chars":370}],
        "engine_runs":1, "baseline_chars":370, "baseline_tokens":93,
        "output_chars":chars, "output_tokens":tokens, "tokens_saved":tokens_saved, "percent":percent(tokens_saved, 93)
    }));
}

#[test]
fn savings_json_counts_unicode_raw_ruff_output_in_characters() {
    let (repo, home) = savings_repo();
    // One 98-character object, two brackets and a newline: 101 characters, 108 UTF-8 bytes.
    let raw = concat!(
        "[{\"code\":\"E501\",\"filename\":\"a.py\",\"location\":{\"row\":1,\"column\":8},",
        "\"message\":\"unused import — é 日本\"}]\n"
    );
    assert_eq!(raw.chars().count(), 101);
    assert_eq!(raw.len(), 108);
    repo.stub(&format!("printf '%s' '{raw}'\nexit 0"));
    let stdout = text(&savings_check(&repo, &home, &["--json"]), 0);
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(value["saved"]["baseline_chars"], 101);
    assert_eq!(value["saved"]["baseline_tokens"], 25);
}

#[test]
fn savings_text_line_and_json_savings_each_follow_their_printed_body() {
    let (repo, home) = savings_repo();
    let json = text(&savings_check(&repo, &home, &["--json"]), 0);
    let (body, saved) = printed_body(&json);
    let chars = body.chars().count();
    assert_eq!(saved["output_chars"], chars);
    let tokens = output_tokens(chars);
    let tokens_saved = 93usize.saturating_sub(tokens);
    let stdout = text(&savings_check(&repo, &home, &[]), 0);
    // Text and JSON have different output sizes; each saving uses its own report size.
    assert_eq!(saved["output_tokens"], tokens);
    assert_eq!(saved["tokens_saved"], tokens_saved);
    assert_eq!(saved["percent"], percent(tokens_saved, 93));
    let (line, text_body) = stdout.split_once('\n').unwrap();
    assert!(text_body.starts_with("engines: ruff (version unknown)\n"), "{stdout}");
    let text_tokens = output_tokens(text_body.chars().count());
    let text_saved = 93usize.saturating_sub(text_tokens);
    assert_eq!(percent(53, 93), 56);
    assert!(line.starts_with(&format!("[temperare] tokens saved ≈ {text_saved} ({}%) — this output ≈ {text_tokens} tok", percent(text_saved, 93))), "{stdout}");
}

#[test]
fn savings_config_disables_the_text_line() {
    let (repo, home) = savings_repo();
    fs::create_dir_all(home.join(".config/temperare")).unwrap();
    fs::write(home.join(".config/temperare/config.toml"), "savings_line = false\n").unwrap();
    let stdout = text(&savings_check(&repo, &home, &[]), 0);
    assert!(stdout.starts_with("engines: ruff (version unknown)\n0 findings"), "{stdout}");
    assert!(!stdout.contains("tokens saved"), "{stdout}");
}

#[test]
fn savings_log_appends_one_line_per_check() {
    let (repo, home) = savings_repo();
    let log = home.join(".local/state/temperare/savings.jsonl");
    assert!(!log.exists());
    text(&savings_check(&repo, &home, &[]), 0);
    assert_eq!(fs::read_to_string(&log).unwrap().lines().count(), 1);
    text(&savings_check(&repo, &home, &["--json"]), 0);
    assert_eq!(fs::read_to_string(&log).unwrap().lines().count(), 2);
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
fn check_runs_with_home_at_the_test_home() {
    let repo = Repo::new(QUALITY);
    repo.stub("printf '[]\\n'\nexit 0");
    text(&repo.check(&[]), 0);
    let log = repo.home.path().join(".local/state/temperare/savings.jsonl");
    assert_eq!(fs::read_to_string(&log).unwrap().lines().count(), 1);
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

/// Follows savings_real_engine's on_tm_path branch for an absent ruff.
#[test]
fn real_ruff_unknown_check_is_not_available_with_engine_version_and_check() {
    let repo = crate::repo::Repo::new();
    let home = TempHome::new();
    repo.write("temperare.toml", r#"version = 1
[[rule]]
id = "unknown-check"
engine = "ruff"
check = "ZZZ999"
fix = "Choose a supported check."
"#);
    repo.write("a.py", "import os\n");
    repo.commit("unknown check");
    let output = crate::run_clean::run_clean(env!("CARGO_BIN_EXE_temperare"), &["check"], repo.path(), crate::env::tm_env(None, home.path()));
    let stdout = String::from_utf8(output.stdout).unwrap();
    if !crate::on_tm_path::on_tm_path("ruff") {
        assert_eq!(output.code, Some(3), "{stdout}");
        assert!(stdout.contains("not available: unknown-check (ruff): ruff not found on PATH"), "{stdout}");
        assert!(!stdout.lines().any(|line| line.starts_with("engines:")), "an absent engine is not listed: {stdout}");
        return;
    }
    assert_eq!(output.code, Some(3), "{stdout}\n{}", String::from_utf8_lossy(&output.stderr));
    let why = stdout.lines().find_map(|line| line.strip_prefix("not available: unknown-check (ruff): ")).expect("not available line");
    // The version is machine-specific, so it is read from the same ruff tm runs for this expected value only;
    // for ruff's `ruff X.Y.Z` output, word nth(1) is the same word engines' `version_word` takes.
    let version_output = crate::run_clean::run_clean("ruff", &["--version"], repo.path(), crate::env::tm_env(None, home.path()));
    assert_eq!(version_output.code, Some(0));
    let version_stdout = String::from_utf8(version_output.stdout).unwrap();
    let version = version_stdout.split_whitespace().nth(1).expect("ruff version");
    // An installed engine stays listed even when it does not know the rule's check.
    assert!(stdout.lines().any(|line| line == format!("engines: ruff {version}")), "{stdout}");
    let said = why.strip_prefix(&format!("ruff {version} does not know the check `ZZZ999`: ")).unwrap_or_else(|| panic!("{why}"));
    assert!(!said.trim().is_empty() && said.contains("ZZZ999"), "{why}");
    assert!(!stdout.contains("failed:"), "{stdout}");
    assert_eq!(stdout.lines().last(), Some(format!("next: ruff: {why} (or run without those rules)").as_str()));
}

/// Follows savings_real_engine's on_tm_path branch for an absent ruff.
#[test]
fn real_ruff_param_it_does_not_take_fails_before_the_check_runs() {
    let repo = crate::repo::Repo::new();
    let home = TempHome::new();
    repo.write("temperare.toml", r#"version = 1
[[rule]]
id = "param-rule"
engine = "ruff"
check = "F401"
params = { foo = ["x"] }
fix = "Remove the unsupported parameter."
"#);
    repo.write("a.py", "import os\n");
    repo.commit("unsupported parameter");
    let output = crate::run_clean::run_clean(env!("CARGO_BIN_EXE_temperare"), &["check"], repo.path(), crate::env::tm_env(None, home.path()));
    let stdout = String::from_utf8(output.stdout).unwrap();
    if !crate::on_tm_path::on_tm_path("ruff") {
        assert_eq!(output.code, Some(3), "{stdout}");
        assert!(stdout.contains("not available: param-rule (ruff): ruff not found on PATH"), "{stdout}");
        return;
    }
    assert_eq!(output.code, Some(5), "{stdout}\n{}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("failed: param-rule (ruff), no exit code"), "{stdout}");
    assert!(stdout.contains("stderr of param-rule (ruff):\n  ruff param foo: a ruff rule reads only the `max_complexity` param"), "{stdout}");
    assert_eq!(stdout.lines().last(), Some("next: see the engine output above"));
    assert!(!stdout.contains("[temperare] tokens saved"), "{stdout}");
    // The version is machine-specific, so it is read from the same ruff tm runs for this expected value only.
    let version_output = crate::run_clean::run_clean("ruff", &["--version"], repo.path(), crate::env::tm_env(None, home.path()));
    assert_eq!(version_output.code, Some(0));
    let version_stdout = String::from_utf8(version_output.stdout).unwrap();
    let version = version_stdout.split_whitespace().nth(1).expect("ruff version");
    // A refused param fails the rule, but ruff was found, so it stays listed.
    assert!(stdout.lines().any(|line| line == format!("engines: ruff {version}")), "{stdout}");
    let json = crate::run_clean::run_clean(env!("CARGO_BIN_EXE_temperare"), &["check", "--json"], repo.path(), crate::env::tm_env(None, home.path()));
    assert_eq!(json.code, Some(5), "{}", String::from_utf8_lossy(&json.stderr));
    let value: serde_json::Value = serde_json::from_slice(&json.stdout).unwrap();
    assert_eq!(value["engines"], serde_json::json!([{ "engine": "ruff", "version": version }]), "{}", String::from_utf8_lossy(&json.stdout));
    assert_eq!(value["saved"]["engine_runs"], 0);
    assert_eq!(value["saved"]["counted"], serde_json::json!([{ "rule": "param-rule", "engine": "ruff", "chars": 0 }]));
    let failed = value["failed"].as_array().unwrap_or_else(|| panic!("{}", String::from_utf8_lossy(&json.stdout)));
    assert_eq!(failed.len(), 1, "{}", String::from_utf8_lossy(&json.stdout));
    assert_eq!(failed[0]["rule"], "param-rule", "{}", String::from_utf8_lossy(&json.stdout));
    assert_eq!(failed[0]["engine"], "ruff", "{}", String::from_utf8_lossy(&json.stdout));
    let tail = failed[0]["stderr_tail"].as_str().unwrap_or_else(|| panic!("{}", String::from_utf8_lossy(&json.stdout)));
    assert!(tail.contains("ruff param foo: a ruff rule reads only the `max_complexity` param"), "{}", String::from_utf8_lossy(&json.stdout));
}
