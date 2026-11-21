//! ZUT for read_baseline.rs.

use super::baseline_set::BaselineSet;
use super::read_baseline::read_baseline;
use crate::Problem;
use toml::de::DeTable;

/// Out of name and version order on purpose, so the sorted lists in the fixes are the code's doing.
const SETS: &[BaselineSet] = &[
    BaselineSet { name: "standard", version: 2, text: "version = 1\n# s2\n", yaml: &[], cases: &[] },
    BaselineSet { name: "standard", version: 1, text: "version = 1\n# s1\n", yaml: &[], cases: &[] },
    BaselineSet { name: "house", version: 1, text: "version = 1\n# h1\n", yaml: &[], cases: &[] },
];
const STANDARD_2: &BaselineSet = &SETS[0];
const STANDARD_1: &BaselineSet = &SETS[1];
const HOUSE_1: &BaselineSet = &SETS[2];

fn read(text: &str) -> (Vec<(&'static BaselineSet, usize)>, Vec<Problem>) {
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    let taken = read_baseline(text, doc.get_ref(), SETS, &mut out);
    (taken, out)
}

fn problem(line: usize, col: usize, what: &str, fix: &str) -> Problem {
    Problem { line, col, what: what.to_string(), fix: fix.to_string() }
}

const MALFORMED_FIX: &str = "Write the entry as a set name, '@' and a whole-number version, e.g. \"standard@1\".";

#[test]
fn absent_key_takes_nothing_and_is_no_problem() {
    let (taken, out) = read("version = 1\n");
    assert_eq!(taken, vec![]);
    assert_eq!(out, vec![]);
}

#[test]
fn good_entries_come_back_in_file_order_with_their_offsets() {
    // `baseline = [` is 12 bytes; `"standard@2", ` is 14 more, so "house@1" starts at 26.
    let (taken, out) = read("baseline = [\"standard@2\", \"house@1\"]\n");
    assert_eq!(out, vec![]);
    assert_eq!(taken, vec![(STANDARD_2, 12), (HOUSE_1, 26)]);
}

#[test]
fn a_version_with_a_leading_zero_is_that_version() {
    let (taken, out) = read("baseline = [\"standard@01\"]\n");
    assert_eq!(out, vec![]);
    assert_eq!(taken, vec![(STANDARD_1, 12)]);
}

#[test]
fn a_baseline_that_is_not_an_array_is_the_wrong_type() {
    let (taken, out) = read("baseline = \"standard@1\"\n");
    assert_eq!(taken, vec![]);
    assert_eq!(
        out,
        vec![problem(
            1,
            12,
            "`baseline` must be an array of strings, not string",
            "Write `baseline` as an array of strings."
        )]
    );
}

#[test]
fn malformed_entries_are_problems_and_left_out() {
    let text = "baseline = [\n  \"standard\",\n  \"@1\",\n  \"standard@\",\n  \"standard@x1\",\n  \"standard@1@2\",\n  \
                \"standard@4294967296\",\n  \"standard@+1\",\n]\n";
    let (taken, out) = read(text);
    assert_eq!(taken, vec![]);
    let malformed = |line, entry: &str| problem(line, 3, &format!("malformed baseline entry \"{entry}\""), MALFORMED_FIX);
    assert_eq!(
        out,
        vec![
            malformed(2, "standard"),
            malformed(3, "@1"),
            malformed(4, "standard@"),
            malformed(5, "standard@x1"),
            malformed(6, "standard@1@2"),
            malformed(7, "standard@4294967296"),
            malformed(8, "standard@+1"),
        ]
    );
}

#[test]
fn an_unknown_set_is_a_problem_naming_the_sets() {
    let (taken, out) = read("baseline = [\"nosuch@1\"]\n");
    assert_eq!(taken, vec![]);
    assert_eq!(out, vec![problem(1, 13, "unknown baseline set \"nosuch\"", "Use one of: house, standard.")]);
}

#[test]
fn an_unknown_version_is_a_problem_naming_the_versions_and_takes_nothing() {
    // The bad entry takes nothing, so the later "standard@1" is not taken twice.
    let (taken, out) = read("baseline = [\"standard@9\", \"standard@1\"]\n");
    assert_eq!(taken, vec![(STANDARD_1, 26)]);
    assert_eq!(
        out,
        vec![problem(
            1,
            13,
            "unknown version 9 of baseline set \"standard\"",
            "Use one of: standard@1, standard@2."
        )]
    );
}

#[test]
fn a_set_taken_twice_is_a_problem_at_the_second_entry() {
    let (taken, out) = read("baseline = [\"standard@1\", \"standard@2\"]\n");
    assert_eq!(taken, vec![(STANDARD_1, 12)]);
    assert_eq!(
        out,
        vec![problem(1, 27, "baseline set \"standard\" is taken twice", "Take each set once, at one version.")]
    );
}
