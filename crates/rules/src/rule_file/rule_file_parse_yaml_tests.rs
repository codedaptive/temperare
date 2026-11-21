//! ZUT for rule_file_parse.rs: the ast-grep YAML a taken baseline set carries for its rules.

use super::baseline_set::BaselineSet;
use super::rule_file_parse::parse_with_sets;
use crate::{Problem, RuleFile, Severity};
use std::path::Path;

/// `kit@1` carries the YAML of its ast-grep rule `tm-y1`; `gap@3`'s ast-grep rule `tm-g1` names a file it does not
/// carry.
const SETS: &[BaselineSet] = &[
    BaselineSet {
        name: "kit",
        version: 1,
        ready: true,
        text: "version = 1\n[[rule]]\nid = \"tm-y1\"\nengine = \"ast-grep\"\ncheck = \"no-unwrap.yml\"\nfix = \"f\"\n\
               failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
        yaml: &[("no-unwrap.yml", "id: no-unwrap\nlanguage: rust\nrule:\n  pattern: $X.unwrap()\n")],
        cases: &[("cases/tm-y1/failing/a.rs", "x\n"), ("cases/tm-y1/passing/a.rs", "x\n")],
    },
    BaselineSet {
        name: "gap",
        version: 3,
        ready: true,
        text: "version = 1\n[[rule]]\nid = \"tm-g1\"\nengine = \"ast-grep\"\ncheck = \"gone.yml\"\nfix = \"f\"\n\
               failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
        yaml: &[],
        cases: &[("cases/tm-g1/failing/a.rs", "x\n"), ("cases/tm-g1/passing/a.rs", "x\n")],
    },
];

const TAKES_KIT: &str = "version = 1\nbaseline = [\"kit@1\"]\n";

fn valid(text: &str) -> RuleFile {
    parse_with_sets(text, Path::new("."), SETS).unwrap_or_else(|p| panic!("valid, but: {p:?}"))
}

#[test]
fn a_set_ast_grep_rule_gives_the_carried_yaml_and_keeps_its_check() {
    let file = valid(TAKES_KIT);
    assert_eq!(file.rules[0].id, "tm-y1");
    assert_eq!(file.rules[0].check, "no-unwrap.yml");
    assert_eq!(file.rules[0].embedded_yaml(), Some("id: no-unwrap\nlanguage: rust\nrule:\n  pattern: $X.unwrap()\n"));
}

#[test]
fn an_override_changes_neither_check_nor_the_yaml() {
    let file = valid(&format!("{TAKES_KIT}[[override]]\nrule = \"tm-y1\"\nseverity = \"warning\"\n"));
    let rule = &file.rules[0];
    assert_eq!(rule.severity, Severity::Warning);
    assert_eq!(rule.check, "no-unwrap.yml");
    assert_eq!(rule.embedded_yaml(), Some("id: no-unwrap\nlanguage: rust\nrule:\n  pattern: $X.unwrap()\n"));
}

#[test]
fn the_files_own_ast_grep_rule_has_no_yaml_even_at_a_carried_path() {
    let own = "[[rule]]\nid = \"own\"\nengine = \"ast-grep\"\ncheck = \"no-unwrap.yml\"\nfix = \"f\"\n";
    let file = valid(&format!("{TAKES_KIT}{own}"));
    assert_eq!(file.rules[1].id, "own");
    assert_eq!(file.rules[1].embedded_yaml(), None);
}

#[test]
fn a_set_rule_naming_yaml_the_set_does_not_carry_is_a_problem_at_its_baseline_entry() {
    let found = parse_with_sets("version = 1\nbaseline = [\"gap@3\"]\n", Path::new("."), SETS).expect_err("missing YAML");
    // The entry: `baseline = [` is 12 chars, so "gap@3" starts at line 2, column 13. In the set, `check = "gone.yml"`
    // is line 5 and its value starts at column 9.
    let expected = Problem {
        line: 2,
        col: 13,
        what: "baseline set gap@3 has a problem at 5:9: check \"gone.yml\" names a YAML file that baseline set gap@3 \
               does not carry"
            .to_string(),
        fix: "This is a fault in tm's embedded set; report it with this message.".to_string(),
    };
    assert_eq!(found, vec![expected]);
}
