//! ZUT for baseline_set.rs.

use super::baseline_set::BaselineSet;

#[test]
fn sets_differing_only_in_version_are_different_sets() {
    let one = BaselineSet { name: "standard", version: 1, ready: true, text: "version = 1\n", yaml: &[], cases: &[] };
    let two = BaselineSet { version: 2, ..one };
    assert_ne!(one, two);
    assert_eq!(two, BaselineSet { name: "standard", version: 2, ready: true, text: "version = 1\n", yaml: &[], cases: &[] });
}

#[test]
fn sets_differing_only_in_their_case_files_are_different_sets() {
    let none = BaselineSet { name: "standard", version: 1, ready: true, text: "version = 1\n", yaml: &[], cases: &[] };
    let one = BaselineSet { cases: &[("cases/tm-a/failing/a.rs", "x\n")], ..none };
    assert_ne!(none, one);
    assert_eq!(
        one,
        BaselineSet {
            name: "standard",
            version: 1,
            ready: true,
            text: "version = 1\n",
            yaml: &[],
            cases: &[("cases/tm-a/failing/a.rs", "x\n")]
        }
    );
}
