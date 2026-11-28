//! `apply_override`: a baseline rule as a repository's `[[override]]` changes it.

use super::override_entry::OverrideEntry;
use super::override_owner::override_owner;
use super::scope_or_problem::scope_or_problem;
use crate::{Problem, Rule, Scope};

/// `rule` with `o` applied, or None when `o` turns it off. A given severity, paths or exclude replaces the rule's;
/// given params are merged key by key, the override's value winning. When paths or exclude were given the scope is
/// rebuilt from the rule's paths and exclude with no kit limit (a set's rules are read with no kits); a failed build
/// is a Problem at the override's `rule` string, whose what starts `[[override]] for "<rule>": `, and None.
pub(crate) fn apply_override(text: &str, mut rule: Rule, o: &OverrideEntry, out: &mut Vec<Problem>) -> Option<Rule> {
    if o.off {
        return None;
    }
    if let Some(severity) = o.severity {
        rule.severity = severity;
    }
    if let Some(paths) = &o.paths {
        rule.paths = paths.clone();
    }
    if let Some(exclude) = &o.exclude {
        rule.exclude = exclude.clone();
    }
    for (key, value) in o.params.iter().flatten() {
        rule.params.insert(key.clone(), value.clone());
    }
    if o.paths.is_some() || o.exclude.is_some() {
        let built = Scope::build(&rule.paths, &rule.exclude, None);
        rule.scope = scope_or_problem(
            built,
            text,
            o.at,
            |e| format!("{}: {e}", override_owner(Some(&o.rule))),
            "Correct the override's globs.",
            out,
        )?;
    }
    Some(rule)
}
