//! `read_set_cases`: the failing and passing cases of every rule of a baseline set.

use super::baseline_set::BaselineSet;
use super::param_type::ParamType;
use super::problem_at::problem_at;
use super::read_case::read_case;
use super::read_required_params::read_required_params;
use crate::{CaseFile, ExpectedFinding, Problem, RuleCases};
use toml::de::DeTable;

/// Each `[[rule]]` of `root` (the set's own `text`) whose `id` is a string and whose cases have no Problem: that id,
/// its cases, and the params its failing and its passing case give (each empty when the case gives none), in file
/// order. A rule with no `failing` or no `passing` case is a Problem at its `[[rule]]` header, and read_case reports
/// each problem in a case, its params checked against the rule's typed required params; every Problem goes to `out`.
/// A `rule` that is not an array, an entry that is not a table, or an `id` that is absent or not a string gives nothing
/// here, and a fault in `required_params` is not reported here: read_rules reports those.
pub(crate) fn read_set_cases(
    text: &str,
    root: &DeTable,
    set: &BaselineSet,
    out: &mut Vec<Problem>,
) -> Vec<(String, RuleCases, toml::Table, toml::Table)> {
    let Some(tables) = root.get("rule").and_then(|value| value.get_ref().as_array()) else {
        return Vec::new();
    };
    // One plain parse of the whole text gives each case's `params` as a toml::Table directly, as read_rules does.
    let doc: toml::Table = toml::from_str(text).unwrap_or_default();
    let mut found = Vec::new();
    for (i, item) in tables.iter().enumerate() {
        let Some(table) = item.get_ref().as_table() else {
            continue;
        };
        let Some(id) = table.get("id").and_then(|id| id.get_ref().as_str()) else {
            continue;
        };
        let at = item.span().start;
        let types = read_required_params(text, table, &mut Vec::new());
        let failing = case(text, table, at, id, "failing", &types, set, out);
        let passing = case(text, table, at, id, "passing", &types, set, out);
        if let (Some((failing, Some((path, line)))), Some((passing, _))) = (failing, passing) {
            let expected = ExpectedFinding { rule_id: id.to_string(), path, line };
            let cases = RuleCases { failing, passing, expected };
            found.push((id.to_string(), cases, params(&doc, i, "failing"), params(&doc, i, "passing")));
        }
    }
    found
}

/// The `params` of the case `kind` of the `i`th `[[rule]]` in `doc`, empty when absent.
fn params(doc: &toml::Table, i: usize, kind: &str) -> toml::Table {
    doc.get("rule")
        .and_then(|rules| rules.get(i))
        .and_then(|rule| rule.get(kind))
        .and_then(|case| case.get("params"))
        .and_then(|params| params.as_table())
        .cloned()
        .unwrap_or_default()
}

/// The case `kind` of the rule `id` in `table`, read by read_case; absent is a Problem at `at`, the rule's header.
fn case(
    text: &str,
    table: &DeTable,
    at: usize,
    id: &str,
    kind: &str,
    types: &[(String, Option<ParamType>)],
    set: &BaselineSet,
    out: &mut Vec<Problem>,
) -> Option<(Vec<CaseFile>, Option<(String, usize)>)> {
    let Some(value) = table.get(kind) else {
        let shape = match kind {
            "failing" => "failing = { files = [...], path = \"...\", line = N }",
            _ => "passing = { files = [...] }",
        };
        let (name, version) = (set.name, set.version);
        out.push(problem_at(
            text,
            at,
            format!("baseline rule \"{id}\" has no {kind} case"),
            &format!(
                "Add {shape} to the rule, and put its files in crates/rules/baseline/{name}/{version}/cases/{id}/{kind}/, \
                 listed in the set's cases in crates/rules/src/rule_file/{name}_{version}.rs."
            ),
        ));
        return None;
    };
    read_case(text, value, id, kind, types, set, out)
}
