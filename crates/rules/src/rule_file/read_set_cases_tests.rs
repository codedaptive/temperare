//! ZUT for read_set_cases.rs.

use super::baseline_set::BaselineSet;
use super::read_set_cases::read_set_cases;
use crate::{CaseFile, ExpectedFinding, Problem, RuleCases};
use toml::de::DeTable;

const SET: BaselineSet = BaselineSet {
    name: "house",
    version: 2,
    ready: true,
    text: "",
    yaml: &[],
    cases: &[
        ("cases/tm-a/failing/a.rs", "a\nfailing\n"),
        ("cases/tm-a/passing/a.rs", "a passing\n"),
        ("cases/tm-b/failing/b.rs", B_FAILING),
        ("cases/tm-b/passing/b.rs", "b passing\n"),
    ],
};

/// tm-b's carried failing file: nine lines, the finding on the last.
const B_FAILING: &str = "b\nfailing\n3\n4\n5\n6\n7\n8\nfinding\n";

/// Well-formed cases of `tm-a` and of `tm-b`; tm-a's finding is line 2 of its two-line a.rs, tm-b's line 9 of
/// its nine-line b.rs.
const CASES_A: &str = "failing = { files = [\"a.rs\"], path = \"a.rs\", line = 2 }\npassing = { files = [\"a.rs\"] }\n";
const CASES_B: &str = "failing = { files = [\"b.rs\"], path = \"b.rs\", line = 9 }\npassing = { files = [\"b.rs\"] }\n";
const FAILING_B: &str = "failing = { files = [\"b.rs\"], path = \"b.rs\", line = 9 }\n";

/// A `[[rule]]` of five lines with this `id` value, then `cases`.
fn rule(id: &str, cases: &str) -> String {
    format!("[[rule]]\nid = {id}\nengine = \"ruff\"\ncheck = \"E1\"\nfix = \"f\"\n{cases}")
}

fn read(text: &str) -> (Vec<(String, RuleCases)>, Vec<Problem>) {
    let doc = DeTable::parse(text).expect("valid TOML");
    let mut out = Vec::new();
    let found = read_set_cases(text, doc.get_ref(), &SET, &mut out);
    (found, out)
}

fn file(path: &str, text: &'static str) -> CaseFile {
    CaseFile { path: path.to_string(), text }
}

fn cases_a() -> (String, RuleCases) {
    let expected = ExpectedFinding { rule_id: "tm-a".to_string(), path: "a.rs".to_string(), line: 2 };
    let failing = vec![file("a.rs", "a\nfailing\n")];
    ("tm-a".to_string(), RuleCases { failing, passing: vec![file("a.rs", "a passing\n")], expected })
}

/// tm-b has no failing case, its header at `line`.
fn missing_failing(line: usize) -> Problem {
    Problem {
        line,
        col: 1,
        what: "baseline rule \"tm-b\" has no failing case".to_string(),
        fix: "Add failing = { files = [...], path = \"...\", line = N } to the rule, and put its files in \
              crates/rules/baseline/house/2/cases/tm-b/failing/, listed in the set's cases in \
              crates/rules/src/rule_file/house_2.rs."
            .to_string(),
    }
}

fn missing_passing(line: usize, id: &str) -> Problem {
    Problem {
        line,
        col: 1,
        what: format!("baseline rule \"{id}\" has no passing case"),
        fix: format!(
            "Add passing = {{ files = [...] }} to the rule, and put its files in crates/rules/baseline/house/2/cases/\
             {id}/passing/, listed in the set's cases in crates/rules/src/rule_file/house_2.rs."
        ),
    }
}

#[test]
fn every_rule_gives_its_id_and_cases_in_file_order() {
    let text = format!("version = 1\n{}{}", rule("\"tm-a\"", CASES_A), rule("\"tm-b\"", CASES_B));
    let expected_b = ExpectedFinding { rule_id: "tm-b".to_string(), path: "b.rs".to_string(), line: 9 };
    let b = RuleCases {
        failing: vec![file("b.rs", "b\nfailing\n3\n4\n5\n6\n7\n8\nfinding\n")],
        passing: vec![file("b.rs", "b passing\n")],
        expected: expected_b,
    };
    assert_eq!(read(&text), (vec![cases_a(), ("tm-b".to_string(), b)], vec![]));
}

#[test]
fn a_rule_with_no_cases_is_two_problems_at_its_header_and_is_left_out() {
    let text = format!("version = 1\n{}{}", rule("\"tm-a\"", CASES_A), rule("\"tm-b\"", ""));
    // tm-a's [[rule]] is lines 2-8, so tm-b's header is line 9.
    assert_eq!(read(&text), (vec![cases_a()], vec![missing_failing(9), missing_passing(9, "tm-b")]));
}

#[test]
fn a_rule_with_no_failing_case_is_that_problem() {
    let text = format!("version = 1\n{}", rule("\"tm-b\"", "passing = { files = [\"b.rs\"] }\n"));
    assert_eq!(read(&text), (vec![], vec![missing_failing(2)]));
}

#[test]
fn every_rule_without_a_case_is_its_own_problem() {
    let failing_a = "failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\n";
    let text = format!("version = 1\n{}{}", rule("\"tm-a\"", failing_a), rule("\"tm-b\"", FAILING_B));
    // Each rule is six lines, so the headers are lines 2 and 8.
    assert_eq!(read(&text), (vec![], vec![missing_passing(2, "tm-a"), missing_passing(8, "tm-b")]));
}

#[test]
fn a_problem_in_a_case_leaves_its_rule_out() {
    let text = format!(
        "version = 1\n{}{}",
        rule("\"tm-a\"", CASES_A),
        rule("\"tm-b\"", &format!("{FAILING_B}passing = {{ files = [\"gone.rs\"] }}\n"))
    );
    // tm-b's passing is line 15; `passing = { files = [` is 21 chars, so "gone.rs" is column 22.
    let expected = Problem {
        line: 15,
        col: 22,
        what: "case file \"gone.rs\" of baseline rule \"tm-b\" is not carried by baseline set house@2".to_string(),
        fix: "Put it at crates/rules/baseline/house/2/cases/tm-b/passing/gone.rs and list it in the set's cases in \
              crates/rules/src/rule_file/house_2.rs, or correct files."
            .to_string(),
    };
    assert_eq!(read(&text), (vec![cases_a()], vec![expected]));
}

#[test]
fn rules_read_rules_reports_give_nothing_here() {
    let texts = [
        format!("version = 1\n{}", rule("1", "")),
        "version = 1\n[[rule]]\nengine = \"ruff\"\n".to_string(),
        "version = 1\nrule = \"tm-a\"\n".to_string(),
        "version = 1\nrule = [1]\n".to_string(),
        "version = 1\n".to_string(),
    ];
    for text in &texts {
        assert_eq!(read(text), (vec![], vec![]), "{text}");
    }
}
