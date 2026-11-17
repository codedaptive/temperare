//! ZUT for scope_matches.rs.

use crate::{RelPath, Scope};

fn globs(list: &[&str]) -> Vec<String> {
    list.iter().map(|g| g.to_string()).collect()
}

fn scope(paths: &[&str], exclude: &[&str], kits: Option<&[&str]>) -> Scope {
    let kit_paths = kits.map(globs);
    Scope::build(&globs(paths), &globs(exclude), kit_paths.as_deref()).unwrap()
}

fn matches(scope: &Scope, file: &str) -> bool {
    scope.matches(&RelPath::new(file))
}

#[test]
fn double_star_matches_root_and_nested_files() {
    let s = scope(&["**/*.rs"], &[], None);
    assert!(matches(&s, "a.rs"));
    assert!(matches(&s, "x/y/a.rs"));
    assert!(!matches(&s, "x/y/a.py"));
}

#[test]
fn single_star_does_not_cross_a_separator() {
    let s = scope(&["src/*.rs"], &[], None);
    assert!(matches(&s, "src/a.rs"));
    assert!(!matches(&s, "src/a/b.rs"));
}

#[test]
fn exclude_beats_include() {
    let s = scope(&["src/**"], &["src/gen/**"], None);
    assert!(matches(&s, "src/lib.rs"));
    assert!(!matches(&s, "src/gen/out.rs"));
}

#[test]
fn kits_limit_the_paths() {
    let s = scope(&["**/*.rs"], &[], Some(&["crates/a/**", "crates/b/**"]));
    assert!(matches(&s, "crates/a/lib.rs"));
    assert!(matches(&s, "crates/b/src/x.rs"));
    assert!(!matches(&s, "crates/c/lib.rs"));
}

#[test]
fn kits_alone_are_the_scope_when_paths_are_empty() {
    let s = scope(&[], &[], Some(&["kit/**"]));
    assert!(matches(&s, "kit/README"));
    assert!(!matches(&s, "other/README"));
}

#[test]
fn empty_paths_cover_every_file() {
    let s = scope(&[], &[], None);
    assert!(matches(&s, "a.rs"));
    assert!(matches(&s, "deep/down/b.swift"));
}

#[test]
fn exclude_applies_when_paths_are_empty() {
    let s = scope(&[], &["vendor/**"], None);
    assert!(matches(&s, "src/a.rs"));
    assert!(!matches(&s, "vendor/x/a.rs"));
}

#[test]
fn leading_dot_slash_in_the_file_is_ignored() {
    let s = scope(&["src/*.rs"], &[], None);
    assert!(matches(&s, "./src/a.rs"));
}
