//! ZUT for read_kits.rs.

use super::read_kits::read_kits;
use std::collections::BTreeMap;
use toml::de::DeTable;

#[test]
fn kits_are_read_with_their_paths() {
    let text = "[kits.core]\npaths = [\"a/**\", \"b/**\"]\n[kits.web]\npaths = [\"w/**\"]\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    let kits = read_kits(text, doc.get_ref(), &mut out);
    let expected = BTreeMap::from([
        ("core".to_string(), vec!["a/**".to_string(), "b/**".to_string()]),
        ("web".to_string(), vec!["w/**".to_string()]),
    ]);
    assert_eq!(kits, expected);
    assert_eq!(out, vec![]);
}

#[test]
fn absent_is_no_kits() {
    let text = "version = 1\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert!(read_kits(text, doc.get_ref(), &mut out).is_empty());
    assert_eq!(out, vec![]);
}

#[test]
fn a_kit_with_problems_stays_defined_with_its_good_globs() {
    // Line 3: an unknown key; line 2 col 18: a bad glob.
    let text = "[kits.core]\npaths = [\"a/**\", \"src/[a.rs\"]\nname = \"x\"\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    let kits = read_kits(text, doc.get_ref(), &mut out);
    assert_eq!(kits, BTreeMap::from([("core".to_string(), vec!["a/**".to_string()])]));
    let mut found: Vec<(usize, usize)> = out.iter().map(|p| (p.line, p.col)).collect();
    found.sort();
    assert_eq!(found, vec![(2, 18), (3, 1)]);
}
