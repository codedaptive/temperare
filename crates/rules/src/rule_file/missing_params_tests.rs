//! ZUT for missing_params.rs.

use super::missing_params::missing_params;
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

const ENTITY_TYPES_REASON: &str =
    "required params not given: entity_types; give them in the params of an [[override]] for \"tm-house-no-stored-bool\"";

#[test]
fn no_required_params_is_none() {
    assert_eq!(missing_params(&rule(&[]), None), None);
    assert_eq!(missing_params(&rule(&[]), Some(&table(&["x"]))), None);
}

#[test]
fn every_required_param_given_is_none() {
    let given = table(&["limit", "entity_types", "other"]);
    assert_eq!(missing_params(&rule(&["entity_types", "limit"]), Some(&given)), None);
}

#[test]
fn one_missing_param_is_named_with_the_rule() {
    assert_eq!(missing_params(&rule(&["entity_types"]), Some(&table(&[]))).as_deref(), Some(ENTITY_TYPES_REASON));
}

#[test]
fn only_the_missing_ones_are_named_in_required_order() {
    assert_eq!(
        missing_params(&rule(&["zeta", "given", "alpha"]), Some(&table(&["given"]))).as_deref(),
        Some(
            "required params not given: zeta, alpha; give them in the params of an [[override]] for \
             \"tm-house-no-stored-bool\""
        ),
    );
}

#[test]
fn a_default_in_the_rules_own_params_is_not_given() {
    let mut defaulted = rule(&["entity_types"]);
    defaulted.params.insert("entity_types".to_string(), toml::Value::Array(vec![toml::Value::String("X".into())]));
    assert_eq!(missing_params(&defaulted, None).as_deref(), Some(ENTITY_TYPES_REASON));
}

#[test]
fn the_override_giving_it_over_a_default_is_none() {
    let mut defaulted = rule(&["entity_types"]);
    defaulted.params.insert("entity_types".to_string(), toml::Value::Array(vec![toml::Value::String("X".into())]));
    assert_eq!(missing_params(&defaulted, Some(&table(&["entity_types"]))), None);
}
