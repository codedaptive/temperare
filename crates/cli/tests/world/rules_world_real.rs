//! Tests of crates/cli/src/app/rules_world_real.rs: the `exists`, `read` and `toplevel` seams of `RulesWorld::real()`
//! are wired to their real functions. The file mixes kinds, each case marked: RITs (cli→ccvcslib/rules, kept beside
//! their caller) and a ZIT. `exists` and `read` are real disk functions, so the only way to tell them from a stub is to
//! use them on the disk: the tests use a throwaway folder under the system temp folder and remove it. `toplevel` asks
//! git through ccvcslib (found on PATH); on a folder that does not exist the spawn fails before git starts, so the
//! answer is Unavailable. One case makes a throwaway git repository and removes it.

use cli::RulesWorld;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

const RULE_FILE: &str = r#"version = 1

[[rule]]
id = "lint-all"
engine = "clippy"
check = "clippy::all"
fix = "Do what clippy says."
"#;

// Tests run in parallel in one process, so each name carries a process-wide counter: no two calls here get the
// same number.
static NEXT: AtomicU64 = AtomicU64::new(0);

/// The next counter-numbered path under the system temp folder.
fn next_path(tag: &str) -> PathBuf {
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("tm-cli-rw-{}-{n}-{tag}", std::process::id()))
}

/// A path under the system temp folder that does not exist; nothing is created there.
fn unique_path(tag: &str) -> PathBuf {
    loop {
        let path = next_path(tag);
        if !path.exists() {
            return path;
        }
    }
}

/// A new folder under the system temp folder, removed on drop (also when an assertion fails). A folder already
/// there is never reused: the next number is tried.
struct TempFolder(PathBuf);

impl TempFolder {
    fn new(tag: &str) -> TempFolder {
        loop {
            let path = next_path(tag);
            match fs::create_dir(&path) {
                Ok(()) => return TempFolder(path),
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(e) => panic!("create {}: {e}", path.display()),
            }
        }
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// ZIT: the seam is `Path::is_file`, std only (crates/cli/src/app/rules_world_real.rs:13).
#[test]
fn exists_is_true_for_a_file_only() {
    let world = RulesWorld::real();
    let dir = TempFolder::new("exists");
    let file = dir.0.join("temperare.toml");
    fs::write(&file, RULE_FILE).unwrap();
    assert!((world.exists)(&file));
    assert!(!(world.exists)(&dir.0), "a folder is not a rule file");
    assert!(!(world.exists)(&dir.0.join("missing.toml")));
}

/// RIT: `RuleFile::read` reads the disk (crates/rules/src/rule_file/rule_file_read.rs:11).
#[test]
fn read_reads_a_rule_file_from_disk() {
    let world = RulesWorld::real();
    let dir = TempFolder::new("read");
    let file = dir.0.join("temperare.toml");
    fs::write(&file, RULE_FILE).unwrap();
    let rule_file = (world.read)(&file).unwrap();
    let ids: Vec<&str> = rule_file.rules.iter().map(|r| r.id.as_str()).collect();
    assert_eq!(ids, vec!["lint-all"]);
}

/// RIT: `RuleFile::read` reads the disk (crates/rules/src/rule_file/rule_file_read.rs:11).
#[test]
fn read_of_a_missing_file_is_one_problem_at_line_zero() {
    let world = RulesWorld::real();
    let dir = TempFolder::new("read-missing");
    let problems = (world.read)(&dir.0.join("missing.toml")).unwrap_err();
    assert_eq!(problems.len(), 1);
    assert_eq!((problems[0].line, problems[0].col), (0, 0));
    assert!(problems[0].what.starts_with("cannot read"), "what: {}", problems[0].what);
}

/// RIT: the real toplevel seam (through ccvcslib) returns git's repository top in native form.
#[test]
fn toplevel_of_a_repository_is_native() {
    let dir = TempFolder::new("toplevel-native");
    let home = dir.0.join("home");
    fs::create_dir(&home).unwrap();
    let mut cmd = procio::Cmd::new("git");
    cmd.cwd = Some(dir.0.clone());
    cmd.env_remove = std::env::vars_os().map(|(name, _)| name).collect();
    cmd.env = vec![
        ("PATH".into(), std::env::var_os("PATH").unwrap_or_else(|| "/usr/bin:/bin".into())),
        ("HOME".into(), home.into_os_string()),
        ("GIT_CONFIG_NOSYSTEM".into(), "1".into()),
    ];
    cmd.args = ["init", "-q"].iter().map(|arg| (*arg).into()).collect();
    let init = procio::run(&cmd).unwrap();
    assert_eq!(init.code, Some(0), "git init: {}", String::from_utf8_lossy(&init.stderr));
    cmd.args = ["rev-parse", "--show-toplevel"].iter().map(|arg| (*arg).into()).collect();
    let output = procio::run(&cmd).unwrap();
    assert_eq!(output.code, Some(0), "git rev-parse: {}", String::from_utf8_lossy(&output.stderr));
    let git_top = String::from_utf8(output.stdout).unwrap();
    let git_top = git_top.trim();
    let world = RulesWorld::real();
    let result = (world.toplevel)(&dir.0).unwrap();
    assert_eq!(result, PathBuf::from(git_top));
    #[cfg(unix)]
    {
        // Git's Unix form is already native, so tm's output is unchanged.
        assert_eq!(result.as_os_str(), std::ffi::OsStr::new(git_top));
    }
    #[cfg(windows)]
    {
        // Git prints C:/..., while tm prints native C:\... paths.
        let native = result.to_str().unwrap();
        assert!(!native.contains('/'), "{native}");
        assert!(!native.starts_with(r"\\?\"), "{native}");
    }
}

/// RIT: git is asked (ccvcslib's git_working_copy_root.rs, through crates/cli/src/app/git_toplevel.rs).
#[test]
fn toplevel_of_a_missing_folder_is_an_error() {
    let world = RulesWorld::real();
    assert!((world.toplevel)(&unique_path("toplevel-missing")).is_err());
}
