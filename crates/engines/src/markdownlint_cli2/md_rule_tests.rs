//! Unit test for md_rule.rs: plain data; equality covers both fields, the order of names included.

use super::MdRule;

fn rule(names: &'static [&'static str], tags: &'static [&'static str]) -> MdRule {
    MdRule { names, tags }
}

#[test]
fn md_rules_compare_by_every_field() {
    assert_eq!(rule(&["MD013", "line-length"], &["line_length"]), rule(&["MD013", "line-length"], &["line_length"]));
    assert_ne!(rule(&["MD013", "line-length"], &["line_length"]), rule(&["MD013"], &["line_length"]));
    assert_ne!(rule(&["MD013", "line-length"], &["line_length"]), rule(&["line-length", "MD013"], &["line_length"]));
    assert_ne!(rule(&["MD013", "line-length"], &["line_length"]), rule(&["MD013", "line-length"], &["headings"]));
}
