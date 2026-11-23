//! Regular file checks against real temporary filesystem entries.

use crate::is_regular_file::is_regular_file;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

fn tree(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("engines-zit-{}-regular-file-{name}", std::process::id()));
    fs::create_dir(&root).unwrap();
    fs::canonicalize(root).unwrap()
}

#[test]
fn regular_file_returns_true() {
    let root = tree("file");
    fs::write(root.join("file"), "contents").unwrap();
    let got = is_regular_file(&root, &root.join("file"));
    fs::remove_dir_all(root).unwrap();
    assert!(got);
}

#[test]
fn symlink_to_file_returns_false() {
    let root = tree("symlink");
    fs::write(root.join("file"), "contents").unwrap();
    symlink("file", root.join("link")).unwrap();
    let got = is_regular_file(&root, &root.join("link"));
    fs::remove_dir_all(root).unwrap();
    assert!(!got);
}

#[test]
fn directory_returns_false() {
    let root = tree("directory");
    let got = is_regular_file(&root, &root);
    fs::remove_dir_all(root).unwrap();
    assert!(!got);
}

#[test]
fn missing_path_returns_false() {
    let root = tree("missing");
    let got = is_regular_file(&root, &root.join("missing"));
    fs::remove_dir_all(root).unwrap();
    assert!(!got);
}

#[test]
fn dangling_symlink_returns_false() {
    let root = tree("dangling");
    symlink("missing", root.join("link")).unwrap();
    let got = is_regular_file(&root, &root.join("link"));
    fs::remove_dir_all(root).unwrap();
    assert!(!got);
}

#[test]
fn regular_file_below_symlinked_folder_returns_false() {
    let root = tree("symlinked-folder");
    fs::create_dir(root.join("top")).unwrap();
    fs::create_dir(root.join("outside")).unwrap();
    fs::write(root.join("outside/file"), "contents").unwrap();
    symlink("../outside", root.join("top/link")).unwrap();
    let top = fs::canonicalize(root.join("top")).unwrap();
    let got = is_regular_file(&top, &top.join("link/file"));
    fs::remove_dir_all(root).unwrap();
    assert!(!got);
}

#[test]
fn regular_file_outside_top_returns_false() {
    let root = tree("outside-top");
    fs::create_dir(root.join("top")).unwrap();
    fs::write(root.join("outside"), "contents").unwrap();
    let top = fs::canonicalize(root.join("top")).unwrap();
    let got = is_regular_file(&top, &root.join("outside"));
    fs::remove_dir_all(root).unwrap();
    assert!(!got);
}

#[test]
fn regular_file_in_real_subfolder_returns_true() {
    let root = tree("real-subfolder");
    fs::create_dir(root.join("sub")).unwrap();
    fs::write(root.join("sub/file"), "contents").unwrap();
    let got = is_regular_file(&root, &root.join("sub/file"));
    fs::remove_dir_all(root).unwrap();
    assert!(got);
}
