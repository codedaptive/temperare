//! ZIT for temp_dir_write.rs (and the plain struct in temp_dir.rs) on real temp folders: the folder's place and
//! name, the one file in it and its contents, two folders for two writes, a file that cannot be created, and a name
//! that is not a plain file name refused before anything is made.

use crate::entry::TempDir;
use std::io::ErrorKind;

#[test]
fn the_folder_holds_one_file_with_the_text() {
    let temp = TempDir::write("clippy", "clippy.toml", "cognitive-complexity-threshold = 12\n").unwrap();
    let text = std::fs::read_to_string(temp.path.join("clippy.toml")).unwrap();
    assert_eq!(text, "cognitive-complexity-threshold = 12\n");
    let names: Vec<_> = std::fs::read_dir(&temp.path).unwrap().map(|entry| entry.unwrap().file_name()).collect();
    assert_eq!(names, ["clippy.toml"]);
}

#[test]
fn the_folder_is_named_for_the_stem_and_the_process_under_the_temp_folder() {
    let temp = TempDir::write("clippy", "clippy.toml", "").unwrap();
    assert!(temp.path.is_dir());
    assert_eq!(temp.path.parent(), Some(std::env::temp_dir().as_path()));
    let name = temp.path.file_name().unwrap().to_str().unwrap();
    let prefix = format!("cctao-clippy-{}-", std::process::id());
    let n = name.strip_prefix(&prefix).unwrap_or_else(|| panic!("{name}"));
    assert!(n.parse::<u64>().is_ok(), "{name}");
}

#[test]
fn two_folders_written_by_one_process_get_two_folders() {
    let first = TempDir::write("clippy", "clippy.toml", "one").unwrap();
    let second = TempDir::write("clippy", "clippy.toml", "two").unwrap();
    assert_ne!(first.path, second.path);
    assert_eq!(std::fs::read_to_string(first.path.join("clippy.toml")).unwrap(), "one");
    assert_eq!(std::fs::read_to_string(second.path.join("clippy.toml")).unwrap(), "two");
}

#[test]
fn a_file_that_cannot_be_written_is_an_error_and_leaves_no_folder() {
    let before = cctao_folders("cctao-unwritable");
    // A plain name over the 255-byte name limit of common file systems: the folder is made, the file cannot be.
    assert!(TempDir::write("cctao-unwritable", &"n".repeat(300), "x").is_err());
    assert_eq!(cctao_folders("cctao-unwritable"), before);
}

#[test]
fn an_absolute_name_is_refused_and_the_file_there_is_kept() {
    let own = std::env::temp_dir().join(format!("cctao-zit-absolute-{}", std::process::id()));
    std::fs::create_dir_all(&own).unwrap();
    let existing = own.join("clippy.toml");
    std::fs::write(&existing, "keep me").unwrap();
    let got = TempDir::write("cctao-absolute", existing.to_str().unwrap(), "x");
    let kept = std::fs::read_to_string(&existing);
    std::fs::remove_dir_all(&own).unwrap();
    assert_eq!(kept.ok().as_deref(), Some("keep me"));
    assert_eq!(got.map(|_| ()).map_err(|err| err.kind()), Err(ErrorKind::InvalidInput));
}

#[test]
fn a_name_reaching_up_out_of_the_folder_is_refused_and_nothing_is_made() {
    let name = format!("cctao-zit-escape-{}", std::process::id());
    let before = cctao_folders("cctao-escape");
    let got = TempDir::write("cctao-escape", &format!("../{name}"), "x");
    let escaped = std::env::temp_dir().join(&name).exists();
    let after = cctao_folders("cctao-escape");
    assert_eq!(got.map(|_| ()).map_err(|err| err.kind()), Err(ErrorKind::InvalidInput));
    assert!(!escaped, "{name} was made in the temp folder");
    assert_eq!(after, before);
}

#[test]
fn a_dot_an_empty_name_and_a_name_with_a_folder_are_refused_and_nothing_is_made() {
    for file in [".", "", "a/b"] {
        let before = cctao_folders("cctao-refused");
        let got = TempDir::write("cctao-refused", file, "x");
        let after = cctao_folders("cctao-refused");
        assert_eq!(got.map(|_| ()).map_err(|err| err.kind()), Err(ErrorKind::InvalidInput), "{file:?}");
        assert_eq!(after, before, "{file:?}");
    }
}

/// The folders under the temp folder whose names `stem` gives in this process.
fn cctao_folders(stem: &str) -> Vec<std::ffi::OsString> {
    let prefix = format!("cctao-{stem}-{}-", std::process::id());
    let mut names: Vec<_> = std::fs::read_dir(std::env::temp_dir())
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .filter(|name| name.to_string_lossy().starts_with(&prefix))
        .collect();
    names.sort();
    names
}
