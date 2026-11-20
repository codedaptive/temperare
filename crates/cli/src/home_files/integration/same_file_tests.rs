//! Tests the comparison open_log uses to accept an opened handle; these tests do not run open_log.

use crate::home_files::same_file;
use std::fs::{self, File};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

struct Home(PathBuf);
impl Home {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!("tm-same-file-test-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Home {
    fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); }
}

#[test]
fn same_file_accepts_look_and_open_of_same_file() {
    let home = Home::new();
    let path = home.0.join("file");
    fs::write(&path, b"content").unwrap();
    let looked = fs::symlink_metadata(&path).unwrap();
    let opened = File::open(&path).unwrap().metadata().unwrap();
    assert!(same_file(&looked, &opened));
}

#[test]
fn same_file_accepts_hard_link_to_looked_at_file() {
    let home = Home::new();
    let path = home.0.join("file");
    let link = home.0.join("link");
    fs::write(&path, b"content").unwrap();
    fs::hard_link(&path, &link).unwrap();
    let looked = fs::symlink_metadata(&path).unwrap();
    let opened = File::open(&link).unwrap().metadata().unwrap();
    assert!(same_file(&looked, &opened));
}

#[test]
fn same_file_rejects_distinct_files_with_identical_content() {
    let home = Home::new();
    let first = home.0.join("first");
    let second = home.0.join("second");
    fs::write(&first, b"content").unwrap();
    fs::write(&second, b"content").unwrap();
    let looked = fs::symlink_metadata(&first).unwrap();
    let opened = File::open(&second).unwrap().metadata().unwrap();
    assert!(!same_file(&looked, &opened));
}

#[test]
fn same_file_rejects_replacement_at_same_path() {
    let home = Home::new();
    let path = home.0.join("file");
    fs::write(&path, b"content").unwrap();
    let looked = fs::symlink_metadata(&path).unwrap();
    fs::rename(&path, home.0.join("original")).unwrap();
    fs::write(&path, b"content").unwrap();
    let opened = File::open(&path).unwrap().metadata().unwrap();
    assert!(!same_file(&looked, &opened));
}
