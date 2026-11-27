//! Unit test for detekt_rule_list.rs: the list is exactly the rules detekt 1.23.8's own SARIF report names, in sorted order:
//! `runs[0].tool.driver.rules[].id` of the real fixture `fixtures/magic_number.sarif`, each with `detekt.` cut.

use super::detekt_rule_list;

#[test]
fn detekt_rule_list_is_every_rule_detekts_sarif_names_sorted() {
    let sarif: serde_json::Value = serde_json::from_str(include_str!("fixtures/magic_number.sarif")).unwrap();
    let mut expected: Vec<(&str, &str)> = sarif["runs"][0]["tool"]["driver"]["rules"]
        .as_array()
        .unwrap()
        .iter()
        .map(|rule| rule["id"].as_str().unwrap().strip_prefix("detekt.").unwrap().split_once('.').unwrap())
        .collect();
    expected.sort_unstable();
    assert_eq!(expected.len(), 214);
    assert_eq!(detekt_rule_list(), expected);
}
