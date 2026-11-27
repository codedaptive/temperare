#![cfg(windows)]
// These tests look up Windows programs and test-written stubs.

use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

use procio::{which, which_in_dir};

#[test]
fn which_finds_cmd() {
    let path = which("cmd").unwrap();
    assert!(path.file_name().unwrap().eq_ignore_ascii_case("cmd.exe"));
    assert!(path.is_file());
}

/// Requires this test process's PATHEXT to list .CMD.
#[test]
fn which_in_dir_finds_the_stub_only_in_an_absolute_dir() {
    let dir = std::env::temp_dir().join(format!("procio-zitw-{}-which-in-dir", std::process::id()));
    fs::create_dir(&dir).unwrap();
    fs::write(dir.join("stub.cmd"), "@echo off\r\n").unwrap();
    let rel = PathBuf::from(format!("procio-zitw-{}-rel-dir", std::process::id()));
    fs::create_dir(&rel).unwrap();
    fs::write(rel.join("stub.cmd"), "@echo off\r\n").unwrap();
    let expected = fs::canonicalize(dir.join("stub.cmd")).unwrap();
    // No colon row: names_to_try's "colon with extension rules" case gives no names for a name holding `:` under
    // PATHEXT, and which_in_dir tries only those names (src/names_to_try_tests.rs).
    let cases = [
        (dir.as_path(), "stub", Some(expected)),
        (dir.as_path(), "absent", None),
        // The folder holds stub.cmd: without the absolute-dir guard, first_in_dir would find it via cwd.
        (rel.as_path(), "stub", None),
    ];
    let results: Vec<_> = cases
        .iter()
        .map(|(lookup, name, _)| {
            which_in_dir(lookup, OsStr::new(name)).map(|path| fs::canonicalize(path).unwrap())
        })
        .collect();
    fs::remove_dir_all(&dir).unwrap();
    fs::remove_dir_all(&rel).unwrap();
    for ((lookup, name, expected), result) in cases.iter().zip(results) {
        assert_eq!(&result, expected, "dir {lookup:?}, name {name}");
    }
}
