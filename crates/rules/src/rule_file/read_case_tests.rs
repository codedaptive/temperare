//! ZUT for read_case.rs.

use super::baseline_set::BaselineSet;
use super::read_case::read_case;
use crate::{CaseFile, Problem};
use toml::de::DeTable;

/// Carries two failing files and one passing file of `tm-a`; `src/lib.rs` is carried under both, with other texts.
/// The failing `src/lib.rs` has three lines, so `line = 3` is its last line.
const SET: BaselineSet = BaselineSet {
    name: "house",
    version: 2,
    ready: true,
    text: "",
    yaml: &[],
    cases: &[
        ("cases/tm-a/failing/src/lib.rs", "failing\nlib\nthree\n"),
        ("cases/tm-a/failing/Cargo.toml", "failing toml\n"),
        ("cases/tm-a/passing/src/lib.rs", "passing lib\n"),
    ],
};

type Case = Option<(Vec<CaseFile>, Option<(String, usize)>)>;

/// Reads the case `kind` of rule `tm-a` from the one-line `text`, which sets `kind`.
fn read(text: &str, kind: &str) -> (Case, Vec<Problem>) {
    let doc = DeTable::parse(text).expect("valid TOML");
    let value = doc.get_ref().get(kind).expect("the case");
    let mut out = Vec::new();
    let case = read_case(text, value, "tm-a", kind, &[], &SET, &mut out);
    (case, out)
}

fn file(path: &str, text: &'static str) -> CaseFile {
    CaseFile { path: path.to_string(), text }
}

fn problem(col: usize, what: &str, fix: &str) -> Problem {
    Problem { line: 1, col, what: what.to_string(), fix: fix.to_string() }
}

#[test]
fn a_failing_case_gives_its_carried_files_in_order_and_its_finding() {
    let text = "failing = { files = [\"src/lib.rs\", \"Cargo.toml\"], path = \"src/lib.rs\", line = 3 }";
    let files = vec![file("src/lib.rs", "failing\nlib\nthree\n"), file("Cargo.toml", "failing toml\n")];
    assert_eq!(read(text, "failing"), (Some((files, Some(("src/lib.rs".to_string(), 3)))), vec![]));
}

#[test]
fn a_passing_case_gives_the_files_carried_under_passing_and_no_finding() {
    let text = "passing = { files = [\"src/lib.rs\"] }";
    assert_eq!(read(text, "passing"), (Some((vec![file("src/lib.rs", "passing lib\n")], None)), vec![]));
}

#[test]
fn a_case_that_is_not_a_table_is_the_wrong_type() {
    let expected = problem(11, "`failing` must be a table, not string", "Write `failing` as a table.");
    assert_eq!(read("failing = \"src/lib.rs\"", "failing"), (None, vec![expected]));
}

#[test]
fn an_unknown_key_in_a_failing_case_is_a_problem_at_it() {
    let text = "failing = { files = [\"src/lib.rs\"], path = \"src/lib.rs\", line = 3, rule = \"x\" }";
    let expected = problem(68, "unknown key `rule` in failing", "Remove it, or use one of: files, path, line, params.");
    assert_eq!(read(text, "failing"), (None, vec![expected]));
}

#[test]
fn a_path_in_a_passing_case_is_an_unknown_key() {
    let text = "passing = { files = [\"src/lib.rs\"], path = \"src/lib.rs\" }";
    let expected = problem(37, "unknown key `path` in passing", "Remove it, or use one of: files, params.");
    assert_eq!(read(text, "passing"), (None, vec![expected]));
}

#[test]
fn a_case_with_files_absent_or_empty_lists_no_files_at_the_case() {
    let what = "the passing case of baseline rule \"tm-a\" lists no files";
    for text in ["passing = {}", "passing = { files = [] }"] {
        let expected = problem(11, what, "List the case's files in files.");
        assert_eq!(read(text, "passing"), (None, vec![expected]), "{text}");
    }
}

#[test]
fn files_that_is_not_an_array_is_only_the_wrong_type() {
    let expected =
        problem(21, "`files` must be an array of strings, not string", "Write `files` as an array of strings.");
    assert_eq!(read("passing = { files = \"src/lib.rs\" }", "passing"), (None, vec![expected]));
}

#[test]
fn a_file_entry_that_is_not_a_string_is_the_wrong_type_at_it() {
    let expected =
        problem(36, "`files` must be an array of strings, not integer", "Write `files` as an array of strings.");
    assert_eq!(read("passing = { files = [\"src/lib.rs\", 1] }", "passing"), (None, vec![expected]));
}

