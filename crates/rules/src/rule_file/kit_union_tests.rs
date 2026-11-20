//! Unit tests for kit_union.rs.

use super::kit_union::kit_union;
use crate::Kit;

fn kit(name: &str, paths: &[&str]) -> Kit {
    Kit { name: name.to_string(), paths: paths.iter().map(|p| p.to_string()).collect(), owner: None, scope: None }
}

/// Two kits, "web" first, so a lookup by name cannot rely on their order.
fn kits() -> Vec<Kit> {
    vec![kit("web", &["web/**", "site/**"]), kit("core", &["crates/core/**"])]
}

#[test]
fn no_kit_named_is_none() {
    let mut out = Vec::new();
    assert_eq!(kit_union("", &[], &kits(), "kits", &mut out), None);
    assert_eq!(out, vec![]);
}

#[test]
fn named_kits_give_the_union_of_their_paths() {
    let names = vec![("core".to_string(), 0), ("web".to_string(), 0)];
    let mut out = Vec::new();
    assert_eq!(
        kit_union("", &names, &kits(), "kits", &mut out),
        Some(vec!["crates/core/**".to_string(), "web/**".to_string(), "site/**".to_string()])
    );
    assert_eq!(out, vec![]);
}

#[test]
fn an_undefined_kit_is_a_problem_at_its_string() {
    let text = "kits = [\"core\", \"app\"]";
    let names = vec![("core".to_string(), 8), ("app".to_string(), 16)];
    let mut out = Vec::new();
    assert_eq!(kit_union(text, &names, &kits(), "kits", &mut out), Some(vec!["crates/core/**".to_string()]));
    assert_eq!(out.len(), 1);
    assert_eq!((out[0].line, out[0].col), (1, 17));
    assert!(out[0].what.contains("\"app\""), "{}", out[0].what);
    assert!(out[0].fix.contains("from `kits`"), "{}", out[0].fix);
}

#[test]
fn an_undefined_kit_fix_names_the_key_it_came_from() {
    let text = "outside_kits = [\"app\"]";
    let names = vec![("app".to_string(), 16)];
    let mut out = Vec::new();
    assert_eq!(kit_union(text, &names, &kits(), "outside_kits", &mut out), Some(vec![]));
    assert_eq!(out.len(), 1);
    assert_eq!((out[0].line, out[0].col), (1, 17));
    assert!(out[0].fix.contains("`outside_kits`"), "{}", out[0].fix);
    assert!(!out[0].fix.contains("from `kits`"), "{}", out[0].fix);
}
