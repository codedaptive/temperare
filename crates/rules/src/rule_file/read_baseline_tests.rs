//! Unit tests for read_baseline.rs.

use super::baseline_set::BaselineSet;
use super::read_baseline::read_baseline;
use crate::Problem;
use toml::de::DeTable;

/// Out of name and version order on purpose, so the sorted lists in the fixes are the code's doing.
const SETS: &[BaselineSet] = &[
    BaselineSet { name: "standard", version: 2, ready: true, text: "version = 1\n# s2\n", yaml: &[], cases: &[] },
    BaselineSet { name: "standard", version: 1, ready: true, text: "version = 1\n# s1\n", yaml: &[], cases: &[] },
    BaselineSet { name: "house", version: 1, ready: true, text: "version = 1\n# h1\n", yaml: &[], cases: &[] },
    BaselineSet { name: "house", version: 2, ready: false, text: "version = 1\n# h2\n", yaml: &[], cases: &[] },
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
fn an_unknown_set_is_a_problem_naming_only_the_ready_sets() {
    const READY_AND_NOT: &[BaselineSet] = &[
        BaselineSet { name: "standard", version: 1, ready: true, text: "version = 1\n", yaml: &[], cases: &[] },
        BaselineSet { name: "house", version: 1, ready: false, text: "version = 1\n", yaml: &[], cases: &[] },
    ];
    const NONE_READY: &[BaselineSet] =
        &[BaselineSet { name: "house", version: 1, ready: false, text: "version = 1\n", yaml: &[], cases: &[] }];
    const TWO_AND_TEN: &[BaselineSet] = &[
        BaselineSet { name: "standard", version: 10, ready: true, text: "version = 1\n", yaml: &[], cases: &[] },
        BaselineSet { name: "standard", version: 2, ready: true, text: "version = 1\n", yaml: &[], cases: &[] },
    ];
    // SETS: house@1 is ready and listed; house@2 is not ready and is not.
    let rows: [(&str, &[BaselineSet], &str); 4] = [
        ("every ready version is listed", SETS, "Use one of: house@1, standard@1, standard@2."),
        ("a set with no ready version is not listed", READY_AND_NOT, "Use one of: standard@1."),
        ("versions sort as numbers", TWO_AND_TEN, "Use one of: standard@2, standard@10."),
        ("no ready set", NONE_READY, "No baseline set is ready in this build; remove the entry."),
    ];
    let text = "baseline = [\"nosuch@1\"]\n";
    let doc = DeTable::parse(text).unwrap();
    for (case, sets, fix) in rows {
        let mut out = Vec::new();
        let taken = read_baseline(text, doc.get_ref(), sets, &mut out);
        assert_eq!(taken, vec![], "{case}");
        assert_eq!(out, vec![problem(1, 13, "unknown baseline set \"nosuch\"", fix)], "{case}");
    }
}

#[test]
fn an_unknown_version_is_a_problem_naming_the_ready_versions_and_takes_nothing() {
    const NONE_READY: &[BaselineSet] = &[
        BaselineSet { name: "draft", version: 2, ready: false, text: "version = 1\n", yaml: &[], cases: &[] },
        BaselineSet { name: "draft", version: 1, ready: false, text: "version = 1\n", yaml: &[], cases: &[] },
    ];
    // Each bad entry takes nothing, so the later good entry is not taken twice; `baseline = [` is 12 bytes, so the
    // bad entry's string starts at column 13 and the good one 2 bytes past the bad one's end.
    let rows: [(&str, &[BaselineSet], &str, Vec<(&BaselineSet, usize)>, &str, &str); 3] = [
        (
            "every version ready",
            SETS,
            "baseline = [\"standard@9\", \"standard@1\"]\n",
            vec![(STANDARD_1, 26)],
            "unknown version 9 of baseline set \"standard\"",
            "Use one of: standard@1, standard@2.",
        ),
        (
            "an unready version is not listed",
            SETS,
            "baseline = [\"house@9\", \"house@1\"]\n",
            vec![(HOUSE_1, 23)],
            "unknown version 9 of baseline set \"house\"",
            "Use one of: house@1.",
        ),
        (
            "no version ready",
            NONE_READY,
            "baseline = [\"draft@9\"]\n",
            vec![],
            "unknown version 9 of baseline set \"draft\"",
            "No version of baseline set \"draft\" is ready in this build; remove the entry.",
        ),
    ];
    for (case, sets, text, expected_taken, what, fix) in rows {
        let doc = DeTable::parse(text).unwrap();
        let mut out = Vec::new();
        let taken = read_baseline(text, doc.get_ref(), sets, &mut out);
        assert_eq!(taken, expected_taken, "{case}");
        assert_eq!(out, vec![problem(1, 13, what, fix)], "{case}");
    }
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

#[test]
fn a_set_not_ready_is_a_problem_naming_it_and_keeps_the_others() {
    // `baseline = [` is 12 bytes; `"house@2", ` is 11 more, so "standard@1" starts at 23.
    let (taken, out) = read("baseline = [\"house@2\", \"standard@1\"]\n");
    assert_eq!(taken, vec![(STANDARD_1, 23)]);
    assert_eq!(
        out,
        vec![problem(
            1,
            13,
            "house@2 is not ready in this build: its rules are not yet proven",
            "remove house@2 from baseline"
        )]
    );
}
