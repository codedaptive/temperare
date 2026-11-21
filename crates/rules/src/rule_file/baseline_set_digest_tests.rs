//! ZUT for baseline_set_digest.rs.

use super::baseline_set::BaselineSet;
use super::baseline_set_digest::baseline_set_digest;

/// A tiny set named "t" at version 1, ready, with these files.
const fn set(text: &'static str, yaml: &'static [(&'static str, &'static str)], cases: &'static [(&'static str, &'static str)]) -> BaselineSet {
    BaselineSet { name: "t", version: 1, ready: true, text, yaml, cases }
}

#[test]
fn baseline_set_digest_gives_the_fnv_1a_value_of_each_set() {
    let rows = [
        ("empty", set("", &[], &[]), 0x0a99_c807_b6f6_45b0_u64),
        ("base", set("a", &[("y", "b")], &[("c", "d")]), 0xc261_77f7_8ab2_783f),
        ("moved", set("a", &[], &[("y", "b"), ("c", "d")]), 0xbdbf_2829_cda5_295d),
        ("renamed", set("a", &[("z", "b")], &[("c", "d")]), 0xcc94_dcd8_6156_b7fe),
        ("swapped", set("a", &[("b", "y")], &[("c", "d")]), 0x1245_1652_712b_68e3),
    ];
    for (label, set, expected) in rows {
        let found = baseline_set_digest(&set);
        assert_eq!(found, expected, "{label}: digest {found:#018x}, expected {expected:#018x}");
    }
}
