//! Validation (S-28): tm's pieces work together on a throwaway repository, from discovery via git and
//! RuleFile::read to the rules listing and exit codes; and the listing's engine availability, with stub engines on a
//! PATH set for the child only. Every program starts through procio::run with a cleared environment.

use std::ffi::{OsStr, OsString};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use procio::{Cmd, Output};

const GOOD_RULES: &str = r#"version = 1

[[rule]]
id = "lint-all"
engine = "clippy"
check = "clippy::all"
fix = "Follow clippy."

[[rule]]
id = "lint-pedantic"
engine = "clippy"
check = "clippy::pedantic"
fix = "Follow the pedantic lint."

[[exception]]
rule = "lint-all"
path = "src/legacy.rs"
reason = "Legacy code."
approved = "reviewer 2026-11-17"
"#;

const BAD_RULES: &str = "version = 2\n\ncolour = \"blue\"\n";

const AVAILABILITY_RULES: &str = r#"version = 1

[[rule]]
id = "python-lint"
engine = "ruff"
check = "E501"
fix = "Fix Python lint."

[[rule]]
id = "swift-lint"
engine = "swiftlint"
check = "line_length"
fix = "Fix Swift lint."

[[rule]]
id = "slop"
engine = "tessera"
check = "slop"
fix = "Fix slop."
"#;

struct TempFolder(PathBuf);

impl TempFolder {
    // Tests run in parallel in one process, so the name carries a process-wide counter: no two calls here get
    // the same number. A folder already there (left by an earlier process with the same pid) is never reused:
    // the next number is tried.
    fn new(tag: &str) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let parent = fs::canonicalize(std::env::temp_dir()).unwrap();
        loop {
            let n = NEXT.fetch_add(1, Ordering::Relaxed);
            let path = parent.join(format!("tm-cli-zit-{}-{n}-{tag}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => return Self(path),
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(e) => panic!("create {}: {e}", path.display()),
            }
        }
    }

    fn init_git(&self) {
        let output = procio::run(&clean_cmd("git".as_ref(), &["init", "-q"], &self.0, &self.0, None)).unwrap();
        assert!(output.code == Some(0), "git init: {}", String::from_utf8_lossy(&output.stderr));
    }

    fn write_rules(&self, text: &str) {
        fs::write(self.0.join("temperare.toml"), text).unwrap();
    }

