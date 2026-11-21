//! Unit tests for rule_file_new.rs: the folder kept, everything else empty.

use crate::RuleFile;
use std::path::PathBuf;

#[test]
fn the_folder_is_kept_and_the_rest_is_empty() {
    let file = RuleFile::new(PathBuf::from("repo/sub"));
    assert_eq!(file.folder, PathBuf::from("repo/sub"));
    assert!(file.kits.is_empty());
    assert_eq!(file.owner, None);
    assert!(file.rules.is_empty());
    assert!(file.exceptions.is_empty());
    assert_eq!(file.baseline, Vec::<String>::new());
    assert_eq!(file.retired, Vec::<String>::new());
    assert_eq!(file.not_applicable_rules.len(), 0);
}
