//! Zone integration tests for rule_folder.rs: a relative and an absolute folder, both canonical, on a temp tree the test makes and
//! removes; a folder that does not exist is an error.

use crate::rule_folder::rule_folder;
use rules::RuleFile;
use std::fs;
use std::path::{Path, PathBuf};

/// A fresh, canonical temp folder for one test.
fn tree(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("engines-zit-{}-rule-folder-{name}", std::process::id()));
    fs::create_dir(&root).unwrap();
    fs::canonicalize(root).unwrap()
}

fn file_in(folder: &Path) -> RuleFile {
    RuleFile::parse("version = 1\n", folder).unwrap()
}

#[test]
fn a_relative_folder_is_joined_to_the_repo_and_made_canonical() {
    let root = tree("relative");
    fs::create_dir_all(root.join("sub/py")).unwrap();
    let got = rule_folder(&file_in(Path::new("sub/./py/../py")), &root).unwrap();
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, root.join("sub/py"));
}

#[test]
fn an_absolute_folder_ignores_the_repo() {
    let root = tree("absolute");
    fs::create_dir_all(root.join("rules")).unwrap();
    let got = rule_folder(&file_in(&root.join("rules")), Path::new("/no/such/repo")).unwrap();
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got, root.join("rules"));
}

#[test]
fn a_folder_that_does_not_exist_is_an_error() {
    let root = tree("missing");
    let got = rule_folder(&file_in(Path::new("gone")), &root);
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(got.unwrap_err().kind(), std::io::ErrorKind::NotFound);
}
