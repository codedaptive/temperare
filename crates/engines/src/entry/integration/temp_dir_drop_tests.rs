//! Integration tests for temp_dir_drop.rs on real temp folders: dropping the value removes its file and folder; a file already
//! gone is no panic; a folder holding something else keeps it; a file the value did not create is kept.

use crate::entry::TempDir;

#[test]
fn dropping_the_value_removes_its_file_and_folder() {
    let temp = TempDir::write("clippy", "clippy.toml", "x = 1\n").unwrap();
    let path = temp.path.clone();
    assert!(path.join("clippy.toml").exists());
    drop(temp);
    assert!(!path.exists());
}

#[test]
fn dropping_a_value_whose_file_is_gone_removes_the_folder_without_panic() {
    let temp = TempDir::write("clippy", "clippy.toml", "x = 1\n").unwrap();
    std::fs::remove_file(temp.path.join("clippy.toml")).unwrap();
    let path = temp.path.clone();
    drop(temp);
    assert!(!path.exists());
}

#[test]
fn a_folder_holding_another_file_keeps_it() {
    let temp = TempDir::write("clippy", "clippy.toml", "x = 1\n").unwrap();
    let other = temp.path.join("other.txt");
    std::fs::write(&other, "keep").unwrap();
    let path = temp.path.clone();
    drop(temp);
    assert!(!path.join("clippy.toml").exists());
    assert_eq!(std::fs::read_to_string(&other).unwrap(), "keep");
    std::fs::remove_file(&other).unwrap();
    std::fs::remove_dir(&path).unwrap();
}

#[test]
fn a_file_the_value_did_not_create_is_kept_and_so_is_its_folder() {
    let folder = std::env::temp_dir().join(format!("cctao-zit-unowned-{}", std::process::id()));
    std::fs::create_dir_all(&folder).unwrap();
    let file = folder.join("clippy.toml");
    std::fs::write(&file, "keep").unwrap();
    drop(TempDir { path: folder.clone(), file: None });
    let kept = std::fs::read_to_string(&file);
    let folder_kept = folder.is_dir();
    std::fs::remove_dir_all(&folder).unwrap();
    assert_eq!(kept.ok().as_deref(), Some("keep"));
    assert!(folder_kept);
}
