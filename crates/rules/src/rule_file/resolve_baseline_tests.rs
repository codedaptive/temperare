//! ZUT for resolve_baseline.rs, on sets written here.

use super::baseline_set::BaselineSet;
use super::override_entry::OverrideEntry;
use super::resolve_baseline::resolve_baseline;
use super::resolved_baseline::ResolvedBaseline;
use crate::Problem;
use std::collections::BTreeSet;

const ALPHA: BaselineSet = BaselineSet {
    name: "alpha",
    version: 1,
    ready: true,
    text: "version = 1\nretired = [\"tm-old-a\"]\n[[rule]]\nid = \"tm-a1\"\nengine = \"ruff\"\ncheck = \"a1\"\nfix = \"f\"\n\
           failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n\
           [[rule]]\nid = \"tm-a2\"\nengine = \"ruff\"\ncheck = \"a2\"\nfix = \"f\"\nrequired_params = [\"types\"]\n\
           failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
    yaml: &[],
    cases: &[
        ("cases/tm-a1/failing/a.rs", "x\n"),
        ("cases/tm-a1/passing/a.rs", "x\n"),
        ("cases/tm-a2/failing/a.rs", "x\n"),
        ("cases/tm-a2/passing/a.rs", "x\n"),
    ],
};
const BETA: BaselineSet = BaselineSet {
    name: "beta",
    version: 1,
    ready: true,
    text: "version = 1\nretired = [\"tm-old-b\"]\n[[rule]]\nid = \"tm-b1\"\nengine = \"clippy\"\ncheck = \"b1\"\nfix = \"f\"\n\
           failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
    yaml: &[],
    cases: &[("cases/tm-b1/failing/a.rs", "x\n"), ("cases/tm-b1/passing/a.rs", "x\n")],
};
/// Holds `tm-a1` as ALPHA does, with another check.
const DUP: BaselineSet = BaselineSet {
    name: "dup",
    version: 1,
    ready: true,
    text: "version = 1\n[[rule]]\nid = \"tm-d1\"\nengine = \"ruff\"\ncheck = \"d1\"\nfix = \"f\"\n\
           failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n\
           [[rule]]\nid = \"tm-a1\"\nengine = \"ruff\"\ncheck = \"from-dup\"\nfix = \"f\"\n\
           failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
    yaml: &[],
    cases: &[
        ("cases/tm-d1/failing/a.rs", "x\n"),
        ("cases/tm-d1/passing/a.rs", "x\n"),
        ("cases/tm-a1/failing/a.rs", "x\n"),
        ("cases/tm-a1/passing/a.rs", "x\n"),
    ],
};
const BROKEN: BaselineSet =
    BaselineSet { name: "broken", version: 2, ready: true, text: "version = 1\ncolour = 1\n", yaml: &[], cases: &[] };

/// Two entries: the first string starts at byte 12 (column 13), the second at byte 23 (column 24); the override's
/// rule string at byte 55 (line 3, column 8).
const TEXT: &str = "baseline = [\"first@1\", \"second@1\"]\n[[override]]\nrule = \"tm-x\"\n";

fn resolve(sets: &[&BaselineSet], overrides: &[OverrideEntry]) -> (ResolvedBaseline, Vec<Problem>) {
    let taken: Vec<(&BaselineSet, usize)> = sets.iter().copied().zip([12, 23]).collect();
    let mut out = Vec::new();
    let resolved = resolve_baseline(TEXT, &taken, overrides, &mut out);
    (resolved, out)
}

fn ids(rules: &[crate::Rule]) -> Vec<&str> {
    rules.iter().map(|rule| rule.id.as_str()).collect()
}

fn of(rule: &str) -> OverrideEntry {
    OverrideEntry { rule: rule.to_string(), at: 55, severity: None, off: false, paths: None, exclude: None, params: None }
}

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| item.to_string()).collect()
}

