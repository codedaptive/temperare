//! `read_set_cases`: the failing and passing cases of every rule of a baseline set.

use super::baseline_set::BaselineSet;
use super::problem_at::problem_at;
use super::read_case::read_case;
use crate::{CaseFile, ExpectedFinding, Problem, RuleCases};
use toml::de::DeTable;

/// Each `[[rule]]` of `root` (the set's own `text`) whose `id` is a string and whose cases have no Problem: that id
/// and its cases, in file order. A rule with no `failing` or no `passing` case is a Problem at its `[[rule]]` header,
/// and read_case reports each problem in a case; every Problem goes to `out`. A `rule` that is not an array, an entry
/// that is not a table, or an `id` that is absent or not a string gives nothing here: read_rules reports those.
pub(crate) fn read_set_cases(
    text: &str,
    root: &DeTable,
    set: &BaselineSet,
    out: &mut Vec<Problem>,
) -> Vec<(String, RuleCases)> {
    let Some(tables) = root.get("rule").and_then(|value| value.get_ref().as_array()) else {
        return Vec::new();
    };
    let mut found = Vec::new();
    for item in tables {
        let Some(table) = item.get_ref().as_table() else {
            continue;
        };
        let Some(id) = table.get("id").and_then(|id| id.get_ref().as_str()) else {
            continue;
        };
        let at = item.span().start;
        let failing = case(text, table, at, id, "failing", set, out);
        let passing = case(text, table, at, id, "passing", set, out);
        if let (Some((failing, Some((path, line)))), Some((passing, _))) = (failing, passing) {
            let expected = ExpectedFinding { rule_id: id.to_string(), path, line };
            found.push((id.to_string(), RuleCases { failing, passing, expected }));
        }
    }
    found
}

/// The case `kind` of the rule `id` in `table`, read by read_case; absent is a Problem at `at`, the rule's header.
fn case(
    text: &str,
    table: &DeTable,
    at: usize,
    id: &str,
    kind: &str,
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
                 listed in the set's cases in EMBEDDED_SETS."
            ),
        ));
        return None;
    };
    read_case(text, value, id, kind, set, out)
}
