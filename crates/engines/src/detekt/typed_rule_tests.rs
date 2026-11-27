//! Unit test for typed_rule.rs: rules that need type resolution (fixtures/commands.txt, "Type resolution probes") are
//! typed, others and set names are not; and each of the 67 entries of detekt_typed_rules.txt is a rule of
//! detekt_rule_list under the same set.

use super::{detekt_rule_list, typed_rule};

#[test]
fn typed_rule_cases() {
    let rows: [(&str, bool); 6] = [
        ("Deprecation", true),
        ("UnsafeCast", true),
        ("VarCouldBeVal", true),
        ("MagicNumber", false),
        ("potential-bugs", false),
        ("potential-bugs.Deprecation", false),
    ];
    for (rule, expected) in rows {
        assert_eq!(typed_rule(rule), expected, "{rule}");
    }
}

#[test]
fn every_typed_entry_is_a_listed_rule_of_its_set() {
    let typed: Vec<(&str, &str)> = include_str!("detekt_typed_rules.txt")
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| line.split_once('.').unwrap())
        .collect();
    assert_eq!(typed.len(), 67);
    let listed = detekt_rule_list();
    let missing: Vec<_> = typed.iter().filter(|pair| !listed.contains(pair)).collect();
    assert!(missing.is_empty(), "{missing:?}");
}
