//! Unit tests for keep_added.rs.

use crate::keep_added;
use rules::{Engine, Finding, Kind, RelPath, Severity};
use std::collections::BTreeMap;
use std::ops::Range;
use std::path::{Path, PathBuf};

fn finding(path: &str, line: u32) -> Finding {
    Finding {
        rule: "no-unwrap".to_string(),
        engine: Engine::Clippy,
        kind: Kind::Quality,
        severity: Severity::Error,
        owner: None,
        path: RelPath::new(path),
        line,
        col: None,
        what: "used unwrap".to_string(),
        fix: "Return a typed error.".to_string(),
    }
}

fn added(entries: &[(&str, Vec<Range<u32>>)]) -> BTreeMap<PathBuf, Vec<Range<u32>>> {
    entries.iter().map(|(path, ranges)| (PathBuf::from(path), ranges.clone())).collect()
}

fn kept(findings: Vec<Finding>, added: &BTreeMap<PathBuf, Vec<Range<u32>>>, folder: &str) -> Vec<(String, u32)> {
    kept_under(findings, added, &[], folder)
}

fn kept_under(
    findings: Vec<Finding>,
    added: &BTreeMap<PathBuf, Vec<Range<u32>>>,
    under: &[&str],
    folder: &str,
) -> Vec<(String, u32)> {
    let under: Vec<PathBuf> = under.iter().map(PathBuf::from).collect();
    keep_added(findings, added, &under, Path::new(folder))
        .into_iter()
        .map(|f| (f.path.as_str().to_string(), f.line))
        .collect()
}

fn at(path: &str, line: u32) -> (String, u32) {
    (path.to_string(), line)
}

#[test]
fn a_line_inside_a_range_is_kept() {
    let map = added(&[("src/a.rs", vec![10..20])]);
    assert_eq!(kept(vec![finding("src/a.rs", 15)], &map, ""), vec![at("src/a.rs", 15)]);
}

#[test]
fn the_range_start_is_kept_and_the_half_open_end_is_dropped() {
    let map = added(&[("src/a.rs", vec![10..20])]);
    assert_eq!(kept(vec![finding("src/a.rs", 10), finding("src/a.rs", 20)], &map, ""), vec![at("src/a.rs", 10)]);
}

#[test]
fn a_line_in_the_second_range_is_kept() {
    let map = added(&[("src/a.rs", vec![1..3, 40..42])]);
    assert_eq!(kept(vec![finding("src/a.rs", 41), finding("src/a.rs", 5)], &map, ""), vec![at("src/a.rs", 41)]);
}

#[test]
fn a_path_absent_from_the_map_is_dropped() {
    let map = added(&[("src/a.rs", vec![1..100])]);
    assert!(kept(vec![finding("src/b.rs", 5)], &map, "").is_empty());
}

#[test]
fn a_path_with_no_ranges_is_dropped() {
    let map = added(&[("src/a.rs", Vec::new())]);
    assert!(kept(vec![finding("src/a.rs", 1)], &map, "").is_empty());
}

#[test]
fn a_folder_re_roots_the_finding_path() {
    let nested = added(&[("crates/x/src/a.rs", vec![1..10])]);
    assert_eq!(kept(vec![finding("src/a.rs", 4)], &nested, "crates/x"), vec![at("src/a.rs", 4)]);
    let top_only = added(&[("src/a.rs", vec![1..10])]);
    assert!(kept(vec![finding("src/a.rs", 4)], &top_only, "crates/x").is_empty());
}

#[test]
fn an_empty_folder_is_the_repository_top() {
    let map = added(&[("src/a.rs", vec![1..10])]);
    assert_eq!(kept(vec![finding("src/a.rs", 4)], &map, ""), vec![at("src/a.rs", 4)]);
}

#[test]
fn a_dot_folder_is_the_repository_top() {
    let map = added(&[("src/a.rs", vec![1..10])]);
    assert_eq!(kept(vec![finding("src/a.rs", 4)], &map, "."), vec![at("src/a.rs", 4)]);
}

#[test]
fn a_range_over_a_whole_new_file_keeps_every_line() {
    let map = added(&[("src/new.rs", vec![1..4])]);
    let findings = vec![finding("src/new.rs", 1), finding("src/new.rs", 2), finding("src/new.rs", 3)];
    assert_eq!(kept(findings, &map, ""), vec![at("src/new.rs", 1), at("src/new.rs", 2), at("src/new.rs", 3)]);
    let open = added(&[("src/new.rs", vec![1..u32::MAX])]);
    let far = vec![finding("src/new.rs", 1), finding("src/new.rs", 90_000)];
    assert_eq!(kept(far, &open, ""), vec![at("src/new.rs", 1), at("src/new.rs", 90_000)]);
}

#[test]
fn kept_findings_keep_their_order() {
    let map = added(&[("src/a.rs", vec![1..10]), ("src/b.rs", vec![1..10])]);
    let findings = vec![
        finding("src/b.rs", 9),
        finding("src/a.rs", 50),
        finding("src/a.rs", 2),
        finding("src/c.rs", 1),
        finding("src/b.rs", 1),
    ];
    assert_eq!(kept(findings, &map, ""), vec![at("src/b.rs", 9), at("src/a.rs", 2), at("src/b.rs", 1)]);
}

#[test]
fn a_finding_under_a_prefix_entry_is_kept_at_any_line() {
    let none = added(&[]);
    let findings = vec![finding("nested/c.rs", 1), finding("nested/c.rs", 90_000)];
    assert_eq!(kept_under(findings, &none, &["nested/"], ""), vec![at("nested/c.rs", 1), at("nested/c.rs", 90_000)]);
}

#[test]
fn a_deep_path_under_a_prefix_entry_is_kept() {
    let none = added(&[]);
    assert_eq!(kept_under(vec![finding("nested/deep/d.rs", 3)], &none, &["nested/"], ""), vec![at("nested/deep/d.rs", 3)]);
}

#[test]
fn a_sibling_name_sharing_the_prefix_text_is_dropped() {
    let none = added(&[]);
    assert!(kept_under(vec![finding("nestedx/c.rs", 3)], &none, &["nested/"], "").is_empty());
}

#[test]
fn a_prefix_entry_matches_the_re_rooted_path() {
    let none = added(&[]);
    assert_eq!(kept_under(vec![finding("c.rs", 5)], &none, &["sub/nested/"], "sub/nested"), vec![at("c.rs", 5)]);
    assert_eq!(kept_under(vec![finding("nested/c.rs", 5)], &none, &["sub/nested/"], "sub"), vec![at("nested/c.rs", 5)]);
    assert!(kept_under(vec![finding("nested/c.rs", 5)], &none, &["sub/nested/"], "").is_empty());
}

#[test]
fn prefix_and_range_matches_keep_input_order() {
    let map = added(&[("src/a.rs", vec![1..10])]);
    let findings = vec![finding("nested/c.rs", 40), finding("src/a.rs", 50), finding("src/a.rs", 2)];
    assert_eq!(kept_under(findings, &map, &["nested/"], ""), vec![at("nested/c.rs", 40), at("src/a.rs", 2)]);
}
