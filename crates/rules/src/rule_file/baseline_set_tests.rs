//! ZUT for baseline_set.rs.

use super::baseline_set::BaselineSet;

#[test]
fn sets_differing_only_in_version_are_different_sets() {
    let one = BaselineSet { name: "standard", version: 1, text: "version = 1\n", yaml: &[] };
    let two = BaselineSet { version: 2, ..one };
    assert_ne!(one, two);
    assert_eq!(two, BaselineSet { name: "standard", version: 2, text: "version = 1\n", yaml: &[] });
}
