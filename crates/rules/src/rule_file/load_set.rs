//! `load_set`: the rules and retired ids of one embedded baseline set.

use super::baseline_set::BaselineSet;
use super::check_keys::check_keys;
use super::check_set_carried::check_set_carried;
use super::check_set_yaml::check_set_yaml;
use super::check_version::check_version;
use super::problem_at::problem_at;
use super::read_rules::read_rules;
use super::read_set_cases::read_set_cases;
use super::string_list::string_list;
use super::syntax_problem::syntax_problem;
use crate::{Engine, Problem, Rule};
use std::collections::{BTreeMap, BTreeSet};
use toml::de::DeTable;

const SET_TOP_KEYS: [&str; 3] = ["version", "rule", "retired"];

/// The rules of `set` (read as a set's rules, with no kits) and the ids its top-level `retired` lists, in file order;
/// or every Problem, positioned in the set's own text and sorted by (line, col). Each ast-grep rule's `check` must
/// name a YAML file the set carries, and the rule comes back with that file's text as its embedded YAML; a rule of
/// any other engine has none. Every rule must declare its failing and passing cases, whose files the set carries,
/// and comes back with them as its embedded cases, and with the params each case gives as its `failing_params` and
/// `passing_params` (empty when the case gives none). Every case file and YAML file the set carries must be named by a
/// rule; one that is not is a Problem at 1:1, since carried data has no place in the set's text.
pub(crate) fn load_set(set: &BaselineSet) -> Result<(Vec<Rule>, Vec<String>), Vec<Problem>> {
    let text = set.text;
    let doc = DeTable::parse(text).map_err(|e| vec![syntax_problem(text, &e)])?;
    let root = doc.get_ref();
    let mut out = Vec::new();
    check_keys(text, root, &SET_TOP_KEYS, "the top level", &mut out);
    check_version(text, root, &mut out);
    let (mut rules, ids) = read_rules(text, root, &[], true, &mut out);
    check_set_yaml(text, root, set, &mut out);
    check_set_carried(text, root, set, &mut out);
    let mut cases: BTreeMap<_, _> = read_set_cases(text, root, set, &mut out)
        .into_iter()
        .map(|(id, cases, failing, passing)| (id, (cases, failing, passing)))
        .collect();
    let retired = string_list(text, root, "retired", &mut out);
    for (id, at) in &retired {
        check_retired(text, id, *at, &ids, &mut out);
    }
    if out.is_empty() {
        for rule in rules.iter_mut() {
            if rule.engine == Engine::AstGrep {
                rule.embedded_text = set.yaml_for(&rule.check);
            }
            if let Some((cases, failing, passing)) = cases.remove(&rule.id) {
                (rule.embedded_cases, rule.failing_params, rule.passing_params) = (Some(cases), failing, passing);
            }
        }
        return Ok((rules, retired.into_iter().map(|(id, _)| id).collect()));
    }
    out.sort_by_key(|p| (p.line, p.col));
    Err(out)
}

/// A Problem at `at` when the retired `id` does not start "tm-", or else when a `[[rule]]` of the set has it: an id
/// is never reused, so a retired id names no rule.
fn check_retired(text: &str, id: &str, at: usize, rule_ids: &BTreeSet<String>, out: &mut Vec<Problem>) {
    let (what, fix) = if !id.starts_with("tm-") {
        (format!("retired id \"{id}\" does not start with \"tm-\""), "Start every baseline rule's id with \"tm-\".")
    } else if rule_ids.contains(id) {
        (
            format!("retired id \"{id}\" is also a rule in this set"),
            "Take the rule out of the set, or out of retired: an id is never reused.",
        )
    } else {
        return;
    };
    out.push(problem_at(text, at, what, fix));
}