#[test]
fn sets_resolve_in_taken_order_with_retired_ids_and_not_applicable_rules() {
    let (resolved, out) = resolve(&[&BETA, &ALPHA], &[]);
    assert_eq!(out, vec![]);
    assert_eq!(resolved.taken, strings(&["beta@1", "alpha@1"]));
    assert_eq!(ids(&resolved.rules), vec!["tm-b1", "tm-a1"]);
    assert_eq!(resolved.rules.iter().map(|rule| rule.not_applicable.clone()).collect::<Vec<_>>(), vec![None, None]);
    assert_eq!(ids(&resolved.not_applicable), vec!["tm-a2"]);
    let reason = "required params not given: types; give them in the params of an [[override]] for \"tm-a2\"";
    assert_eq!(resolved.not_applicable[0].not_applicable.as_deref(), Some(reason));
    assert_eq!(resolved.retired, strings(&["tm-old-b", "tm-old-a"]));
    assert_eq!(resolved.ids, BTreeSet::from(["tm-a1".to_string(), "tm-a2".to_string(), "tm-b1".to_string()]));
}

#[test]
fn a_required_param_given_by_the_override_makes_the_rule_apply() {
    let mut params = toml::Table::new();
    params.insert("types".to_string(), toml::Value::Array(vec![toml::Value::String("Order".to_string())]));
    let (resolved, out) = resolve(&[&ALPHA], &[OverrideEntry { params: Some(params), ..of("tm-a2") }]);
    assert_eq!(out, vec![]);
    assert_eq!(ids(&resolved.rules), vec!["tm-a1", "tm-a2"]);
    assert_eq!(resolved.rules[1].not_applicable, None);
    assert_eq!(ids(&resolved.not_applicable), Vec::<&str>::new());
}

#[test]
fn an_override_turning_a_rule_off_removes_it_but_keeps_its_id() {
    let (resolved, out) = resolve(&[&ALPHA, &BETA], &[OverrideEntry { off: true, ..of("tm-a1") }]);
    assert_eq!(out, vec![]);
    assert_eq!(ids(&resolved.rules), vec!["tm-b1"]);
    assert_eq!(ids(&resolved.not_applicable), vec!["tm-a2"]);
    assert!(resolved.ids.contains("tm-a1"), "{:?}", resolved.ids);
}

#[test]
fn an_override_of_an_id_no_taken_set_holds_is_a_problem_at_its_rule_string() {
    let (resolved, out) = resolve(&[&BETA], &[of("tm-a1")]);
    let expected = Problem {
        line: 3,
        col: 8,
        what: "override of unknown baseline rule \"tm-a1\": no taken set holds it".to_string(),
        fix: "Name the id of a rule in a set this file takes in baseline, or remove the [[override]].".to_string(),
    };
    assert_eq!(out, vec![expected]);
    assert_eq!(ids(&resolved.rules), vec!["tm-b1"]);
}

#[test]
fn an_id_in_two_sets_is_a_problem_at_the_later_entry_and_the_first_copy_stays() {
    let (resolved, out) = resolve(&[&ALPHA, &DUP], &[]);
    let expected = Problem {
        line: 1,
        col: 24,
        what: "rule \"tm-a1\" is in both alpha@1 and dup@1".to_string(),
        fix: "Take only one of the two sets.".to_string(),
    };
    assert_eq!(out, vec![expected]);
    assert_eq!(ids(&resolved.rules), vec!["tm-a1", "tm-d1"]);
    assert_eq!(resolved.rules[0].check, "a1");
}

#[test]
fn a_set_with_its_own_problem_is_a_problem_at_its_entry_and_gives_nothing() {
    let (resolved, out) = resolve(&[&BROKEN, &BETA], &[]);
    let expected = Problem {
        line: 1,
        col: 13,
        what: "baseline set broken@2 has a problem at 2:1: unknown key `colour` in the top level".to_string(),
        fix: "This is a fault in tm's embedded set; report it with this message.".to_string(),
    };
    assert_eq!(out, vec![expected]);
    assert_eq!(ids(&resolved.rules), vec!["tm-b1"]);
    assert_eq!(resolved.ids, BTreeSet::from(["tm-b1".to_string()]));
}

