//! Integration tests for temp_file_drop.rs on real temp files: dropping the value removes its file, and a file already gone is no
//! panic.

use crate::entry::TempFile;

#[test]
fn dropping_the_value_removes_its_file() {
    let temp = TempFile::write("ast-grep", "yml", "id: a\n").unwrap();
    let path = temp.path.clone();
    assert!(path.exists());
    drop(temp);
    assert!(!path.exists());
}

#[test]
fn dropping_a_value_whose_file_is_gone_does_not_panic() {
    let temp = TempFile::write("ast-grep", "yml", "id: a\n").unwrap();
    std::fs::remove_file(&temp.path).unwrap();
    let path = temp.path.clone();
    drop(temp);
    assert!(!path.exists());
}
