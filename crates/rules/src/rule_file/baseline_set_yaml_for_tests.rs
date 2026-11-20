//! ZUT for baseline_set_yaml_for.rs.

use super::baseline_set::BaselineSet;

const SET: BaselineSet = BaselineSet {
    name: "test",
    version: 1,
    text: "version = 1\n",
    yaml: &[("no-unwrap.yml", "rule: first\n"), ("swift/no-force.yml", "rule: second\n")],
    cases: &[],
};

#[test]
fn a_carried_path_gives_its_text() {
    assert_eq!(SET.yaml_for("no-unwrap.yml"), Some("rule: first\n"));
}

#[test]
fn the_second_entry_gives_its_own_text() {
    assert_eq!(SET.yaml_for("swift/no-force.yml"), Some("rule: second\n"));
}

#[test]
fn a_path_the_set_does_not_carry_gives_none() {
    assert_eq!(SET.yaml_for("missing.yml"), None);
}

#[test]
fn a_near_miss_path_is_not_found() {
    for check in ["./no-unwrap.yml", "no-unwrap", "no-unwrap.yml ", "NO-UNWRAP.yml", "no-force.yml"] {
        assert_eq!(SET.yaml_for(check), None, "{check}");
    }
}

#[test]
fn a_path_listed_twice_gives_the_first_entrys_text() {
    let twice = BaselineSet { yaml: &[("x.yml", "rule: one\n"), ("x.yml", "rule: two\n")], ..SET };
    assert_eq!(twice.yaml_for("x.yml"), Some("rule: one\n"));
}

#[test]
fn a_set_carrying_nothing_gives_none() {
    let empty = BaselineSet { yaml: &[], ..SET };
    assert_eq!(empty.yaml_for("no-unwrap.yml"), None);
}