/// One security rule, `tm-x1`.
const SEC: BaselineSet = BaselineSet {
    name: "sec",
    version: 1,
    ready: true,
    text: "version = 1\n[[rule]]\nid = \"tm-x1\"\nengine = \"tessera\"\nkind = \"security\"\ncheck = \"x1\"\nfix = \"f\"\n\
           failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
    yaml: &[],
    cases: &[("cases/tm-x1/failing/a.rs", "x\n"), ("cases/tm-x1/passing/a.rs", "x\n")],
};
/// Another security rule, `tm-y1`.
const SEC2: BaselineSet = BaselineSet {
    name: "sec2",
    version: 1,
    ready: true,
    text: "version = 1\n[[rule]]\nid = \"tm-y1\"\nengine = \"tessera\"\nkind = \"security\"\ncheck = \"y1\"\nfix = \"f\"\n\
           failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
    yaml: &[],
    cases: &[("cases/tm-y1/failing/a.rs", "x\n"), ("cases/tm-y1/passing/a.rs", "x\n")],
};
/// A security rule, `tm-n1`, that needs the param `types`.
const SEC_NEEDS: BaselineSet = BaselineSet {
    name: "secneeds",
    version: 1,
    ready: true,
    text: "version = 1\n[[rule]]\nid = \"tm-n1\"\nengine = \"tessera\"\nkind = \"security\"\ncheck = \"n1\"\nfix = \"f\"\n\
           required_params = [\"types\"]\n\
           failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
    yaml: &[],
    cases: &[("cases/tm-n1/failing/a.rs", "x\n"), ("cases/tm-n1/passing/a.rs", "x\n")],
};

#[test]
fn no_security_rule_in_force_is_no_security_entry() {
    let (resolved, out) = resolve(&[&ALPHA, &BETA], &[]);
    assert_eq!(out, vec![]);
    assert_eq!(resolved.security_at, None);
}

#[test]
fn a_security_rule_in_the_second_set_is_at_the_second_entry() {
    let (resolved, out) = resolve(&[&BETA, &SEC], &[]);
    assert_eq!(out, vec![]);
    assert_eq!(resolved.security_at, Some(23));
}

#[test]
fn a_security_rule_turned_off_is_not_in_force() {
    let (resolved, out) = resolve(&[&SEC, &BETA], &[OverrideEntry { off: true, ..of("tm-x1") }]);
    assert_eq!(out, vec![]);
    assert_eq!(ids(&resolved.rules), vec!["tm-b1"]);
    assert_eq!(resolved.security_at, None);
}

#[test]
fn a_security_rule_turned_off_leaves_the_next_sets_security_rule() {
    let (resolved, out) = resolve(&[&SEC, &SEC2], &[OverrideEntry { off: true, ..of("tm-x1") }]);
    assert_eq!(out, vec![]);
    assert_eq!(resolved.security_at, Some(23));
}

#[test]
fn a_security_rule_not_applicable_is_not_in_force() {
    let (resolved, out) = resolve(&[&SEC_NEEDS, &BETA], &[]);
    assert_eq!(out, vec![]);
    assert_eq!(ids(&resolved.not_applicable), vec!["tm-n1"]);
    assert_eq!(resolved.security_at, None);
}

#[test]
fn two_sets_with_security_rules_give_the_first_entry() {
    let (resolved, out) = resolve(&[&SEC, &SEC2], &[]);
    assert_eq!(out, vec![]);
    assert_eq!(resolved.security_at, Some(12));
}

/// A rule `tm-t1` that requires `types` and carries a default for it in its own params; the set carries its YAML.
const DEFAULTED: BaselineSet = BaselineSet {
    name: "defaulted",
    version: 1,
    ready: true,
    text: "version = 1\n[[rule]]\nid = \"tm-t1\"\nengine = \"ast-grep\"\ncheck = \"t1\"\nfix = \"f\"\n\
           params = { types = [\"X\"] }\nrequired_params = [\"types\"]\n\
           failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
    yaml: &[("t1", "rule: t1\n")],
    cases: &[("cases/tm-t1/failing/a.rs", "x\n"), ("cases/tm-t1/passing/a.rs", "x\n")],
};
/// Retires `tm-b1`, which BETA gives.
const RETIRES_B1: BaselineSet =
    BaselineSet { name: "ret", version: 1, ready: true, text: "version = 1\nretired = [\"tm-b1\"]\n", yaml: &[], cases: &[] };

