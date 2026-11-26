//! procio's integration tests for the runner: `run` and `which` on real programs, with a throwaway dir
//! where a test needs files.

use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;

use procio::{run, which, Cmd, ProcError};

const MISSING: &str = "procio-no-such-program-zit";

/// A fresh, empty dir under the temp dir, unique to this process and test; the test removes it at its end.
fn scratch(test: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("procio-zit-{}-{test}", std::process::id()));
    fs::create_dir(&dir).unwrap();
    dir
}

fn sh(script: &str) -> Cmd {
    Cmd { args: vec!["-c".into(), script.into()], ..Cmd::new("sh") }
}

#[test]
fn true_exits_zero_with_empty_output() {
    let out = run(&Cmd::new("true")).unwrap();
    assert_eq!(out.code, Some(0));
    assert_eq!(out.stdout, b"");
    assert_eq!(out.stderr, b"");
}

#[test]
fn false_is_an_answer_not_an_error() {
    let out = run(&Cmd::new("false")).unwrap();
    assert_eq!(out.code, Some(1));
}

#[test]
fn missing_program_on_path_is_not_found() {
    match run(&Cmd::new(MISSING)) {
        Err(ProcError::NotFound { program }) => assert_eq!(program, OsString::from(MISSING)),
        other => panic!("expected NotFound, got {other:?}"),
    }
}

/// An empty PATH entry must not run a same-named executable in the child's cwd.
#[cfg(unix)]
#[test]
fn empty_path_entry_does_not_run_cwd_executable() {
    use std::os::unix::fs::PermissionsExt;

    let dir = scratch("empty_path_entry");
    let name = format!("procio-zit-cwd-trap-{}", std::process::id());
    let executable = dir.join(&name);
    let marker = dir.join("marker");
    fs::write(&executable, "#!/bin/sh\nprintf ran > \"$PROCIO_ZIT_MARKER\"\n").unwrap();
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o755)).unwrap();
    let result = run(&Cmd {
        cwd: Some(dir.clone()),
        env: vec![("PATH".into(), ":/usr/bin:/bin".into()), ("PROCIO_ZIT_MARKER".into(), marker.clone().into())],
        ..Cmd::new(&name)
    });
    let marker_exists = marker.exists();
    fs::remove_dir_all(&dir).unwrap();
    assert!(
        matches!(&result, Err(ProcError::NotFound { program }) if program == &OsString::from(&name)),
        "expected NotFound for {name}, got {result:?}; marker exists: {marker_exists}"
    );
    assert!(!marker_exists, "the cwd executable ran");
}

#[test]
fn missing_absolute_path_is_not_found() {
    let dir = scratch("missing_absolute_path");
    let path = dir.join("absent-program");
    let result = run(&Cmd::new(path.clone()));
    fs::remove_dir_all(&dir).unwrap();
    match result {
        Err(ProcError::NotFound { program }) => assert_eq!(program, path.into_os_string()),
        other => panic!("expected NotFound, got {other:?}"),
    }
}

#[test]
fn missing_cwd_with_existing_program_is_spawn() {
    // Never created: the program exists, so the failure is the spawn, not NotFound.
    let cwd = std::env::temp_dir().join(format!("procio-zit-{}-missing-cwd", std::process::id()));
    match run(&Cmd { cwd: Some(cwd), ..Cmd::new("true") }) {
        Err(ProcError::Spawn { program, .. }) => assert_eq!(program, OsString::from("true")),
        other => panic!("expected Spawn, got {other:?}"),
    }
}

#[test]
fn stdin_is_echoed_through_cat() {
    let out = run(&Cmd { stdin: b"hello\nprocio\n".to_vec(), ..Cmd::new("cat") }).unwrap();
    assert_eq!(out.code, Some(0));
    assert_eq!(out.stdout, b"hello\nprocio\n");
}

#[test]
fn large_output_on_both_pipes_does_not_deadlock() {
    // 1 MiB on each pipe is far past a pipe buffer: reading one pipe after the other would block the child.
    // The timeout only turns a deadlock into TimedOut instead of a hang; no duration is asserted.
    let cmd = Cmd {
        timeout: Some(Duration::from_secs(120)),
        ..sh("head -c 1048576 /dev/zero; head -c 1048576 /dev/zero >&2")
    };
    let out = run(&cmd).unwrap();
    assert_eq!(out.code, Some(0));
    assert_eq!(out.stdout.len(), 1_048_576);
    assert_eq!(out.stderr.len(), 1_048_576);
}

