//! Unit tests for rule_file.rs: `folder` is the folder the file was parsed with, so paths in the file resolve against it.

use crate::RuleFile;
use std::path::{Path, PathBuf};

#[test]
fn folder_is_kept_and_resolves_a_rules_check_file() {
    let text = "version = 1\n[[rule]]\nid = \"r\"\nengine = \"ast-grep\"\ncheck = \"rules/r.yml\"\nfix = \"f\"\n";
    let file = RuleFile::parse(text, Path::new("some/repo")).expect("valid");
    assert_eq!(file.folder, PathBuf::from("some/repo"));
    assert_eq!(file.folder.join(&file.rules[0].check), PathBuf::from("some/repo/rules/r.yml"));
}
