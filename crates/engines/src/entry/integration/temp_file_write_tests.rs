//! Integration tests for temp_file_write.rs (and the plain struct in temp_file.rs) on real temp files: the file's place, name and
//! contents.

use crate::entry::TempFile;

#[test]
fn the_written_file_holds_the_text() {
    let temp = TempFile::write("ast-grep", "yml", "id: a\nlanguage: JavaScript\n").unwrap();
    assert_eq!(std::fs::read_to_string(&temp.path).unwrap(), "id: a\nlanguage: JavaScript\n");
}

#[test]
fn the_file_is_named_for_the_stem_the_process_and_the_extension_under_the_temp_folder() {
    let temp = TempFile::write("ast-grep", "yml", "").unwrap();
    assert_eq!(temp.path.parent(), Some(std::env::temp_dir().as_path()));
    let name = temp.path.file_name().unwrap().to_str().unwrap();
    let prefix = format!("cctao-ast-grep-{}-", std::process::id());
    assert!(name.starts_with(&prefix) && name.ends_with(".yml"), "{name}");
    let other = TempFile::write("ruff", "toml", "").unwrap();
    let name = other.path.file_name().unwrap().to_str().unwrap();
    let prefix = format!("cctao-ruff-{}-", std::process::id());
    assert!(name.starts_with(&prefix) && name.ends_with(".toml"), "{name}");
}

#[test]
fn two_files_written_by_one_process_get_two_files() {
    let first = TempFile::write("ast-grep", "yml", "one").unwrap();
    let second = TempFile::write("ast-grep", "yml", "two").unwrap();
    assert_ne!(first.path, second.path);
    assert_eq!(std::fs::read_to_string(&first.path).unwrap(), "one");
    assert_eq!(std::fs::read_to_string(&second.path).unwrap(), "two");
}
