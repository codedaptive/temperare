//! `resolve_baseline`: the rules of the taken baseline sets, with the rule file's overrides applied.

use super::apply_override::apply_override;
use super::baseline_set::BaselineSet;
use super::load_set::load_set;
use super::missing_params::missing_params;
use super::override_entry::OverrideEntry;
use super::problem_at::problem_at;
use super::resolved_baseline::ResolvedBaseline;
use crate::{Kind, Problem, Rule};
use std::collections::BTreeMap;

/// The sets of `taken` (each with the byte offset of its entry in `text`) loaded in order, and `overrides` applied.
/// A set with problems of its own is one Problem per problem at its entry and gives nothing. A rule id an earlier
/// set already gave is a Problem at the later entry, whose copy is left out. An id one set retires and another gives
/// is a Problem at the retiring set's entry. An override naming no rule of the taken sets is a Problem at its `rule`
/// string. A rule whose required params its override's `params` do not give goes to `not_applicable` with the
/// reason. Each rule in `rules` or `not_applicable` gets the next `position` from 0 in taken order, then each set's
/// order; an off rule takes none. `security_at` is the entry offset of the first security rule in `rules`.
pub(crate) fn resolve_baseline(
    text: &str,
    taken: &[(&BaselineSet, usize)],
    overrides: &[OverrideEntry],
    out: &mut Vec<Problem>,
) -> ResolvedBaseline {
    let mut resolved = ResolvedBaseline::default();
    // Each rule id with the "name@version" of the set that gave it.
    let mut given_by: BTreeMap<String, String> = BTreeMap::new();
    // Each kept set rule with the byte offset of the entry that brings it.
    let mut set_rules = Vec::new();
    // Each set's retired ids with its label and entry offset, checked once every set is loaded.
    let mut retired_by = Vec::new();
    for (set, at) in taken {
        let label = format!("{}@{}", set.name, set.version);
        if let Some((rules, retired)) = load_or_problems(text, set, &label, *at, out) {
            let kept = first_copies(text, rules, &label, *at, &mut given_by, out);
            set_rules.extend(kept.into_iter().map(|rule| (rule, *at)));
            resolved.retired.extend(retired.iter().cloned());
            retired_by.push((retired, label.clone(), *at));
        }
        resolved.taken.push(label);
    }
    for (retired, label, at) in &retired_by {
        check_retired_given(text, retired, label, *at, &given_by, out);
    }
    for o in overrides.iter().filter(|o| !given_by.contains_key(&o.rule)) {
        out.push(problem_at(
            text,
            o.at,
            format!("override of unknown baseline rule \"{}\": no taken set holds it", o.rule),
            "Name the id of a rule in a set this file takes in baseline, or remove the [[override]].",
        ));
    }
    for (rule, at) in set_rules {
        let found = overrides.iter().find(|o| o.rule == rule.id);
        let applied = match found {
            Some(o) => apply_override(text, rule, o, out),
            None => Some(rule),
        };
        let Some(mut rule) = applied else {
            continue;
        };
        rule.not_applicable = missing_params(&rule, found.and_then(|o| o.params.as_ref()));
        rule.position = resolved.rules.len() + resolved.not_applicable.len();
        if rule.not_applicable.is_some() {
            resolved.not_applicable.push(rule);
        } else {
            if rule.kind == Kind::Security && resolved.security_at.is_none() {
                resolved.security_at = Some(at);
            }
            resolved.rules.push(rule);
        }
    }
    resolved.ids = given_by.into_keys().collect();
    resolved
}

/// The rules and retired ids of `set`, or None with one Problem at its entry for each problem of the set's own text.
fn load_or_problems(
    text: &str,
    set: &BaselineSet,
    label: &str,
    at: usize,
    out: &mut Vec<Problem>,
) -> Option<(Vec<Rule>, Vec<String>)> {
    match load_set(set) {
        Ok(loaded) => Some(loaded),
        Err(problems) => {
            out.extend(problems.into_iter().map(|p| {
                problem_at(
                    text,
                    at,
                    format!("baseline set {label} has a problem at {}:{}: {}", p.line, p.col, p.what),
                    "This is a fault in cctao's embedded set; report it with this message.",
                )
            }));
            None
        }
    }
}

/// The rules of the set `label` whose ids no earlier set gave, recording each in `given_by`; a repeated id is a
/// Problem at the set's entry, naming the earlier set first.
fn first_copies(
    text: &str,
    rules: Vec<Rule>,
    label: &str,
    at: usize,
    given_by: &mut BTreeMap<String, String>,
    out: &mut Vec<Problem>,
) -> Vec<Rule> {
    let mut kept = Vec::new();
    for rule in rules {
        if let Some(earlier) = given_by.get(&rule.id) {
            out.push(problem_at(
                text,
                at,
                format!("rule \"{}\" is in both {earlier} and {label}", rule.id),
                "Take only one of the two sets.",
            ));
            continue;
        }
        given_by.insert(rule.id.clone(), label.to_string());
        kept.push(rule);
    }
    kept
}

/// A Problem at the entry `at` of the set `label` for each id it retires that another taken set gives: an id is never
/// reused.
fn check_retired_given(
    text: &str,
    retired: &[String],
    label: &str,
    at: usize,
    given_by: &BTreeMap<String, String>,
    out: &mut Vec<Problem>,
) {
    for (id, giver) in retired.iter().filter_map(|id| Some((id, given_by.get(id)?))) {
        out.push(problem_at(
            text,
            at,
            format!("retired id \"{id}\" in {label} is a rule in {giver}"),
            "Take only one of the two sets: a retired id is never given again.",
        ));
    }
}
