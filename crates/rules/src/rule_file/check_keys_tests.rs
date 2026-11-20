//! Unit tests for check_keys.rs.

use super::check_keys::check_keys;
use toml::de::DeTable;

#[test]
fn each_key_not_allowed_is_a_problem_at_the_key() {
    let text = "a = 1\nzz = 2\n  b = 3\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    check_keys(text, doc.get_ref(), &["a"], "[[rule]]", &mut out);
    let found: Vec<(usize, usize)> = out.iter().map(|p| (p.line, p.col)).collect();
    // Keys iterate in name order: b (3:3), then zz (2:1).
    assert_eq!(found, vec![(3, 3), (2, 1)]);
    assert_eq!(out[0].what, "unknown key `b` in [[rule]]");
    assert_eq!(out[1].what, "unknown key `zz` in [[rule]]");
    assert!(out[0].fix.contains('a'), "{}", out[0].fix);
}

#[test]
fn allowed_keys_are_no_problem() {
    let text = "a = 1\nb = 2\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    check_keys(text, doc.get_ref(), &["a", "b"], "the top level", &mut out);
    assert_eq!(out, vec![]);
}
