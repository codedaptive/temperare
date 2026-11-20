//! Integration test for append_savings_log, on a HOME folder the test made: log creation, append order and
//! silent failures.

use crate::home_files::append_savings_log;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

struct Home(PathBuf);
impl Home {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!("tm-log-test-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Home {
    fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.0); }
}

#[test]
fn savings_contract_creates_folders_and_writes_line_with_newline() {
    let home = Home::new();
    append_savings_log(Some(&home.0), "first");
    assert_eq!(std::fs::read(home.0.join(".local/state/temperare/savings.jsonl")).unwrap(), b"first\n");
}

#[test]
fn savings_contract_appends_two_lines_in_order() {
    let home = Home::new();
    append_savings_log(Some(&home.0), "first");
    append_savings_log(Some(&home.0), "second");
    assert_eq!(std::fs::read(home.0.join(".local/state/temperare/savings.jsonl")).unwrap(), b"first\nsecond\n");
}

#[test]
fn savings_contract_unwritable_path_is_silent_and_unchanged() {
    let home = Home::new();
    std::fs::write(home.0.join(".local"), b"occupied").unwrap();
    append_savings_log(Some(&home.0), "line");
    assert_eq!(std::fs::read(home.0.join(".local")).unwrap(), b"occupied");
    assert!(!home.0.join(".local/state/temperare/savings.jsonl").exists());
}

#[test]
fn savings_contract_absent_or_empty_home_does_nothing() {
    let home = Home::new();
    let cwd_local = std::env::current_dir().unwrap().join(".local");
    // An empty HOME would put .local under the current directory (the crate's folder under cargo test).
    assert!(!cwd_local.exists(), "no .local under {} before", cwd_local.display());
    append_savings_log(None, "line");
    append_savings_log(Some(Path::new("")), "line");
    assert!(!cwd_local.exists(), "no .local under {} after", cwd_local.display());
    assert_eq!(std::fs::read_dir(&home.0).unwrap().count(), 0);
}
