//! ZUT for rule_file_parse.rs: the failing and passing cases a taken baseline set carries for its rules.

use super::baseline_set::BaselineSet;
use super::rule_file_parse::parse_with_sets;
use crate::{CaseFile, ExpectedFinding, Problem, RuleCases, RuleFile, Severity};
use std::path::Path;

/// `kit@1`'s rule `tm-k1` has well-formed cases; `nocase@1`'s `tm-m1` has no failing case; `nofile@1`'s `tm-f1` lists
/// a failing file the set does not carry; `two@1` has both problems, on `tm-t1` and `tm-t2`.
const SETS: &[BaselineSet] = &[
    BaselineSet {
        name: "kit",
        version: 1,
        text: "version = 1\n[[rule]]\nid = \"tm-k1\"\nengine = \"ruff\"\ncheck = \"E1\"\nfix = \"f\"\n\
               failing = { files = [\"src/lib.rs\", \"Cargo.toml\"], path = \"src/lib.rs\", line = 3 }\n\
               passing = { files = [\"src/lib.rs\"] }\n",
        yaml: &[],
        cases: &[
            ("cases/tm-k1/failing/src/lib.rs", "fn f(x: Option<u8>) {\n\n    x.unwrap();\n}\n"),
            ("cases/tm-k1/failing/Cargo.toml", "[package]\nname = \"case\"\n"),
            ("cases/tm-k1/passing/src/lib.rs", "fn f() {}\n"),
        ],
    },
    BaselineSet {
        name: "nocase",
        version: 1,
        text: "version = 1\n[[rule]]\nid = \"tm-m1\"\nengine = \"ruff\"\ncheck = \"E1\"\nfix = \"f\"\n\
               passing = { files = [\"a.rs\"] }\n",
        yaml: &[],
        cases: &[("cases/tm-m1/passing/a.rs", "x\n")],
    },
    BaselineSet {
        name: "nofile",
        version: 1,
        text: "version = 1\n[[rule]]\nid = \"tm-f1\"\nengine = \"ruff\"\ncheck = \"E1\"\nfix = \"f\"\n\
               failing = { files = [\"gone.rs\"], path = \"gone.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
        yaml: &[],
        cases: &[("cases/tm-f1/passing/a.rs", "x\n")],
    },
    BaselineSet {
        name: "two",
        version: 1,
        text: "version = 1\n[[rule]]\nid = \"tm-t1\"\nengine = \"ruff\"\ncheck = \"E1\"\nfix = \"f\"\n\
               failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\n\
               [[rule]]\nid = \"tm-t2\"\nengine = \"ruff\"\ncheck = \"E2\"\nfix = \"f\"\n\
               failing = { files = [\"gone.rs\"], path = \"gone.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
        yaml: &[],
        cases: &[("cases/tm-t1/failing/a.rs", "x\n"), ("cases/tm-t2/passing/a.rs", "x\n")],
    },
];

const TAKES_KIT: &str = "version = 1\nbaseline = [\"kit@1\"]\n";
const SET_FAULT_FIX: &str = "This is a fault in tm's embedded set; report it with this message.";

fn valid(text: &str) -> RuleFile {
    parse_with_sets(text, Path::new("."), SETS).unwrap_or_else(|p| panic!("valid, but: {p:?}"))
}

fn problems(text: &str) -> Vec<Problem> {
    parse_with_sets(text, Path::new("."), SETS).expect_err("the set has a problem")
}

/// `tm-k1`'s cases, written out from kit@1 above.
fn kit_cases() -> RuleCases {
    let file = |path: &str, text: &'static str| CaseFile { path: path.to_string(), text };
    RuleCases {
        failing: vec![
            file("src/lib.rs", "fn f(x: Option<u8>) {\n\n    x.unwrap();\n}\n"),
            file("Cargo.toml", "[package]\nname = \"case\"\n"),
        ],
        passing: vec![file("src/lib.rs", "fn f() {}\n")],
        expected: ExpectedFinding { rule_id: "tm-k1".to_string(), path: "src/lib.rs".to_string(), line: 3 },
    }
}

/// The Problem at the entry 2:13 for the set problem `inner` ("L:C: what") of `label`.
fn at_entry(label: &str, inner: &str) -> Problem {
    Problem {
        line: 2,
        col: 13,
        what: format!("baseline set {label} has a problem at {inner}"),
        fix: SET_FAULT_FIX.to_string(),
    }
}

#[test]
fn a_set_rule_gives_its_whole_cases() {
    let file = valid(TAKES_KIT);
    assert_eq!(file.rules[0].id, "tm-k1");
    assert_eq!(file.rules[0].cases(), Some(&kit_cases()));
}

#[test]
fn the_files_own_rule_has_no_cases() {
    let file = valid(&format!("{TAKES_KIT}[[rule]]\nid = \"own\"\nengine = \"ruff\"\ncheck = \"E1\"\nfix = \"f\"\n"));
    assert_eq!(file.rules[1].id, "own");
    assert_eq!(file.rules[1].cases(), None);
}

#[test]
fn an_override_keeps_the_cases() {
    let file = valid(&format!("{TAKES_KIT}[[override]]\nrule = \"tm-k1\"\nseverity = \"warning\"\n"));
    assert_eq!(file.rules[0].severity, Severity::Warning);
    assert_eq!(file.rules[0].cases(), Some(&kit_cases()));
}

#[test]
fn a_set_rule_with_no_failing_case_is_a_problem_at_its_baseline_entry() {
    // In the set, tm-m1's [[rule]] header is 2:1.
    let expected = at_entry("nocase@1", "2:1: baseline rule \"tm-m1\" has no failing case");
    assert_eq!(problems("version = 1\nbaseline = [\"nocase@1\"]\n"), vec![expected]);
}

#[test]
fn a_case_file_the_set_does_not_carry_is_a_problem_at_its_baseline_entry() {
    // In the set, the failing case is line 7 and `failing = { files = [` is 21 chars, so "gone.rs" is column 22.
    let inner = "7:22: case file \"gone.rs\" of baseline rule \"tm-f1\" is not carried by baseline set nofile@1";
    assert_eq!(problems("version = 1\nbaseline = [\"nofile@1\"]\n"), vec![at_entry("nofile@1", inner)]);
}

#[test]
fn every_case_problem_of_a_set_is_reported() {
    // tm-t1 is lines 2-7 with no passing case; tm-t2's failing case is line 13, "gone.rs" at column 22.
    let expected = vec![
        at_entry("two@1", "2:1: baseline rule \"tm-t1\" has no passing case"),
        at_entry("two@1", "13:22: case file \"gone.rs\" of baseline rule \"tm-t2\" is not carried by baseline set two@1"),
    ];
    assert_eq!(problems("version = 1\nbaseline = [\"two@1\"]\n"), expected);
}
