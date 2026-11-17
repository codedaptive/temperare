//! ZUT for read_kits.rs.

use super::read_kits::read_kits;
use crate::{Kit, Problem, RelPath};
use toml::de::DeTable;

fn read(text: &str) -> (Vec<Kit>, Vec<Problem>) {
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    let kits = read_kits(text, doc.get_ref(), &mut out);
    (kits, out)
}

fn names(kits: &[Kit]) -> Vec<&str> {
    kits.iter().map(|kit| kit.name.as_str()).collect()
}

#[test]
fn kits_are_read_with_their_paths() {
    let (kits, out) = read("[kits.core]\npaths = [\"a/**\", \"b/**\"]\n[kits.web]\npaths = [\"w/**\"]\n");
    assert_eq!(out, vec![]);
    assert_eq!(names(&kits), vec!["core", "web"]);
    assert_eq!(kits[0].paths, vec!["a/**".to_string(), "b/**".to_string()]);
    assert_eq!(kits[1].paths, vec!["w/**".to_string()]);
    assert!(kits[0].holds(&RelPath::new("b/x.rs")));
    assert!(!kits[0].holds(&RelPath::new("w/x.rs")));
}

#[test]
fn kits_come_back_in_file_order_not_name_order() {
    let text = "[kits.zeta]\npaths = [\"z/**\"]\n[kits.mid]\npaths = [\"m/**\"]\n[kits.alpha]\npaths = [\"a/**\"]\n";
    let (kits, out) = read(text);
    assert_eq!(out, vec![]);
    assert_eq!(names(&kits), vec!["zeta", "mid", "alpha"]);
}

#[test]
fn a_kit_owner_is_accepted_and_kept_as_written() {
    let (kits, out) = read("[kits.core]\npaths = [\"c/**\"]\nowner = \" core \"\n");
    assert_eq!(out, vec![]);
    assert_eq!(kits[0].owner.as_deref(), Some(" core "));
}

#[test]
fn a_kit_with_no_owner_has_none() {
    let (kits, out) = read("[kits.core]\npaths = [\"c/**\"]\n");
    assert_eq!(out, vec![]);
    assert_eq!(kits[0].owner, None);
}

#[test]
fn a_blank_kit_owner_counts_as_none() {
    let (kits, out) = read("[kits.core]\npaths = [\"c/**\"]\nowner = \"  \"\n");
    assert_eq!(out, vec![]);
    assert_eq!(kits[0].owner, None);
}

#[test]
fn a_kit_owner_that_is_not_a_string_is_a_problem_at_the_value() {
    let (kits, out) = read("[kits.core]\npaths = [\"c/**\"]\nowner = 3\n");
    assert_eq!(kits[0].owner, None);
    assert_eq!(out.len(), 1, "{out:?}");
    assert_eq!((out[0].line, out[0].col), (3, 9));
    assert!(out[0].what.contains("owner"), "{}", out[0].what);
}

#[test]
fn a_kit_with_no_paths_holds_nothing() {
    let (kits, out) = read("[kits.core]\nowner = \"core\"\n");
    assert_eq!(out, vec![]);
    assert!(kits[0].paths.is_empty());
    assert!(!kits[0].holds(&RelPath::new("any/file.rs")));
}

#[test]
fn absent_is_no_kits() {
    let (kits, out) = read("version = 1\n");
    assert!(kits.is_empty());
    assert_eq!(out, vec![]);
}

#[test]
fn a_kit_with_problems_stays_defined_with_its_good_globs() {
    // Line 3: an unknown key; line 2 col 18: a bad glob.
    let (kits, out) = read("[kits.core]\npaths = [\"a/**\", \"src/[a.rs\"]\nname = \"x\"\n");
    assert_eq!(names(&kits), vec!["core"]);
    assert_eq!(kits[0].paths, vec!["a/**".to_string()]);
    let mut found: Vec<(usize, usize)> = out.iter().map(|p| (p.line, p.col)).collect();
    found.sort();
    assert_eq!(found, vec![(2, 18), (3, 1)]);
}
