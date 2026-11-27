//! `shared_groups`: the rules of a rule file split into groups that can share one engine run.

use crate::shares_with::shares_with;
use rules::Rule;

/// Every index of `rules` exactly once, in groups whose members pairwise `shares_with` each other. In rule-file
/// order, rule i joins the earliest group all of whose members share with it, else it starts a new group. Every
/// member is checked, not only the first, because sharing is not transitive (two ruff checks may each be disjoint
/// from a third yet overlap each other). Groups are ordered by their first index; indices ascend within a group.
pub(crate) fn shared_groups(rules: &[Rule]) -> Vec<Vec<usize>> {
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for (i, rule) in rules.iter().enumerate() {
        match groups.iter_mut().find(|group| group.iter().all(|&j| shares_with(&rules[j], rule))) {
            Some(group) => group.push(i),
            None => groups.push(vec![i]),
        }
    }
    groups
}
