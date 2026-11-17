//! Unix only: the files are made executable through their mode bits.
#![cfg(unix)]

use std::ffi::{OsStr, OsString};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use crate::which::which_in;

/// Two fresh dirs under the temp dir, unique to this process and test; `cleanup` removes them.
fn dirs(test: &str) -> (PathBuf, PathBuf) {
    let root = std::env::temp_dir().join(format!("procio-zut-which-{}-{test}", std::process::id()));
    let (a, b) = (root.join("a"), root.join("b"));
    fs::create_dir_all(&a).unwrap();
    fs::create_dir_all(&b).unwrap();
    (a, b)
}

fn cleanup(a: &Path) {
    fs::remove_dir_all(a.parent().unwrap()).unwrap();
}

fn file(dir: &Path, name: &str, mode: u32) -> PathBuf {
    let path = dir.join(name);
    fs::write(&path, b"").unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(mode)).unwrap();
    path
}

fn path_var(dirs: &[&Path]) -> OsString {
    std::env::join_paths(dirs).unwrap()
}

#[test]
fn first_dir_on_path_wins() {
    let (a, b) = dirs("first_wins");
    let in_a = file(&a, "tool", 0o755);
    file(&b, "tool", 0o755);
    let found = which_in(OsStr::new("tool"), Some(&path_var(&[&a, &b])));
    cleanup(&a);
    assert_eq!(found, Some(in_a));
}

#[test]
fn a_non_executable_file_is_skipped() {
    let (a, b) = dirs("non_exec_skipped");
    file(&a, "tool", 0o644);
    let in_b = file(&b, "tool", 0o700);
    let found = which_in(OsStr::new("tool"), Some(&path_var(&[&a, &b])));
    cleanup(&a);
    assert_eq!(found, Some(in_b));
}

#[test]
fn only_non_executable_files_give_none() {
    let (a, b) = dirs("only_non_exec");
    file(&a, "tool", 0o644);
    let found = which_in(OsStr::new("tool"), Some(&path_var(&[&a, &b])));
    cleanup(&a);
    assert_eq!(found, None);
}

#[test]
fn a_directory_with_the_name_is_not_a_program() {
    let (a, b) = dirs("dir_not_program");
    fs::create_dir(a.join("tool")).unwrap();
    let in_b = file(&b, "tool", 0o755);
    let found = which_in(OsStr::new("tool"), Some(&path_var(&[&a, &b])));
    cleanup(&a);
    assert_eq!(found, Some(in_b));
}

#[test]
fn a_missing_dir_on_path_is_passed_over() {
    let (a, b) = dirs("missing_dir");
    let in_b = file(&b, "tool", 0o755);
    let missing = a.join("not-there");
    let found = which_in(OsStr::new("tool"), Some(&path_var(&[&missing, &b])));
    cleanup(&a);
    assert_eq!(found, Some(in_b));
}

#[test]
fn a_path_with_a_separator_is_checked_as_given_without_path() {
    let (a, b) = dirs("separator_path");
    let in_a = file(&a, "tool", 0o755);
    file(&b, "tool", 0o755);
    let given = which_in(in_a.as_os_str(), Some(&path_var(&[&b])));
    let absent = which_in(a.join("absent").as_os_str(), Some(&path_var(&[&b])));
    let not_exec = which_in(file(&a, "plain", 0o644).as_os_str(), Some(&path_var(&[&b])));
    cleanup(&a);
    assert_eq!(given, Some(in_a));
    assert_eq!(absent, None);
    assert_eq!(not_exec, None);
}

#[test]
fn no_path_gives_none() {
    assert_eq!(which_in(OsStr::new("tool"), None), None);
}
