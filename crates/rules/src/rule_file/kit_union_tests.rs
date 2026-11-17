//! ZUT for kit_union.rs.

use super::kit_union::kit_union;
use std::collections::BTreeMap;

fn kits() -> BTreeMap<String, Vec<String>> {
    BTreeMap::from([
        ("core".to_string(), vec!["crates/core/**".to_string()]),
        ("web".to_string(), vec!["web/**".to_string(), "site/**".to_string()]),
    ])
}

#[test]
fn no_kit_named_is_none() {
    let mut out = Vec::new();
    assert_eq!(kit_union("", &[], &kits(), &mut out), None);
    assert_eq!(out, vec![]);
}

#[test]
fn named_kits_give_the_union_of_their_paths() {
    let names = vec![("core".to_string(), 0), ("web".to_string(), 0)];
    let mut out = Vec::new();
    assert_eq!(
        kit_union("", &names, &kits(), &mut out),
        Some(vec!["crates/core/**".to_string(), "web/**".to_string(), "site/**".to_string()])
    );
    assert_eq!(out, vec![]);
}

#[test]
fn an_undefined_kit_is_a_problem_at_its_string() {
    let text = "kits = [\"core\", \"app\"]";
    let names = vec![("core".to_string(), 8), ("app".to_string(), 16)];
    let mut out = Vec::new();
    assert_eq!(kit_union(text, &names, &kits(), &mut out), Some(vec!["crates/core/**".to_string()]));
    assert_eq!(out.len(), 1);
    assert_eq!((out[0].line, out[0].col), (1, 17));
    assert!(out[0].what.contains("\"app\""), "{}", out[0].what);
}
