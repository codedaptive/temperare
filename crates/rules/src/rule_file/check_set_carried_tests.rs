//! ZUT for check_set_carried.rs.

use super::baseline_set::BaselineSet;
use super::check_set_carried::check_set_carried;
use crate::Problem;
use toml::de::DeTable;

/// `tm-a`, an ast-grep rule with well-formed cases; `tm-b`, a clippy rule whose failing case has a bad line and an
/// unknown key, and which has no passing case.
const TEXT: &str = "version = 1\n\
                    [[rule]]\nid = \"tm-a\"\nengine = \"ast-grep\"\ncheck = \"a.yml\"\nfix = \"f\"\n\
                    failing = { files = [\"a.rs\", \"src/x.rs\"], path = \"a.rs\", line = 1 }\n\
                    passing = { files = [\"a.rs\"] }\n\
                    [[rule]]\nid = \"tm-b\"\nengine = \"clippy\"\ncheck = \"b.yml\"\nfix = \"f\"\n\
                    failing = { files = [\"b.rs\"], path = \"b.rs\", line = 0, colour = 1 }\n";

/// Carries every file TEXT's rules name.
const NAMED: BaselineSet = BaselineSet {
    name: "house",
    version: 2,
    ready: true,
    text: TEXT,
    yaml: &[("a.yml", "rule: a\n")],
    cases: &[
        ("cases/tm-a/failing/a.rs", "x\n"),
        ("cases/tm-a/failing/src/x.rs", "x\n"),
        ("cases/tm-a/passing/a.rs", "x\n"),
        ("cases/tm-b/failing/b.rs", "x\n"),
    ],
};

fn check(set: &BaselineSet) -> Vec<Problem> {
    let doc = DeTable::parse(set.text).expect("valid TOML");
    let mut out = Vec::new();
    check_set_carried(set.text, doc.get_ref(), set, &mut out);
    out
}

fn unnamed(what: &str, list: &str) -> Problem {
    Problem {
        line: 1,
        col: 1,
        what: what.to_string(),
        fix: format!(
            "Remove it from the set's {list} in crates/rules/src/rule_file/house_2.rs and from \
             crates/rules/baseline/house/2/, or name it in a rule."
        ),
    }
}

#[test]
fn a_set_whose_rules_name_every_carried_file_gives_nothing_even_with_a_bad_case() {
    assert_eq!(check(&NAMED), vec![]);
}

#[test]
fn every_unnamed_file_is_a_problem_at_1_1_cases_then_yaml_in_the_sets_order() {
    let set = BaselineSet {
        yaml: &[("b.yml", "rule: b\n"), ("a.yml", "rule: a\n"), ("rust/c.yml", "rule: c\n")],
        cases: &[
            ("cases/tm-a/failing/a.rs", "x\n"),
            // Listed by tm-a's failing case only, so not named under passing.
            ("cases/tm-a/passing/src/x.rs", "x\n"),
            ("cases/tm-c/failing/a.rs", "x\n"),
            ("cases/tm-a/failing/src/x.rs", "x\n"),
            // tm-b's failing case has Problems of its own, but still names b.rs.
            ("cases/tm-b/failing/b.rs", "x\n"),
            ("cases/tm-a/failing/x.rs", "x\n"),
        ],
        ..NAMED
    };
    let expected = vec![
        unnamed("baseline set house@2 carries case file \"cases/tm-a/passing/src/x.rs\" that no rule names", "cases"),
        unnamed("baseline set house@2 carries case file \"cases/tm-c/failing/a.rs\" that no rule names", "cases"),
        unnamed("baseline set house@2 carries case file \"cases/tm-a/failing/x.rs\" that no rule names", "cases"),
        // tm-b's check names b.yml, but tm-b is not an ast-grep rule.
        unnamed("baseline set house@2 carries YAML file \"b.yml\" that no rule names", "yaml"),
        unnamed("baseline set house@2 carries YAML file \"rust/c.yml\" that no rule names", "yaml"),
    ];
    assert_eq!(check(&set), expected);
}

#[test]
fn a_set_with_no_rules_names_nothing_it_carries() {
    let set = BaselineSet { text: "version = 1\n", ..NAMED };
    let expected = vec![
        unnamed("baseline set house@2 carries case file \"cases/tm-a/failing/a.rs\" that no rule names", "cases"),
        unnamed("baseline set house@2 carries case file \"cases/tm-a/failing/src/x.rs\" that no rule names", "cases"),
        unnamed("baseline set house@2 carries case file \"cases/tm-a/passing/a.rs\" that no rule names", "cases"),
        unnamed("baseline set house@2 carries case file \"cases/tm-b/failing/b.rs\" that no rule names", "cases"),
        unnamed("baseline set house@2 carries YAML file \"a.yml\" that no rule names", "yaml"),
    ];
    assert_eq!(check(&set), expected);
}
