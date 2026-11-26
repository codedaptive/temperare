#![cfg(windows)]
// These tests start Windows programs and test-written stubs.

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use procio::{run, Cmd, ProcError};

const STUB: &str = "@echo off\r\necho ran\r\necho ran>marker.txt\r\nexit /b 7\r\n";

fn scratch(test: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("procio-zitw-{}-{test}", std::process::id()));
    fs::create_dir(&dir).unwrap();
    dir
}

fn cmd(body: &str) -> Cmd {
    let mut args = vec![OsString::from("/d"), OsString::from("/c")];
    args.extend(body.split(' ').map(OsString::from));
    Cmd { args, ..Cmd::new("cmd") }
}

#[test]
fn exit_codes_are_answers() {
    for (body, code) in [("exit 0", 0), ("exit 3", 3)] {
        let out = run(&cmd(body)).unwrap();
        assert_eq!(out.code, Some(code), "{body}");
        if code == 0 {
            assert!(out.stdout.is_empty());
            assert!(out.stderr.is_empty());
        }
    }
}

#[test]
fn missing_cwd_with_existing_program_is_spawn() {
    let cwd = std::env::temp_dir().join(format!("procio-zitw-{}-missing-cwd", std::process::id()));
    let result = run(&Cmd { cwd: Some(cwd), ..cmd("exit 0") });
    match result {
        Err(ProcError::Spawn { program, .. }) => assert_eq!(program, OsString::from("cmd")),
        other => panic!("expected Spawn, got {other:?}"),
    }
}

#[test]
fn stdin_is_read_by_findstr() {
    let out = run(&Cmd {
        args: vec!["/R".into(), ".".into()],
        stdin: b"hello\r\nprocio\r\n".to_vec(),
        ..Cmd::new("findstr")
    })
    .unwrap();
    assert_eq!(out.code, Some(0));
    let stdout: Vec<u8> = out.stdout.into_iter().filter(|byte| *byte != b'\r').collect();
    assert_eq!(stdout, b"hello\nprocio\n");
}

#[test]
fn large_output_on_both_pipes_does_not_deadlock() {
    const STDOUT_TEXT: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGH";
    const STDERR_TEXT: &str = "ZYXWVUTSRQPONMLKJIHGFEDCBAZYXWVUTSRQPONMLKJIHGFEDCBAZYXWVUTS";
    // Each of 2000 lines contains 60 letters and a two-byte CRLF.
    const PIPE_BYTES: usize = 2000 * (60 + 2);
    assert_eq!(STDOUT_TEXT.len(), 60);
    assert_eq!(STDERR_TEXT.len(), 60);
    let stdout_loop = format!("(for /L %i in (1,1,2000) do @echo {STDOUT_TEXT})");
    let stderr_loop = format!("(for /L %i in (1,1,2000) do @echo {STDERR_TEXT})");
    let body = format!("{stdout_loop}&{stderr_loop}1>&2");
    let out = run(&Cmd { timeout: Some(Duration::from_secs(120)), ..cmd(&body) }).unwrap();
    assert_eq!(out.code, Some(0));
    assert_eq!(out.stdout.len(), PIPE_BYTES);
    assert_eq!(out.stderr.len(), PIPE_BYTES);
    assert_eq!(first_line(&out.stdout), STDOUT_TEXT.as_bytes());
    assert_eq!(first_line(&out.stderr), STDERR_TEXT.as_bytes());
}

/// The bytes before the first LF, with every CR removed.
fn first_line(bytes: &[u8]) -> Vec<u8> {
    let line = bytes.split(|byte| *byte == b'\n').next().unwrap_or_default();
    line.iter().copied().filter(|byte| *byte != b'\r').collect()
}

#[test]
fn cwd_env_add_and_env_remove_apply() {
    assert!(std::env::var_os("USERNAME").is_some(), "the test process must have USERNAME");
    let dir = scratch("cwd-env");
    let result = run(&Cmd {
        cwd: Some(dir.clone()),
        env: vec![("PROCIO_TEST_A".into(), "yes".into())],
        env_remove: vec!["USERNAME".into()],
        ..cmd("cd&echo %PROCIO_TEST_A%&echo %USERNAME%")
    });
    let actual_cwd = result.as_ref().ok().and_then(|out| {
        let text = String::from_utf8_lossy(&out.stdout);
        text.lines().next().and_then(|line| fs::canonicalize(Path::new(line)).ok())
    });
    let expected_cwd = fs::canonicalize(&dir).unwrap();
    fs::remove_dir_all(&dir).unwrap();
    let out = result.unwrap();
    assert_eq!(out.code, Some(0));
    let text = String::from_utf8_lossy(&out.stdout).replace('\r', "");
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 3);
    assert_eq!(actual_cwd, Some(expected_cwd));
    assert_eq!(lines[1], "yes");
    assert_eq!(lines[2], "%USERNAME%");
}

#[test]
fn timeout_kills_cmd() {
    let after = Duration::from_secs(3);
    let result = run(&Cmd {
        timeout: Some(after),
        ..cmd("for /L %i in (1,1,2147483647) do @rem")
    });
    match result {
        Err(ProcError::TimedOut { program, after: actual }) => {
            assert_eq!(program, OsString::from("cmd"));
            assert_eq!(actual, after);
        }
        other => panic!("expected TimedOut, got {other:?}"),
    }
}

/// Requires this test process's PATHEXT to list .CMD.
#[test]
fn pathext_resolves_a_stub_through_run() {
    let dir = scratch("pathext-run");
    fs::write(dir.join("stub.cmd"), STUB).unwrap();
    let result = run(&Cmd {
        cwd: Some(dir.clone()),
        env: vec![("PATH".into(), dir.clone().into_os_string())],
        ..Cmd::new("stub")
    });
    let marker_exists = dir.join("marker.txt").exists();
    fs::remove_dir_all(&dir).unwrap();
    let out = result.unwrap();
    assert_eq!(out.code, Some(7));
    let stdout: Vec<u8> = out.stdout.into_iter().filter(|byte| *byte != b'\r').collect();
    assert_eq!(stdout, b"ran\n");
    assert!(marker_exists);
}

/// Requires this test process's PATHEXT to list .CMD.
#[test]
fn bare_name_never_runs_a_stub_from_cwd() {
    let dir = scratch("cwd-stub");
    let path = scratch("empty-path");
    fs::write(dir.join("stub.cmd"), STUB).unwrap();
    let result = run(&Cmd {
        cwd: Some(dir.clone()),
        env: vec![("PATH".into(), path.clone().into_os_string())],
        ..Cmd::new("stub")
    });
    let marker_exists = dir.join("marker.txt").exists();
    fs::remove_dir_all(&dir).unwrap();
    fs::remove_dir_all(&path).unwrap();
    match result {
        Err(ProcError::NotFound { program }) => assert_eq!(program, OsString::from("stub")),
        other => panic!("expected NotFound, got {other:?}"),
    }
    assert!(!marker_exists);
}