    fn tm(&self, cwd: &Path, args: &[&str]) -> Output {
        procio::run(&clean_cmd(env!("CARGO_BIN_EXE_temperare").as_ref(), args, cwd, &self.0, None)).unwrap()
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

// The child inherits nothing: every parent variable is removed, then only these are set. HOME is the test's temp
// folder and git reads no system or global config, so no parent variable but PATH shapes the run; git does not look
// for a repository in the temp folder's parent or above. PATH is the parent's, so `git` resolves, unless `path`
// replaces it.
fn clean_cmd(program: &OsStr, args: &[&str], cwd: &Path, home: &Path, path: Option<&OsStr>) -> Cmd {
    let path = path.map_or_else(|| std::env::var_os("PATH").unwrap(), OsStr::to_os_string);
    Cmd {
        program: program.to_os_string(),
        args: args.iter().map(OsString::from).collect(),
        cwd: Some(cwd.to_path_buf()),
        env: vec![
            ("PATH".into(), path),
            ("HOME".into(), home.into()),
            ("GIT_CONFIG_NOSYSTEM".into(), "1".into()),
            ("GIT_CONFIG_GLOBAL".into(), "/dev/null".into()),
            ("GIT_CEILING_DIRECTORIES".into(), home.parent().unwrap().into()),
        ],
        env_remove: std::env::vars_os().map(|(name, _)| name).collect(),
        stdin: vec![],
        timeout: None,
    }
}

fn availability_listing(json: bool) -> Output {
    use std::os::unix::fs::PermissionsExt;

    let tmp = TempFolder::new("availability");
    tmp.init_git();
    tmp.write_rules(AVAILABILITY_RULES);
    let stub_dir = tmp.0.join("stubs");
    fs::create_dir(&stub_dir).unwrap();
    let ruff = stub_dir.join("ruff");
    fs::write(&ruff, "#!/bin/sh\nexit 0\n").unwrap();
    fs::set_permissions(&ruff, fs::Permissions::from_mode(0o755)).unwrap();
    let args: &[&str] = if json {
        &["rules", "--rules", "temperare.toml", "--json"]
    } else {
        &["rules", "--rules", "temperare.toml"]
    };
    let cmd = clean_cmd(env!("CARGO_BIN_EXE_temperare").as_ref(), args, &tmp.0, &tmp.0, Some(stub_dir.as_os_str()));
    let output = procio::run(&cmd).unwrap();
    assert_eq!(output.code, Some(0), "{}", String::from_utf8_lossy(&output.stderr));
    output
}

fn availability_text() -> String {
    let stdout = String::from_utf8(availability_listing(false).stdout).unwrap();
    assert_eq!(stdout.lines().last(), Some("next: nothing to do"));
    stdout
}

fn availability_json_rule(id: &str) -> serde_json::Value {
    let output = availability_listing(true);
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["exit"], 0);
    value["rules"].as_array().unwrap().iter().find(|rule| rule["id"] == id).unwrap().clone()
}

#[test]
fn rules_text_marks_executable_ruff_available() {
    let stdout = availability_text();
    assert!(stdout.lines().any(|line| line == "  python-lint  ruff  quality  error  paths: all files  available"), "{stdout}");
}

#[test]
fn rules_text_explains_missing_swiftlint() {
    let stdout = availability_text();
    assert!(stdout.lines().any(|line| line == "  swift-lint  swiftlint  quality  error  paths: all files  not available: swiftlint not found on PATH"), "{stdout}");
}

#[test]
fn rules_text_explains_unavailable_tessera() {
    let stdout = availability_text();
    assert!(stdout.lines().any(|line| line == "  slop  tessera  quality  error  paths: all files  not available: Tessera's slop records are not defined yet"), "{stdout}");
}

#[test]
fn rules_json_marks_executable_ruff_available_with_null_why() {
    let rule = availability_json_rule("python-lint");
    assert_eq!(rule.get("available"), Some(&serde_json::json!(true)));
    assert_eq!(rule.get("why"), Some(&serde_json::Value::Null));
}

#[test]
fn rules_json_explains_missing_swiftlint() {
    let rule = availability_json_rule("swift-lint");
    assert_eq!(rule.get("available"), Some(&serde_json::json!(false)));
    assert_eq!(rule.get("why"), Some(&serde_json::json!("swiftlint not found on PATH")));
}

#[test]
fn rules_lists_rule_ids_exceptions_and_next_step() {
    let tmp = TempFolder::new("text");
    tmp.init_git();
    tmp.write_rules(GOOD_RULES);
    let output = tmp.tm(&tmp.0, &["rules"]);
    assert_eq!(output.code, Some(0));
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("lint-all"), "{stdout}");
    assert!(stdout.contains("lint-pedantic"), "{stdout}");
    assert!(stdout.contains("exceptions in force (1):"), "{stdout}");
    assert_eq!(stdout.lines().last(), Some("next: nothing to do"));
}

#[test]
fn rules_json_is_one_object_with_counts_and_next_step() {
    let tmp = TempFolder::new("json");
    tmp.init_git();
    tmp.write_rules(GOOD_RULES);
    let output = tmp.tm(&tmp.0, &["rules", "--json"]);
    assert_eq!(output.code, Some(0));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(value.is_object());
    assert_eq!(value["exit"], 0);
    assert_eq!(value["next"], "nothing to do");
    assert_eq!(value["rules"].as_array().unwrap().len(), 2);
    assert_eq!(value["exceptions_in_force"], 1);
}

