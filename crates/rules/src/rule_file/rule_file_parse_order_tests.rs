//! ZUT for rule_file_parse.rs, resolved order: each rule's `position`, and `rules_in_order` over a parsed file.

use super::baseline_set::BaselineSet;
use super::rule_file_parse::parse_with_sets;
use crate::{Rule, RuleFile, Severity};
use std::path::Path;

/// `one@1`: `tm-a1`, then `tm-a2`, which needs the param `types`, then `tm-a3`. `two@1`: `tm-b1`, then `tm-b2`.
const SETS: &[BaselineSet] = &[
    BaselineSet {
        name: "one",
        version: 1,
        ready: true,
        text: "version = 1\n\
               [[rule]]\nid = \"tm-a1\"\nengine = \"ruff\"\ncheck = \"a1\"\nfix = \"f\"\n\
               failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n\
               [[rule]]\nid = \"tm-a2\"\nengine = \"ruff\"\ncheck = \"a2\"\nfix = \"f\"\nrequired_params = [\"types\"]\n\
               failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n\
               [[rule]]\nid = \"tm-a3\"\nengine = \"ruff\"\ncheck = \"a3\"\nfix = \"f\"\n\
               failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
        yaml: &[],
        cases: &[
            ("cases/tm-a1/failing/a.rs", "x\n"),
            ("cases/tm-a1/passing/a.rs", "x\n"),
            ("cases/tm-a2/failing/a.rs", "x\n"),
            ("cases/tm-a2/passing/a.rs", "x\n"),
            ("cases/tm-a3/failing/a.rs", "x\n"),
            ("cases/tm-a3/passing/a.rs", "x\n"),
        ],
    },
    BaselineSet {
        name: "two",
        version: 1,
        ready: true,
        text: "version = 1\n\
               [[rule]]\nid = \"tm-b1\"\nengine = \"ruff\"\ncheck = \"b1\"\nfix = \"f\"\n\
               failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n\
               [[rule]]\nid = \"tm-b2\"\nengine = \"ruff\"\ncheck = \"b2\"\nfix = \"f\"\n\
               failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
        yaml: &[],
        cases: &[
            ("cases/tm-b1/failing/a.rs", "x\n"),
            ("cases/tm-b1/passing/a.rs", "x\n"),
            ("cases/tm-b2/failing/a.rs", "x\n"),
            ("cases/tm-b2/passing/a.rs", "x\n"),
        ],
    },
];

const OWN_RULE: &str = "[[rule]]\nid = \"own\"\nengine = \"ruff\"\ncheck = \"E1\"\nfix = \"f\"\n";

fn valid(text: &str) -> RuleFile {
    parse_with_sets(text, Path::new("."), SETS).unwrap_or_else(|p| panic!("valid, but: {p:?}"))
}

fn placed<'a>(rules: impl IntoIterator<Item = &'a Rule>) -> Vec<(String, usize)> {
    rules.into_iter().map(|rule| (rule.id.clone(), rule.position)).collect()
}

fn pairs(items: &[(&str, usize)]) -> Vec<(String, usize)> {
    items.iter().map(|(id, position)| (id.to_string(), *position)).collect()
}

#[test]
fn a_not_applicable_rule_falls_between_the_rules_in_force_around_it() {
    let file = valid("version = 1\nbaseline = [\"one@1\"]\n");
    assert_eq!(placed(file.rules_in_order()), pairs(&[("tm-a1", 0), ("tm-a2", 1), ("tm-a3", 2)]));
    assert_eq!(placed(&file.rules), pairs(&[("tm-a1", 0), ("tm-a3", 2)]));
    assert_eq!(placed(&file.not_applicable_rules), pairs(&[("tm-a2", 1)]));
}

#[test]
fn the_files_own_rules_come_after_the_baseline_rules() {
    // The own rules are written, as an inline array, before the `baseline` key.
    let text = "version = 1\n\
                rule = [{ id = \"own-1\", engine = \"ruff\", check = \"E1\", fix = \"f\" },\n\
                        { id = \"own-2\", engine = \"ruff\", check = \"E2\", fix = \"f\" }]\n\
                baseline = [\"one@1\"]\n";
    let file = valid(text);
    let expected = pairs(&[("tm-a1", 0), ("tm-a2", 1), ("tm-a3", 2), ("own-1", 3), ("own-2", 4)]);
    assert_eq!(placed(file.rules_in_order()), expected);
    assert_eq!(placed(&file.rules), pairs(&[("tm-a1", 0), ("tm-a3", 2), ("own-1", 3), ("own-2", 4)]));
    assert_eq!(placed(&file.not_applicable_rules), pairs(&[("tm-a2", 1)]));
}

#[test]
fn two_sets_follow_the_files_baseline_order() {
    // SETS holds one@1 before two@1; the file takes them the other way round.
    let file = valid("version = 1\nbaseline = [\"two@1\", \"one@1\"]\n");
    let expected = pairs(&[("tm-b1", 0), ("tm-b2", 1), ("tm-a1", 2), ("tm-a2", 3), ("tm-a3", 4)]);
    assert_eq!(placed(file.rules_in_order()), expected);
    assert_eq!(placed(&file.not_applicable_rules), pairs(&[("tm-a2", 3)]));
}

#[test]
fn an_override_does_not_move_a_rule() {
    // The override gives the middle rule tm-a2 its required param, so it is in force, and changes its severity.
    let text = "version = 1\nbaseline = [\"one@1\"]\n\
                [[override]]\nrule = \"tm-a2\"\nseverity = \"warning\"\nparams = { types = \"x\" }\n";
    let file = valid(text);
    let in_order = pairs(&[("tm-a1", 0), ("tm-a2", 1), ("tm-a3", 2)]);
    assert_eq!(placed(file.rules_in_order()), in_order);
    assert_eq!(placed(&file.rules), in_order);
    assert_eq!(placed(&file.not_applicable_rules), pairs(&[]));
    let severities: Vec<_> = file.rules.iter().map(|rule| (rule.id.as_str(), rule.position, rule.severity)).collect();
    let expected = vec![("tm-a1", 0, Severity::Error), ("tm-a2", 1, Severity::Warning), ("tm-a3", 2, Severity::Error)];
    assert_eq!(severities, expected);
}

#[test]
fn a_rule_turned_off_is_left_out_and_the_next_ones_close_up() {
    let text = format!("version = 1\nbaseline = [\"two@1\", \"one@1\"]\n[[override]]\nrule = \"tm-b2\"\noff = true\n{OWN_RULE}");
    let file = valid(&text);
    let expected = pairs(&[("tm-b1", 0), ("tm-a1", 1), ("tm-a2", 2), ("tm-a3", 3), ("own", 4)]);
    assert_eq!(placed(file.rules_in_order()), expected);
    assert_eq!(placed(&file.rules), pairs(&[("tm-b1", 0), ("tm-a1", 1), ("tm-a3", 3), ("own", 4)]));
    assert_eq!(placed(&file.not_applicable_rules), pairs(&[("tm-a2", 2)]));
}
