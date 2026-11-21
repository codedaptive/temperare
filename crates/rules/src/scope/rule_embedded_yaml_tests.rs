//! Unit tests for rule_embedded_yaml.rs.

use crate::{Engine, Rule};

#[test]
fn a_rule_carrying_yaml_gives_that_exact_text() {
    let yaml = "id: no-unwrap\nlanguage: rust\nrule:\n  pattern: $X.unwrap()\n";
    let rule = Rule { embedded_text: Some(yaml), ..Rule::new("tm-no-unwrap", Engine::AstGrep, "no-unwrap.yml") };
    assert_eq!(rule.embedded_yaml(), Some("id: no-unwrap\nlanguage: rust\nrule:\n  pattern: $X.unwrap()\n"));
}

#[test]
fn a_rule_carrying_none_gives_none() {
    let rule = Rule::new("own", Engine::AstGrep, "rules/own.yml");
    assert_eq!(rule.embedded_yaml(), None);
}