#[test]
fn bad_rule_file_reports_location_and_next_step() {
    let tmp = TempFolder::new("bad");
    tmp.init_git();
    tmp.write_rules(BAD_RULES);
    let output = tmp.tm(&tmp.0, &["rules"]);
    assert_eq!(output.code, Some(2));
    let stdout = String::from_utf8(output.stdout).unwrap();
    let line = stdout.lines().find(|line| line.contains("temperare.toml:1:"))
        .expect("version problem must name line 1");
    let col_and_message = line.split_once("temperare.toml:1:").unwrap().1;
    let (col, message) = col_and_message.split_once("  ").unwrap();
    assert!(col.parse::<usize>().is_ok(), "{line}");
    assert!(!message.is_empty(), "{line}");
    assert!(stdout.lines().last().unwrap().starts_with("next: "), "{stdout}");
}

#[test]
fn rules_discovers_repository_file_from_subfolder() {
    let tmp = TempFolder::new("subfolder");
    tmp.init_git();
    tmp.write_rules(GOOD_RULES);
    let subfolder = tmp.0.join("nested");
    fs::create_dir(&subfolder).unwrap();
    let output = tmp.tm(&subfolder, &["rules"]);
    assert_eq!(output.code, Some(0));
    assert!(String::from_utf8(output.stdout).unwrap().contains("lint-all"));
}

#[test]
fn rules_flag_works_outside_a_repository() {
    let tmp = TempFolder::new("explicit");
    tmp.write_rules(GOOD_RULES);
    let output = tmp.tm(&tmp.0, &["rules", "--rules", "temperare.toml"]);
    assert_eq!(output.code, Some(0));
    assert!(String::from_utf8(output.stdout).unwrap().contains("lint-all"));
}

#[test]
fn rules_outside_repository_reports_error_on_stderr() {
    let tmp = TempFolder::new("outside");
    let output = tmp.tm(&tmp.0, &["rules"]);
    assert_eq!(output.code, Some(2));
    assert!(String::from_utf8(output.stderr).unwrap().contains("not in a git repository"));
    assert!(output.stdout.is_empty());
}

#[test]
fn rules_without_rule_file_reports_error() {
    let tmp = TempFolder::new("missing");
    tmp.init_git();
    let output = tmp.tm(&tmp.0, &["rules"]);
    assert_eq!(output.code, Some(2));
    assert!(String::from_utf8(output.stderr).unwrap().contains("no rule file at"));
}

#[test]
fn version_prints_temperare_workspace_version_and_commit() {
    let tmp = TempFolder::new("version");
    let output = tmp.tm(&tmp.0, &["--version"]);
    assert_eq!(output.code, Some(0));
    let stdout = String::from_utf8(output.stdout).unwrap();
    let commit = env!("TEMPERARE_COMMIT");
    assert_eq!(stdout, format!("temperare 0.1.0 ({commit})\n"));
    if env!("TEMPERARE_CHECKOUT") == "yes" {
        assert_eq!(commit.len(), 12, "{commit}");
        assert!(commit.chars().all(|ch| matches!(ch, '0'..='9' | 'a'..='f')), "{commit}");
    }
}

#[test]
fn help_prints_rules_usage() {
    let tmp = TempFolder::new("help");
    let output = tmp.tm(&tmp.0, &["help"]);
    assert_eq!(output.code, Some(0));
    assert!(String::from_utf8(output.stdout).unwrap().contains("tm rules [--rules <path>] [--json]"));
}

#[test]
fn unknown_command_reports_usage_error() {
    let tmp = TempFolder::new("unknown");
    let output = tmp.tm(&tmp.0, &["frobnicate"]);
    assert_eq!(output.code, Some(2));
    assert!(String::from_utf8(output.stderr).unwrap().contains("unknown command 'frobnicate'"));
}
