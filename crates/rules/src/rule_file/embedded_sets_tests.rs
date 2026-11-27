//! Unit tests for embedded_sets.rs.

use super::embedded_sets::EMBEDDED_SETS;

#[test]
fn the_built_in_sets_are_house_1_house_2_and_standard_1_all_ready() {
    let ids: Vec<(&str, u32, bool)> = EMBEDDED_SETS.iter().map(|set| (set.name, set.version, set.ready)).collect();
    assert_eq!(ids, vec![("house", 1, true), ("house", 2, true), ("standard", 1, true)]);
}

#[test]
fn each_set_text_says_version_1() {
    for set in EMBEDDED_SETS {
        assert!(set.text.lines().any(|line| line == "version = 1"), "{}@{}: {:?}", set.name, set.version, set.text);
    }
}
