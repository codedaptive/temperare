//! `silenced_by_rule`: the silenced findings counted by their rule.

use crate::{RuleCount, Silenced};
use rules::RuleFile;

/// How many of `silenced` each rule has, by the finding's rule id; only rules with at least one. Rules come in the
/// rule file's order (a repeated id at its first place), then the ids the rule file does not hold, in id order, so the
/// counts always add up to `silenced.len()`.
pub(crate) fn silenced_by_rule(rules: &RuleFile, silenced: &[Silenced]) -> Vec<RuleCount> {
    let mut ids: Vec<&str> = Vec::new();
    for rule in &rules.rules {
        if !ids.contains(&rule.id.as_str()) {
            ids.push(&rule.id);
        }
    }
    let mut unknown: Vec<&str> =
        silenced.iter().map(|one| one.finding.rule.as_str()).filter(|id| !ids.contains(id)).collect();
    unknown.sort_unstable();
    unknown.dedup();
    ids.into_iter()
        .chain(unknown)
        .map(|id| RuleCount {
            rule: id.to_string(),
            count: silenced.iter().filter(|one| one.finding.rule == id).count(),
        })
        .filter(|counted| counted.count > 0)
        .collect()
}
