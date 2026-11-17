//! `read_rules`: the `[[rule]]` tables of the rule file.

use super::problem_at::problem_at;
use super::read_rule::read_rule;
use super::wrong_type::wrong_type;
use crate::{Kit, Problem, Rule};
use std::collections::BTreeSet;
use toml::de::DeTable;

/// The rules of the `[[rule]]` tables without problems, and the id of every `[[rule]]`, so an exception for a rule
/// that has problems is not also reported as naming no rule. A repeated id is a Problem at its second and later
/// occurrences.
pub(crate) fn read_rules(
    text: &str,
    root: &DeTable,
    kits: &[Kit],
    out: &mut Vec<Problem>,
) -> (Vec<Rule>, BTreeSet<String>) {
    let mut ids = BTreeSet::new();
    let mut rules = Vec::new();
    let Some(value) = root.get("rule") else {
        return (rules, ids);
    };
    let Some(tables) = value.get_ref().as_array() else {
        out.push(wrong_type(text, "rule", value, "[[rule]] tables"));
        return (rules, ids);
    };
    // One plain parse of the whole text gives every rule's `params` as a toml::Table directly, which is simpler than
    // deserializing each spanned table.
    let doc: toml::Table = toml::from_str(text).unwrap_or_default();
    for (i, item) in tables.iter().enumerate() {
        let Some(table) = item.get_ref().as_table() else {
            out.push(wrong_type(text, "rule", item, "a [[rule]] table"));
            continue;
        };
        let duplicate = is_duplicate(text, table, &mut ids, out);
        let rule = read_rule(text, item.span().start, table, kits, params(&doc, i), out);
        rules.extend(rule.filter(|_| !duplicate));
    }
    (rules, ids)
}

/// Records the table's `id` in `ids`; true, with a Problem at the value, when it was there already.
fn is_duplicate(text: &str, table: &DeTable, ids: &mut BTreeSet<String>, out: &mut Vec<Problem>) -> bool {
    let Some(value) = table.get("id") else {
        return false;
    };
    let Some(id) = value.get_ref().as_str() else {
        return false;
    };
    if ids.insert(id.to_string()) {
        return false;
    }
    out.push(problem_at(
        text,
        value.span().start,
        format!("duplicate id \"{id}\": another [[rule]] has it already"),
        "Give each [[rule]] its own id.",
    ));
    true
}

/// The `params` table of the `i`th `[[rule]]` in `doc`, empty when absent.
fn params(doc: &toml::Table, i: usize) -> toml::Table {
    doc.get("rule")
        .and_then(|rules| rules.get(i))
        .and_then(|rule| rule.get("params"))
        .and_then(|params| params.as_table())
        .cloned()
        .unwrap_or_default()
}
