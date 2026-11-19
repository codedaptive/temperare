//! Unit tests for fold_untracked.rs.

use crate::fold_untracked;
use std::collections::BTreeMap;
use std::ops::Range;
use std::path::PathBuf;

fn paths(entries: &[&str]) -> Vec<PathBuf> {
    entries.iter().map(PathBuf::from).collect()
}

#[test]
fn an_untracked_file_is_added_whole() {
    let mut lines = BTreeMap::new();
    let prefixes = fold_untracked(&mut lines, paths(&["b.rs"]));
    assert!(prefixes.is_empty());
    assert_eq!(lines, BTreeMap::from([(PathBuf::from("b.rs"), vec![1..u32::MAX])]));
}

#[test]
fn entries_ending_in_a_slash_are_returned_in_order_and_not_inserted() {
    let mut lines: BTreeMap<PathBuf, Vec<Range<u32>>> = BTreeMap::new();
    let prefixes = fold_untracked(&mut lines, paths(&["z/", "b.rs", "a/"]));
    let texts: Vec<_> = prefixes.iter().map(|p| p.to_str().unwrap().to_string()).collect();
    assert_eq!(texts, vec!["z/".to_string(), "a/".to_string()]);
    assert_eq!(lines.keys().collect::<Vec<_>>(), vec![&PathBuf::from("b.rs")]);
}

#[test]
fn existing_added_lines_are_kept() {
    let mut lines = BTreeMap::from([(PathBuf::from("a.rs"), vec![2..3])]);
    fold_untracked(&mut lines, paths(&["c/d.rs"]));
    assert_eq!(
        lines,
        BTreeMap::from([(PathBuf::from("a.rs"), vec![2..3]), (PathBuf::from("c/d.rs"), vec![1..u32::MAX])])
    );
}

#[test]
fn no_entries_change_nothing() {
    let mut lines = BTreeMap::from([(PathBuf::from("a.rs"), vec![2..3])]);
    assert!(fold_untracked(&mut lines, Vec::new()).is_empty());
    assert_eq!(lines, BTreeMap::from([(PathBuf::from("a.rs"), vec![2..3])]));
}
