//! ZITs for telemetry written by the binary in throwaway git repositories.

use serde_json::{json, Value};
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
            let path = std::env::temp_dir().join(format!("tm-telemetry-{}-{n}", std::process::id()));
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

struct World {
    temp: TempFolder,
    repo: PathBuf,
    home: PathBuf,
}

impl World {
    fn new() -> Self {
        let temp = TempFolder::new();
        let repo = temp.0.join("repo");
        let home = temp.0.join("home");
        fs::create_dir(&repo).unwrap();
        fs::create_dir(&home).unwrap();
        let world = Self { temp, repo, home };
        world.git(&world.repo, &["init", "-q"]);
        fs::write(world.repo.join("temperare.toml"), r#"version = 1
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
        world
    }

    fn command(&self, program: impl Into<OsString>, repo: &Path, args: &[&str]) -> procio::Output {
        let mut cmd = procio::Cmd {
            cwd: Some(repo.to_path_buf()),
            env_remove: std::env::vars_os().map(|(name, _)| name).collect(),
            env: vec![
                ("PATH".into(), std::env::var_os("PATH").unwrap_or_else(|| "/usr/bin:/bin".into())),
                ("HOME".into(), self.home.as_os_str().into()),
                ("TMPDIR".into(), self.temp.0.as_os_str().into()),
                ("GIT_CONFIG_NOSYSTEM".into(), "1".into()),
            ],
            ..procio::Cmd::new(program)
        };
        cmd.args = args.iter().map(OsString::from).collect();
        procio::run(&cmd).unwrap()
    }

    fn git(&self, repo: &Path, args: &[&str]) -> String {
        let output = self.command("git", repo, args);
        assert_eq!(output.code, Some(0), "git {args:?}: {}", String::from_utf8_lossy(&output.stderr));
        String::from_utf8(output.stdout).unwrap().trim().to_owned()
    }

    fn commit(&self, message: &str) {
        self.git(&self.repo, &["add", "-A"]);
        self.git(&self.repo, &["-c", "user.name=Test", "-c", "user.email=test@example.invalid", "-c", "commit.gpgsign=false", "commit", "-q", "--allow-empty", "-m", message]);
    }

    fn check(&self, repo: &Path, args: &[&str], code: i32) -> procio::Output {
        let output = self.command(env!("CARGO_BIN_EXE_temperare"), repo, args);
        assert_eq!(output.code, Some(code), "stdout: {}\nstderr: {}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        output
    }

    fn telemetry(&self) -> PathBuf {
        self.home.join(".local/state/temperare/telemetry")
    }
}

/// ZIT: a merged history writes one ordered telemetry record under its first-parent root.
#[test]
fn merged_history_records_first_parent_root_and_fields() {
    let world = World::new();
    world.git(&world.repo, &["checkout", "-q", "-b", "main"]);
    world.commit("root");
    let root = world.git(&world.repo, &["rev-parse", "HEAD"]);
    world.git(&world.repo, &["checkout", "-q", "-b", "side"]);
    world.commit("side");
    world.git(&world.repo, &["checkout", "-q", "main"]);
    for message in ["main one", "main two", "main three", "main four"] {
        world.commit(message);
    }
    world.git(&world.repo, &["-c", "user.name=Test", "-c", "user.email=test@example.invalid", "-c", "commit.gpgsign=false", "merge", "-q", "--no-ff", "--no-edit", "side"]);
    world.git(&world.repo, &["checkout", "-q", "--orphan", "unrelated"]);
    world.git(&world.repo, &["rm", "-q", "-r", "--cached", "."]);
    fs::remove_file(world.repo.join("temperare.toml")).unwrap();
    fs::write(world.repo.join("unrelated.txt"), "unrelated history\n").unwrap();
    world.commit("unrelated root");
    world.git(&world.repo, &["checkout", "-q", "main"]);
    world.git(&world.repo, &["-c", "user.name=Test", "-c", "user.email=test@example.invalid", "-c", "commit.gpgsign=false", "merge", "-q", "--no-ff", "--no-edit", "--allow-unrelated-histories", "unrelated"]);
    let head = world.git(&world.repo, &["rev-parse", "HEAD"]);

    let output = world.check(&world.repo, &["check"], 0);
    assert!(!String::from_utf8_lossy(&output.stdout).contains("telemetry"));
    let file = world.telemetry().join(format!("{root}.jsonl"));
    assert!(file.is_file(), "missing telemetry file {}", file.display());
    let files: Vec<_> = fs::read_dir(world.telemetry()).unwrap().collect();
    assert_eq!(files.len(), 1);
    let text = fs::read_to_string(file).unwrap();
    assert!(text.ends_with('\n'));
    assert_eq!(text.lines().count(), 1);
    let record: Value = serde_json::from_str(&text).unwrap();
    let keys = ["format", "time", "version", "folder", "head", "base", "kind", "paths", "exit", "engines", "rules"];
    assert_eq!(record.as_object().unwrap().len(), keys.len());
    let positions: Vec<_> = keys.iter().map(|key| text.find(&format!("\"{key}\":")).unwrap()).collect();
    assert!(positions.windows(2).all(|pair| pair[0] < pair[1]), "{text}");
    assert_eq!(record["format"], 1);
    assert_eq!(record["folder"], world.repo.to_str().unwrap());
    assert_eq!(record["head"], head);
    assert_eq!(record["base"], Value::Null);
    assert_eq!(record["kind"], Value::Null);
    assert_eq!(record["paths"], json!([]));
    assert_eq!(record["exit"], 0);
    assert!(record["engines"].is_array());
    assert!(record["rules"].is_array());
    let version = world.check(&world.repo, &["--version"], 0);
    let version = std::str::from_utf8(&version.stdout).unwrap().strip_prefix("temperare ").unwrap().trim_end_matches('\n');
    assert_eq!(record["version"], version);
    let time = record["time"].as_str().unwrap().as_bytes();
    assert_eq!(time.len(), 20);
    for (i, byte) in time.iter().enumerate() {
        match i {
            4 | 7 => assert_eq!(*byte, b'-'),
            10 => assert_eq!(*byte, b'T'),
            13 | 16 => assert_eq!(*byte, b':'),
            19 => assert_eq!(*byte, b'Z'),
            _ => assert!(byte.is_ascii_digit()),
        }
    }
}

/// ZIT: checks in two worktrees append their distinct folders to one root file.
#[test]
fn worktrees_append_to_the_same_file() {
    let world = World::new();
    world.commit("root");
    let root = world.git(&world.repo, &["rev-parse", "HEAD"]);
    let other = world.temp.0.join("other");
    world.git(&world.repo, &["worktree", "add", "-q", "-b", "other", other.to_str().unwrap()]);
    world.check(&world.repo, &["check"], 0);
    world.check(&other, &["check"], 0);
    let file = world.telemetry().join(format!("{root}.jsonl"));
    assert!(file.is_file(), "missing telemetry file {}", file.display());
    assert_eq!(fs::read_dir(world.telemetry()).unwrap().count(), 1);
    let text = fs::read_to_string(file).unwrap();
    let records: Vec<Value> = text.lines().map(|line| serde_json::from_str(line).unwrap()).collect();
    assert_eq!(records.len(), 2);
    assert_eq!(records[0]["folder"], world.repo.to_str().unwrap());
    assert_eq!(records[1]["folder"], other.to_str().unwrap());
}

/// ZIT: a link at the telemetry file is refused without changing output or its target.
#[cfg(unix)]
#[test]
fn symlink_file_is_refused() {
    use std::os::unix::fs::symlink;

    let world = World::new();
    world.commit("root");
    let root = world.git(&world.repo, &["rev-parse", "HEAD"]);
    let baseline = world.check(&world.repo, &["check"], 0);
    fs::create_dir_all(world.telemetry()).unwrap();
    let file = world.telemetry().join(format!("{root}.jsonl"));
    if file.exists() {
        fs::remove_file(&file).unwrap();
    }
    let target = world.temp.0.join("outside-home");
    fs::write(&target, "").unwrap();
    symlink(&target, &file).unwrap();
    let output = world.check(&world.repo, &["check"], 0);
    assert_eq!(output.stdout, baseline.stdout);
    assert_eq!(output.stderr, baseline.stderr);
    assert_eq!(fs::read(&target).unwrap(), b"");
    assert!(fs::symlink_metadata(file).unwrap().file_type().is_symlink());
}

/// ZIT: an unborn HEAD prints a report without creating telemetry.
#[test]
fn unborn_head_writes_no_telemetry() {
    let world = World::new();
    world.check(&world.repo, &["check"], 0);
    assert!(!world.telemetry().exists());
}

/// ZIT: an unresolved base exits 2 without creating telemetry.
#[test]
fn unresolved_base_writes_no_telemetry() {
    let world = World::new();
    world.commit("root");
    world.check(&world.repo, &["check", "--base", "no-such-rev"], 2);
    assert!(!world.telemetry().exists());
}