#[test]
fn every_file_the_set_does_not_carry_is_a_problem_at_its_entry() {
    let text = "failing = { files = [\"src/lib.rs\", \"gone.rs\", \"src/main.rs\"], path = \"src/lib.rs\", line = 3 }";
    let expected = vec![
        problem(
            36,
            "case file \"gone.rs\" of baseline rule \"tm-a\" is not carried by baseline set house@2",
            "Put it at crates/rules/baseline/house/2/cases/tm-a/failing/gone.rs and list it in the set's cases in \
             crates/rules/src/rule_file/house_2.rs, or correct files.",
        ),
        problem(
            47,
            "case file \"src/main.rs\" of baseline rule \"tm-a\" is not carried by baseline set house@2",
            "Put it at crates/rules/baseline/house/2/cases/tm-a/failing/src/main.rs and list it in the set's cases in \
             crates/rules/src/rule_file/house_2.rs, or correct files.",
        ),
    ];
    assert_eq!(read(text, "failing"), (None, expected));
}

#[test]
fn a_passing_file_carried_only_under_failing_is_not_carried() {
    let expected = problem(
        22,
        "case file \"Cargo.toml\" of baseline rule \"tm-a\" is not carried by baseline set house@2",
        "Put it at crates/rules/baseline/house/2/cases/tm-a/passing/Cargo.toml and list it in the set's cases in \
         crates/rules/src/rule_file/house_2.rs, or correct files.",
    );
    assert_eq!(read("passing = { files = [\"Cargo.toml\"] }", "passing"), (None, vec![expected]));
}

#[test]
fn a_failing_case_missing_path_and_line_is_both_problems_at_the_case() {
    let expected = vec![
        problem(11, "failing is missing `path`", "Add `path = \"...\"` to this failing."),
        problem(
            11,
            "failing is missing `line`",
            "Add `line = N`, the line of the finding the failing case must give.",
        ),
    ];
    assert_eq!(read("failing = { files = [\"src/lib.rs\"] }", "failing"), (None, expected));
}

#[test]
fn a_path_that_is_not_a_string_is_the_wrong_type() {
    let text = "failing = { files = [\"src/lib.rs\"], path = 7, line = 3 }";
    let expected = problem(44, "`path` must be a string, not integer", "Write `path` as a string.");
    assert_eq!(read(text, "failing"), (None, vec![expected]));
}

#[test]
fn a_line_that_is_not_a_line_number_is_a_problem_at_it() {
    let fix = "Write the line of the finding the failing case must give, e.g. `line = 3`.";
    for raw in ["0", "-1", "\"3\"", "1.5"] {
        let text = format!("failing = {{ files = [\"src/lib.rs\"], path = \"src/lib.rs\", line = {raw} }}");
        // `failing = { files = ["src/lib.rs"], path = "src/lib.rs", line = ` is 64 chars, so the value is column 65.
        let expected = problem(65, &format!("`line` must be a line number of 1 or more, not {raw}"), fix);
        assert_eq!(read(&text, "failing"), (None, vec![expected]), "{raw}");
    }
}

#[test]
fn every_problem_of_a_case_is_reported() {
    let text = "failing = { files = [\"gone.rs\"], line = 0, rule = 1 }";
    let expected = vec![
        problem(44, "unknown key `rule` in failing", "Remove it, or use one of: files, path, line, params."),
        problem(
            22,
            "case file \"gone.rs\" of baseline rule \"tm-a\" is not carried by baseline set house@2",
            "Put it at crates/rules/baseline/house/2/cases/tm-a/failing/gone.rs and list it in the set's cases in \
             crates/rules/src/rule_file/house_2.rs, or correct files.",
        ),
        problem(11, "failing is missing `path`", "Add `path = \"...\"` to this failing."),
        problem(
            41,
            "`line` must be a line number of 1 or more, not 0",
            "Write the line of the finding the failing case must give, e.g. `line = 3`.",
        ),
    ];
    assert_eq!(read(text, "failing"), (None, expected));
}

