//! Unit test for picked_rules.rs: a rule part picks its rule, a set part every rule of the set (those that need type
//! resolution included), once each in list order; an unlisted part picks nothing. The empty-blocks rules are written
//! out from detekt_rules.txt.

use super::picked_rules;

const EMPTY_BLOCKS: [(&str, &str); 15] = [
    ("empty-blocks", "EmptyCatchBlock"),
    ("empty-blocks", "EmptyClassBlock"),
    ("empty-blocks", "EmptyDefaultConstructor"),
    ("empty-blocks", "EmptyDoWhileBlock"),
    ("empty-blocks", "EmptyElseBlock"),
    ("empty-blocks", "EmptyFinallyBlock"),
    ("empty-blocks", "EmptyForBlock"),
    ("empty-blocks", "EmptyFunctionBlock"),
    ("empty-blocks", "EmptyIfBlock"),
    ("empty-blocks", "EmptyInitBlock"),
    ("empty-blocks", "EmptyKtFile"),
    ("empty-blocks", "EmptySecondaryConstructor"),
    ("empty-blocks", "EmptyTryBlock"),
    ("empty-blocks", "EmptyWhenBlock"),
    ("empty-blocks", "EmptyWhileBlock"),
];

#[test]
fn picked_rules_gives_each_picked_rule_once_in_list_order() {
    let rows: [(&str, Vec<(&str, &str)>); 8] = [
        ("MagicNumber", vec![("style", "MagicNumber")]),
        ("MagicNumber,MagicNumber", vec![("style", "MagicNumber")]),
        ("empty-blocks", EMPTY_BLOCKS.to_vec()),
        ("NoSuchRule", vec![]),
        ("MagicNumber,EmptyCatchBlock", vec![("empty-blocks", "EmptyCatchBlock"), ("style", "MagicNumber")]),
        ("EmptyIfBlock,empty-blocks", EMPTY_BLOCKS.to_vec()),
        ("Deprecation", vec![("potential-bugs", "Deprecation")]),
        ("Deprecation,UnsafeCast", vec![("potential-bugs", "Deprecation"), ("potential-bugs", "UnsafeCast")]),
    ];
    for (check, expected) in rows {
        assert_eq!(picked_rules(check), expected, "{check}");
    }
}

#[test]
fn a_set_part_picks_its_rules_that_need_type_resolution_too() {
    let got = picked_rules("potential-bugs");
    assert_eq!(got.len(), 38, "{got:?}");
    for pair in [("potential-bugs", "Deprecation"), ("potential-bugs", "UnsafeCast"), ("potential-bugs", "EqualsAlwaysReturnsTrueOrFalse")] {
        assert!(got.contains(&pair), "{pair:?} in {got:?}");
    }
}
