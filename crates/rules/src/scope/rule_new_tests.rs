//! ZUT for rule_new.rs: the rule file's defaults, and a scope covering every file.

use crate::{Engine, Kind, RelPath, Rule, Severity};

#[test]
fn the_given_fields_come_out_as_given() {
    let r = Rule::new("no-unwrap", Engine::Clippy, "clippy::unwrap_used");
    assert_eq!(r.id, "no-unwrap");
    assert_eq!(r.engine, Engine::Clippy);
    assert_eq!(r.check, "clippy::unwrap_used");
}

#[test]
fn the_rest_take_the_rule_files_defaults() {
    let r = Rule::new("no-unwrap", Engine::Ruff, "E501");
    assert_eq!(r.kind, Kind::Quality);
    assert_eq!(r.severity, Severity::Error);
    assert_eq!(r.config, None);
    assert!(r.paths.is_empty());
    assert!(r.exclude.is_empty());
    assert!(r.kits.is_empty());
    assert!(r.outside_kits.is_empty());
    assert_eq!(r.fix, "");
    assert!(r.params.is_empty());
}

#[test]
fn the_baseline_fields_take_their_empty_values() {
    let r = Rule::new("no-unwrap", Engine::Clippy, "clippy::unwrap_used");
    assert_eq!(r.enforces, None);
    assert_eq!(r.leaves, None);
    assert!(!r.only_with_base);
    assert_eq!(r.required_params, Vec::<String>::new());
    assert_eq!(r.not_applicable, None);
    assert_eq!(r.embedded_yaml(), None);
}

#[test]
fn the_rule_applies_to_any_path() {
    let r = Rule::new("no-unwrap", Engine::Clippy, "clippy::unwrap_used");
    for file in ["a.rs", "src/lib.rs", "crates/core/tests/x/y.txt", ".github/workflows/ci.yml"] {
        assert!(r.applies(&RelPath::new(file)), "{file}");
    }
}