#[test]
fn a_set_default_does_not_give_a_required_param() {
    let (resolved, out) = resolve(&[&DEFAULTED], &[]);
    assert_eq!(out, vec![]);
    assert_eq!(ids(&resolved.rules), Vec::<&str>::new());
    assert_eq!(ids(&resolved.not_applicable), vec!["tm-t1"]);
    let reason = "required params not given: types; give them in the params of an [[override]] for \"tm-t1\"";
    assert_eq!(resolved.not_applicable[0].not_applicable.as_deref(), Some(reason));
}

#[test]
fn an_id_retired_by_the_first_set_and_given_by_the_second_is_a_problem_at_the_first_entry() {
    let (_, out) = resolve(&[&RETIRES_B1, &BETA], &[]);
    let expected = Problem {
        line: 1,
        col: 13,
        what: "retired id \"tm-b1\" in ret@1 is a rule in beta@1".to_string(),
        fix: "Take only one of the two sets: a retired id is never given again.".to_string(),
    };
    assert_eq!(out, vec![expected]);
}

#[test]
fn an_id_given_by_the_first_set_and_retired_by_the_second_is_a_problem_at_the_second_entry() {
    let (_, out) = resolve(&[&BETA, &RETIRES_B1], &[]);
    let expected = Problem {
        line: 1,
        col: 24,
        what: "retired id \"tm-b1\" in ret@1 is a rule in beta@1".to_string(),
        fix: "Take only one of the two sets: a retired id is never given again.".to_string(),
    };
    assert_eq!(out, vec![expected]);
}

/// `tm-z1` as a quality rule.
const QZ: BaselineSet = BaselineSet {
    name: "qz",
    version: 1,
    ready: true,
    text: "version = 1\n[[rule]]\nid = \"tm-z1\"\nengine = \"ruff\"\ncheck = \"z1\"\nfix = \"f\"\n\
           failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
    yaml: &[],
    cases: &[("cases/tm-z1/failing/a.rs", "x\n"), ("cases/tm-z1/passing/a.rs", "x\n")],
};
/// `tm-z1` as a security rule.
const SZ: BaselineSet = BaselineSet {
    name: "sz",
    version: 1,
    ready: true,
    text: "version = 1\n[[rule]]\nid = \"tm-z1\"\nengine = \"tessera\"\nkind = \"security\"\ncheck = \"z1\"\nfix = \"f\"\n\
           failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
    yaml: &[],
    cases: &[("cases/tm-z1/failing/a.rs", "x\n"), ("cases/tm-z1/passing/a.rs", "x\n")],
};

#[test]
fn a_security_copy_dropped_as_a_duplicate_is_not_in_force() {
    let (resolved, out) = resolve(&[&QZ, &SZ], &[]);
    let expected = Problem {
        line: 1,
        col: 24,
        what: "rule \"tm-z1\" is in both qz@1 and sz@1".to_string(),
        fix: "Take only one of the two sets.".to_string(),
    };
    assert_eq!(out, vec![expected]);
    assert_eq!(ids(&resolved.rules), vec!["tm-z1"]);
    assert_eq!(resolved.security_at, None);
}

fn placed(rules: &[crate::Rule]) -> Vec<(&str, usize)> {
    rules.iter().map(|rule| (rule.id.as_str(), rule.position)).collect()
}

#[test]
fn set_rules_are_numbered_in_taken_order_with_not_applicable_rules_in_their_place() {
    let (resolved, out) = resolve(&[&BETA, &ALPHA], &[]);
    assert_eq!(out, vec![]);
    assert_eq!(placed(&resolved.rules), vec![("tm-b1", 0), ("tm-a1", 1)]);
    assert_eq!(placed(&resolved.not_applicable), vec![("tm-a2", 2)]);
}

#[test]
fn a_rule_turned_off_takes_no_number() {
    let (resolved, out) = resolve(&[&ALPHA, &BETA], &[OverrideEntry { off: true, ..of("tm-a1") }]);
    assert_eq!(out, vec![]);
    assert_eq!(placed(&resolved.rules), vec![("tm-b1", 1)]);
    assert_eq!(placed(&resolved.not_applicable), vec![("tm-a2", 0)]);
}
