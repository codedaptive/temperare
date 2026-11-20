//! ZUT for shared_groups.rs: group order, joining the earliest group, and a rule that shares with only some members.

use crate::shared_groups::shared_groups;
use rules::RuleFile;
use std::path::Path;

/// The groups of a rule file whose rules are (engine, check), in order.
fn groups(rules: &[(&str, &str)]) -> Vec<Vec<usize>> {
    let blocks: String = rules
        .iter()
        .enumerate()
        .map(|(i, (engine, check))| format!("[[rule]]\nid = \"r{i}\"\nengine = \"{engine}\"\ncheck = \"{check}\"\nfix = \"f\"\n"))
        .collect();
    shared_groups(&RuleFile::parse(&format!("version = 1\n{blocks}"), Path::new(".")).unwrap().rules)
}

#[test]
fn no_rules_make_no_groups() {
    assert_eq!(groups(&[]), Vec::<Vec<usize>>::new());
}

#[test]
fn groups_follow_their_first_index_and_indices_ascend() {
    let rules = [
        ("ruff", "E711"),
        ("swiftlint", "force_cast"),
        ("ruff", "F401"),
        ("tessera", "slop"),
        ("swiftlint", "force_try"),
        ("tessera", "slop"),
    ];
    assert_eq!(groups(&rules), vec![vec![0, 2], vec![1, 4], vec![3], vec![5]]);
}

#[test]
fn a_rule_joins_the_earliest_group_it_shares_with() {
    // E7 and E711 overlap, so they start two groups; F401 shares with both and joins the first.
    assert_eq!(groups(&[("ruff", "E7"), ("ruff", "E711"), ("ruff", "F401")]), vec![vec![0, 2], vec![1]]);
}

#[test]
fn a_rule_sharing_with_only_some_members_starts_its_own_group() {
    // E711 shares with F401 but overlaps E7, so it cannot join the group of F401 and E7.
    assert_eq!(groups(&[("ruff", "F401"), ("ruff", "E7"), ("ruff", "E711")]), vec![vec![0, 1], vec![2]]);
}

#[test]
fn a_rule_sharing_with_every_member_joins() {
    assert_eq!(groups(&[("ruff", "F401"), ("ruff", "E7"), ("ruff", "W291")]), vec![vec![0, 1, 2]]);
}
