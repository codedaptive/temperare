//! The detekt list version matches the release named in the rule list.

use super::DETEKT_LIST_VERSION;

#[test]
fn detekt_list_version_matches_the_rule_list() {
    let version = include_str!("detekt_rules.txt").lines()
        .find_map(|line| line.strip_prefix("# version: detekt "))
        .expect("detekt rule list has a version")
        .split_whitespace().next().unwrap();
    assert_eq!(DETEKT_LIST_VERSION, version);
}
