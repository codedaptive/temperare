//! Integration tests for read_head on real temporary files. The libc calls require unsafe for the effective UID and FIFO creation.

use crate::shellcheck::read_head;
use std::ffi::CString;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::PathBuf;

fn tree(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("engines-zit-{}-read-head-{name}", std::process::id()));
    fs::create_dir(&root).unwrap();
    root
}

#[test]
fn short_script_returns_all_its_bytes() {
    let root = tree("script");
    let path = root.join("script");
    fs::write(&path, b"#!/bin/sh\necho hi\n").unwrap();
    let got = read_head(&path);
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Some(b"#!/bin/sh\necho hi\n".to_vec()));
}

#[test]
fn long_file_returns_exactly_its_first_257_bytes() {
    let root = tree("long");
    let path = root.join("long");
    fs::write(&path, [vec![b'a'; 257], vec![b'b'; 43]].concat()).unwrap();
    let got = read_head(&path);
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Some(vec![b'a'; 257]));
}

#[test]
fn empty_file_returns_some_empty_bytes() {
    let root = tree("empty");
    let path = root.join("empty");
    fs::write(&path, b"").unwrap();
    let got = read_head(&path);
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, Some(vec![]));
}

#[test]
fn symlink_to_script_returns_none() {
    let root = tree("symlink");
    let script = root.join("script");
    let link = root.join("link");
    fs::write(&script, b"#!/bin/sh\necho hi\n").unwrap();
    symlink(&script, &link).unwrap();
    let got = read_head(&link);
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, None);
}

#[test]
fn directory_returns_none() {
    let root = tree("directory");
    let got = read_head(&root);
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, None);
}

#[test]
fn a_mode_000_file_is_none_unless_root_reads_it() {
    // SAFETY: geteuid takes no arguments, touches no memory and cannot fail.
    let as_root = unsafe { libc::geteuid() } == 0;
    let root = tree("unreadable");
    let path = root.join("unreadable");
    fs::write(&path, b"secret").unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o000)).unwrap();
    let got = read_head(&path);
    fs::remove_dir_all(&root).unwrap();
    // Root reads a mode-000 file; anyone else's open fails, which is None.
    let expected = if as_root { Some(b"secret".to_vec()) } else { None };
    assert_eq!(got, expected);
}

#[test]
fn nonexistent_path_returns_none() {
    let root = tree("missing");
    let got = read_head(&root.join("missing"));
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, None);
}

#[test]
fn fifo_returns_none_without_blocking() {
    let root = tree("fifo");
    let path = root.join("fifo");
    let c_path = CString::new(path.as_os_str().as_bytes()).unwrap();
    assert_eq!(unsafe { libc::mkfifo(c_path.as_ptr(), 0o600) }, 0);
    let got = read_head(&path);
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, None);
}
