use std::collections::{BTreeSet, HashSet};

use crate::Oid;

fn oid(text: &str) -> Oid {
    Oid::parse(text).expect("a valid id")
}

#[test]
fn the_same_id_in_either_case_is_one_set_member() {
    let ids: HashSet<Oid> = [
        oid("0123456789abcdef0123456789abcdef01234567"),
        oid("0123456789ABCDEF0123456789ABCDEF01234567"),
        oid("1111111111111111111111111111111111111111"),
    ]
    .into_iter()
    .collect();
    assert_eq!(ids.len(), 2);
}

#[test]
fn ids_order_by_their_text() {
    let ids: BTreeSet<Oid> = [
        oid("ffffffffffffffffffffffffffffffffffffffff"),
        oid("0000000000000000000000000000000000000000"),
        oid("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"),
    ]
    .into_iter()
    .collect();
    let texts: Vec<&str> = ids.iter().map(Oid::as_str).collect();
    assert_eq!(
        texts,
        [
            "0000000000000000000000000000000000000000",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "ffffffffffffffffffffffffffffffffffffffff",
        ]
    );
}
