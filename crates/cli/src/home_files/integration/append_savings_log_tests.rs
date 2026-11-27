//! Integration tests for append_savings_log on a test HOME: creation, append order, silent failures,
//! refusal to write through links, and unchanged directory logs or files at the state or cctao paths.

use crate::home_files::append_savings_log;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

struct Home(PathBuf);
impl Home {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!("cctao-log-test-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
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
    assert_eq!(std::fs::read(home.0.join(".local/state/cctao/savings.jsonl")).unwrap(), b"first\n");
}

#[test]
fn savings_contract_appends_two_lines_in_order() {
    let home = Home::new();
    append_savings_log(Some(&home.0), "first");
    append_savings_log(Some(&home.0), "second");
    assert_eq!(std::fs::read(home.0.join(".local/state/cctao/savings.jsonl")).unwrap(), b"first\nsecond\n");
}

#[test]
fn savings_contract_unwritable_path_is_silent_and_unchanged() {
    let home = Home::new();
    std::fs::write(home.0.join(".local"), b"occupied").unwrap();
    append_savings_log(Some(&home.0), "line");
    assert_eq!(std::fs::read(home.0.join(".local")).unwrap(), b"occupied");
    assert!(!home.0.join(".local/state/cctao/savings.jsonl").exists());
}

#[test]
fn savings_contract_directory_at_log_path_remains_empty() {
    let home = Home::new();
    let log = home.0.join(".local/state/cctao/savings.jsonl");
    std::fs::create_dir_all(&log).unwrap();

    append_savings_log(Some(&home.0), "line");

    assert!(log.is_dir());
    assert_eq!(std::fs::read_dir(&log).unwrap().count(), 0);
}

#[test]
fn savings_contract_file_at_state_path_is_unchanged() {
    let home = Home::new();
    std::fs::create_dir_all(home.0.join(".local")).unwrap();
    let state = home.0.join(".local/state");
    std::fs::write(&state, b"keep").unwrap();

    append_savings_log(Some(&home.0), "line");

    assert_eq!(std::fs::read(&state).unwrap(), b"keep");
}

#[test]
fn savings_contract_file_at_cctao_path_is_unchanged() {
    let home = Home::new();
    std::fs::create_dir_all(home.0.join(".local/state")).unwrap();
    let cctao = home.0.join(".local/state/cctao");
    std::fs::write(&cctao, b"keep").unwrap();

    append_savings_log(Some(&home.0), "line");

    assert!(cctao.is_file());
    assert_eq!(std::fs::read(&cctao).unwrap(), b"keep");
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

#[cfg(unix)]
#[test]
fn savings_contract_does_not_follow_log_link_to_existing_file() {
    use std::os::unix::fs::symlink;
    let home = Home::new();
    let folder = home.0.join(".local/state/cctao");
    std::fs::create_dir_all(&folder).unwrap();
    let target = home.0.join("target.txt");
    std::fs::write(&target, b"keep").unwrap();
    let link = folder.join("savings.jsonl");
    symlink(&target, &link).unwrap();

    append_savings_log(Some(&home.0), "line");

    assert!(std::fs::symlink_metadata(&link).unwrap().file_type().is_symlink());
    assert_eq!(std::fs::read(&target).unwrap(), b"keep");
}

#[cfg(unix)]
#[test]
fn savings_contract_does_not_follow_dangling_log_link() {
    use std::os::unix::fs::symlink;
    let home = Home::new();
    let folder = home.0.join(".local/state/cctao");
    std::fs::create_dir_all(&folder).unwrap();
    let target = home.0.join("absent.txt");
    let link = folder.join("savings.jsonl");
    symlink(&target, &link).unwrap();

    append_savings_log(Some(&home.0), "line");

    assert!(std::fs::symlink_metadata(&link).unwrap().file_type().is_symlink());
    assert!(!target.exists());
}

#[cfg(unix)]
#[test]
fn savings_contract_does_not_follow_cctao_folder_link() {
    use std::os::unix::fs::symlink;
    let home = Home::new();
    std::fs::create_dir_all(home.0.join(".local/state")).unwrap();
    let target = home.0.join("elsewhere");
    std::fs::create_dir(&target).unwrap();
    let link = home.0.join(".local/state/cctao");
    symlink(&target, &link).unwrap();

    append_savings_log(Some(&home.0), "line");

    assert!(std::fs::symlink_metadata(&link).unwrap().file_type().is_symlink());
    assert_eq!(std::fs::read_dir(&target).unwrap().count(), 0);
}

#[cfg(unix)]
#[test]
fn savings_contract_does_not_follow_state_folder_link() {
    use std::os::unix::fs::symlink;
    let home = Home::new();
    std::fs::create_dir(home.0.join(".local")).unwrap();
    let target = home.0.join("elsewhere");
    std::fs::create_dir(&target).unwrap();
    let link = home.0.join(".local/state");
    symlink(&target, &link).unwrap();

    append_savings_log(Some(&home.0), "line");

    assert!(std::fs::symlink_metadata(&link).unwrap().file_type().is_symlink());
    assert_eq!(std::fs::read_dir(&target).unwrap().count(), 0);
}

#[cfg(unix)]
#[test]
fn savings_contract_does_not_follow_local_folder_link() {
    use std::os::unix::fs::symlink;
    let home = Home::new();
    let target = home.0.join("elsewhere");
    std::fs::create_dir(&target).unwrap();
    let link = home.0.join(".local");
    symlink(&target, &link).unwrap();

    append_savings_log(Some(&home.0), "line");

    assert!(std::fs::symlink_metadata(&link).unwrap().file_type().is_symlink());
    assert_eq!(std::fs::read_dir(&target).unwrap().count(), 0);
}
