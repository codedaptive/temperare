//! Unit tests for missing_param_names.rs.

use super::missing_param_names::missing_param_names;
use crate::{Engine, Rule};

fn rule(required: &[&str]) -> Rule {
    let mut rule = Rule::new("tm-house-no-stored-bool", Engine::AstGrep, "c");
    rule.required_params = required.iter().map(|name| name.to_string()).collect();
    rule
}

/// A params table holding each of `names`.
fn table(names: &[&str]) -> toml::Table {
    names.iter().map(|name| (name.to_string(), toml::Value::Integer(1))).collect()
}

#[test]
fn names_the_required_params_not_given_in_required_order() {
    let mut defaulted = rule(&["entity_types"]);
    defaulted.params.insert("entity_types".to_string(), toml::Value::Array(vec![toml::Value::String("X".into())]));
    let rows: [(&str, Rule, Option<toml::Table>, &[&str]); 8] = [
        ("no required params, given None", rule(&[]), None, &[]),
        (
            "every required given",
            rule(&["entity_types", "limit"]),
            Some(table(&["limit", "entity_types", "other"])),
            &[],
        ),
        ("one missing", rule(&["entity_types"]), Some(table(&[])), &["entity_types"]),
        (
            "several missing in declared order",
            rule(&["zeta", "given", "alpha"]),
            Some(table(&["given"])),
            &["zeta", "alpha"],
        ),
        ("given None names every required", rule(&["a", "b"]), None, &["a", "b"]),
        ("given an empty table names every required", rule(&["a", "b"]), Some(table(&[])), &["a", "b"]),
        ("given a table holding a", rule(&["a", "b"]), Some(table(&["a"])), &["b"]),
        ("a default in params with given None is still named", defaulted, None, &["entity_types"]),
    ];
    let wrong: Vec<String> = rows
        .into_iter()
        .map(|(case, rule, given, expected)| (case, missing_param_names(&rule, given.as_ref()), expected))
        .filter(|(_, got, expected)| got != expected)
        .map(|(case, got, expected)| format!("{case}: got {got:?}, expected {expected:?}"))
        .collect();
    assert!(wrong.is_empty(), "rows wrong:\n{}", wrong.join("\n"));
}