#[test]
fn a_case_file_name_with_a_parent_part_is_one_problem_at_its_entry() {
    let text = "failing = { files = [\"src/lib.rs\", \"../x.rs\"], path = \"src/lib.rs\", line = 3 }";
    // `failing = { files = ["src/lib.rs", ` is 35 chars, so "../x.rs" is column 36; it is not also reported as not
    // carried.
    let expected = problem(
        36,
        "case file \"../x.rs\" of baseline rule \"tm-a\" must be a relative path inside its case folder",
        "Name the file relative to cases/tm-a/failing/, with no leading / or \\, no drive letter and no .. part.",
    );
    assert_eq!(read(text, "failing"), (None, vec![expected]));
}

#[test]
fn every_case_file_name_outside_the_case_folder_is_its_own_problem() {
    let text = "passing = { files = [\"/abs.rs\", \"src/../lib.rs\"] }";
    let fix = "Name the file relative to cases/tm-a/passing/, with no leading / or \\, no drive letter and no .. part.";
    // `passing = { files = [` is 21 chars and `"/abs.rs", ` is 11, so the entries are columns 22 and 33.
    let expected = vec![
        problem(
            22,
            "case file \"/abs.rs\" of baseline rule \"tm-a\" must be a relative path inside its case folder",
            fix,
        ),
        problem(
            33,
            "case file \"src/../lib.rs\" of baseline rule \"tm-a\" must be a relative path inside its case folder",
            fix,
        ),
    ];
    assert_eq!(read(text, "passing"), (None, expected));
}

#[test]
fn a_failing_path_that_is_not_one_of_its_files_is_a_problem_at_the_path() {
    // Cargo.toml is carried under failing, but this case does not list it.
    let text = "failing = { files = [\"src/lib.rs\"], path = \"Cargo.toml\", line = 3 }";
    // `failing = { files = ["src/lib.rs"], path = ` is 43 chars, so the path is column 44.
    let expected = problem(
        44,
        "failing path \"Cargo.toml\" of baseline rule \"tm-a\" is not one of its failing files",
        "Set path to one of the failing case's files, or list it in files.",
    );
    assert_eq!(read(text, "failing"), (None, vec![expected]));
}

#[test]
fn a_failing_path_listed_but_not_carried_is_only_the_not_carried_problem() {
    let text = "failing = { files = [\"src/lib.rs\", \"gone.rs\"], path = \"gone.rs\", line = 3 }";
    // `failing = { files = ["src/lib.rs", ` is 35 chars, so "gone.rs" is column 36.
    let expected = problem(
        36,
        "case file \"gone.rs\" of baseline rule \"tm-a\" is not carried by baseline set house@2",
        "Put it at crates/rules/baseline/house/2/cases/tm-a/failing/gone.rs and list it in the set's cases in \
         crates/rules/src/rule_file/house_2.rs, or correct files.",
    );
    assert_eq!(read(text, "failing"), (None, vec![expected]));
}

#[test]
fn a_failing_line_past_the_end_of_its_file_is_a_problem_at_the_line() {
    let text = "failing = { files = [\"src/lib.rs\"], path = \"src/lib.rs\", line = 4 }";
    // `failing = { files = ["src/lib.rs"], path = "src/lib.rs", line = ` is 64 chars, so the line is column 65.
    let expected = problem(
        65,
        "failing line 4 of baseline rule \"tm-a\" is past the end of \"src/lib.rs\", which has 3 lines",
        "Set line to the line of the finding in that file, or correct the case file.",
    );
    assert_eq!(read(text, "failing"), (None, vec![expected]));
}

#[test]
fn every_repeated_file_is_a_problem_at_the_repeat_and_only_the_first_is_looked_up() {
    let text = "passing = { files = [\"src/lib.rs\", \"gone.rs\", \"src/lib.rs\", \"gone.rs\"] }";
    // `passing = { files = [` is 21 chars; the entries take 14, 11 and 14 chars with their `, `, so they start at
    // columns 22, 36, 47 and 61.
    let expected = vec![
        problem(
            36,
            "case file \"gone.rs\" of baseline rule \"tm-a\" is not carried by baseline set house@2",
            "Put it at crates/rules/baseline/house/2/cases/tm-a/passing/gone.rs and list it in the set's cases in \
             crates/rules/src/rule_file/house_2.rs, or correct files.",
        ),
        problem(
            47,
            "case file \"src/lib.rs\" is listed twice in the passing case of baseline rule \"tm-a\"",
            "Remove the repeated entry from files.",
        ),
        problem(
            61,
            "case file \"gone.rs\" is listed twice in the passing case of baseline rule \"tm-a\"",
            "Remove the repeated entry from files.",
        ),
    ];
    assert_eq!(read(text, "passing"), (None, expected));
}