#[test]
fn timeout_kills_and_reaps_the_child() {
    let dir = scratch("timeout_kills_and_reaps");
    let pid_file = dir.join("pid");
    let script = format!("echo $$ > '{}'; exec sleep 30", pid_file.display());
    let cmd = Cmd { timeout: Some(Duration::from_secs(5)), ..sh(&script) };
    let result = run(&cmd);
    let pid = fs::read_to_string(&pid_file).unwrap().trim().to_string();
    fs::remove_dir_all(&dir).unwrap();
    match result {
        Err(ProcError::TimedOut { program, after }) => {
            assert_eq!(program, OsString::from("sh"));
            assert_eq!(after, Duration::from_secs(5));
        }
        other => panic!("expected TimedOut, got {other:?}"),
    }
    // A zombie still answers kill -0, so a failure here proves the child was killed and reaped. Its "No such
    // process" on stderr is that expected answer, so stderr is discarded.
    let alive = Command::new("kill").args(["-0", &pid]).stderr(Stdio::null()).status().unwrap();
    assert!(!alive.success(), "child {pid} still exists after TimedOut");
}

/// True when `pid` no longer runs: `ps` finds no such process, or finds a zombie. A grandchild killed with the
/// child's group is reparented to init and stays a zombie until init reaps it, so a zombie counts as gone.
fn is_gone(pid: &str) -> bool {
    let out = Command::new("ps").args(["-o", "stat=", "-p", pid]).output().unwrap();
    let stat = String::from_utf8_lossy(&out.stdout);
    !out.status.success() || stat.trim().is_empty() || stat.trim_start().starts_with('Z')
}

#[test]
fn timeout_kills_a_grandchild_holding_stdout() {
    let dir = scratch("timeout_kills_a_grandchild");
    let pid_file = dir.join("pid");
    // The child exits at once; only the backgrounded sleep, which inherits stdout, keeps the pipe open.
    let script = format!("sleep 60 & echo $! > '{}'; exit 0", pid_file.display());
    let cmd = Cmd { timeout: Some(Duration::from_secs(3)), ..sh(&script) };
    let result = run(&cmd);
    let pid = fs::read_to_string(&pid_file).unwrap().trim().to_string();
    // Waits for an event, not a duration: up to 600 polls of 100 ms for the grandchild to go.
    let gone = (0..600).any(|_| {
        if is_gone(&pid) {
            return true;
        }
        std::thread::sleep(Duration::from_millis(100));
        false
    });
    if !gone {
        Command::new("kill").args(["-KILL", &pid]).status().unwrap();
    }
    fs::remove_dir_all(&dir).unwrap();
    match result {
        Err(ProcError::TimedOut { program, after }) => {
            assert_eq!(program, OsString::from("sh"));
            assert_eq!(after, Duration::from_secs(3));
        }
        other => panic!("expected TimedOut, got {other:?}"),
    }
    assert!(gone, "grandchild {pid} still running after TimedOut");
}

#[test]
fn cwd_env_add_and_env_remove_apply() {
    let dir = scratch("cwd_env");
    let cmd = Cmd {
        cwd: Some(dir.clone()),
        env: vec![("PROCIO_TEST_A".into(), "yes".into())],
        env_remove: vec!["HOME".into()],
        ..sh(r#"pwd -P; echo "$PROCIO_TEST_A"; echo "${HOME-unset}""#)
    };
    let result = run(&cmd);
    let real = fs::canonicalize(&dir).unwrap();
    fs::remove_dir_all(&dir).unwrap();
    let out = result.unwrap();
    assert_eq!(out.code, Some(0));
    let expected = format!("{}\nyes\nunset\n", real.display());
    assert_eq!(String::from_utf8(out.stdout).unwrap(), expected);
}

#[test]
fn which_finds_sh() {
    let found = which("sh").unwrap();
    assert_eq!(found.file_name().unwrap(), "sh");
    assert!(found.exists());
}

#[test]
fn which_misses_an_absent_program() {
    assert_eq!(which(MISSING), None);
}
