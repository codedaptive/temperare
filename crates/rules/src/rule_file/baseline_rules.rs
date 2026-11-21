//! `baseline_rules`: the rules of one baseline set built into `tm`, whether or not it is ready.

use super::baseline_set::BaselineSet;
use super::embedded_sets::EMBEDDED_SETS;
use super::rule_file_parse::parse_with_sets;
use crate::{Problem, Rule};
use std::path::Path;

/// The rules of the set `set` ("name@version") built into `tm`, as a rule file in "." taking only that set reads them:
/// in the resolved order of `RuleFile::rules_in_order`, each not-applicable rule in its place with its reason. Every
/// set counts as ready here, so `tm baseline test` can prove a set's rules before a rule file may take it (cli does
/// not call this yet). Err holds the Problems of the entry (malformed, unknown set or version) or of the set itself.
pub fn baseline_rules(set: &str) -> Result<Vec<Rule>, Vec<Problem>> {
    baseline_rules_in(set, EMBEDDED_SETS)
}

/// `baseline_rules` with `sets` in place of the sets built into `tm`.
pub(crate) fn baseline_rules_in(set: &str, sets: &[BaselineSet]) -> Result<Vec<Rule>, Vec<Problem>> {
    let ready: Vec<BaselineSet> = sets.iter().map(|known| BaselineSet { ready: true, ..*known }).collect();
    // The toml crate quotes the text, so any `set` is one entry string and the parse judges it.
    let text = format!("version = 1\nbaseline = [{}]\n", toml::Value::String(set.to_string()));
    let file = parse_with_sets(&text, Path::new("."), &ready)?;
    Ok(file.rules_in_order().into_iter().cloned().collect())
}
