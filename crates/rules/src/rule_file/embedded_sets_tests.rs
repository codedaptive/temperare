//! Unit tests for embedded_sets.rs.

use super::embedded_sets::EMBEDDED_SETS;

#[test]
fn the_released_sets_are_house_1_and_standard_1() {
    let ids: Vec<(&str, u32)> = EMBEDDED_SETS.iter().map(|set| (set.name, set.version)).collect();
    assert_eq!(ids, vec![("house", 1), ("standard", 1)]);
}

#[test]
fn each_set_text_says_version_1() {
    for set in EMBEDDED_SETS {
        assert!(set.text.lines().any(|line| line == "version = 1"), "{}@{}: {:?}", set.name, set.version, set.text);
    }
}
