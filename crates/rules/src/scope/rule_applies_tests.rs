//! ZUT for rule_applies.rs.

use crate::{Engine, Kind, RelPath, Rule, Scope, Severity};

fn rule(paths: &[&str], exclude: &[&str], kit_paths: Option<&[&str]>) -> Rule {
    let paths: Vec<String> = paths.iter().map(|g| g.to_string()).collect();
    let exclude: Vec<String> = exclude.iter().map(|g| g.to_string()).collect();
    let kit_paths: Option<Vec<String>> = kit_paths.map(|k| k.iter().map(|g| g.to_string()).collect());
    let scope = Scope::build(&paths, &exclude, kit_paths.as_deref()).unwrap();
    Rule {
        id: "no-unwrap".to_string(),
        engine: Engine::Clippy,
        kind: Kind::Quality,
        check: "clippy::unwrap_used".to_string(),
        config: None,
        paths,
        exclude,
        kits: Vec::new(),
        outside_kits: Vec::new(),
        severity: Severity::Error,
        fix: "Return a typed error.".to_string(),
        params: toml::Table::new(),
        scope,
    }
}

fn applies(rule: &Rule, file: &str) -> bool {
    rule.applies(&RelPath::new(file))
}

#[test]
fn double_star_glob_covers_a_root_file() {
    let r = rule(&["**/*.rs"], &[], None);
    assert!(applies(&r, "a.rs"));
    assert!(applies(&r, "x/y/a.rs"));
}

#[test]
fn single_star_does_not_cross_a_separator() {
    let r = rule(&["src/*.rs"], &[], None);
    assert!(applies(&r, "src/a.rs"));
    assert!(!applies(&r, "src/a/b.rs"));
}

#[test]
fn exclude_wins_over_include() {
    let r = rule(&["**/*.rs"], &["tests/**"], None);
    assert!(applies(&r, "src/a.rs"));
    assert!(!applies(&r, "tests/a.rs"));
}

#[test]
fn kits_limit_the_rule() {
    let r = rule(&["**/*.rs"], &[], Some(&["crates/core/**"]));
    assert!(applies(&r, "crates/core/lib.rs"));
    assert!(!applies(&r, "crates/cli/main.rs"));
}

#[test]
fn empty_paths_cover_every_file() {
    let r = rule(&[], &[], None);
    assert!(applies(&r, "anything/at/all.txt"));
}
