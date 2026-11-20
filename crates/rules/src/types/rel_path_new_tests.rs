//! Unit tests for rel_path_new.rs: '\\' becomes '/', every leading "./" goes, nothing else changes.

use super::RelPath;

#[test]
fn backslashes_and_leading_dot_slash_are_normalised() {
    assert_eq!(RelPath::new(".\\a\\b").as_str(), "a/b");
    assert_eq!(RelPath::new("./a\\b").as_str(), "a/b");
}

#[test]
fn every_leading_dot_slash_is_removed() {
    assert_eq!(RelPath::new("././src/x.rs").as_str(), "src/x.rs");
}

#[test]
fn a_clean_path_is_unchanged() {
    assert_eq!(RelPath::new("crates/core/src/a.rs").as_str(), "crates/core/src/a.rs");
}

#[test]
fn inner_dot_segments_and_dotfiles_are_kept() {
    assert_eq!(RelPath::new("a/./b").as_str(), "a/./b");
    assert_eq!(RelPath::new(".github/x.yml").as_str(), ".github/x.yml");
}

#[test]
fn both_spellings_compare_equal() {
    assert_eq!(RelPath::new("./src\\a.rs"), RelPath::new("src/a.rs"));
}
