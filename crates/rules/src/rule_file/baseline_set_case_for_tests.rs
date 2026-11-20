//! ZUT for baseline_set_case_for.rs.

use super::baseline_set::BaselineSet;

const SET: BaselineSet = BaselineSet {
    name: "test",
    version: 1,
    text: "version = 1\n",
    // A YAML file at the path of a case file: case_for reads only `cases`.
    yaml: &[("cases/tm-a/passing/b.rs", "rule: yaml\n")],
    cases: &[("cases/tm-a/failing/src/a.rs", "fn first() {}\n"), ("cases/tm-a/passing/a.rs", "fn second() {}\n")],
};

#[test]
fn a_carried_path_gives_its_text() {
    assert_eq!(SET.case_for("cases/tm-a/failing/src/a.rs"), Some("fn first() {}\n"));
}

#[test]
fn the_second_entry_gives_its_own_text() {
    assert_eq!(SET.case_for("cases/tm-a/passing/a.rs"), Some("fn second() {}\n"));
}

#[test]
fn a_path_the_set_does_not_carry_gives_none() {
    assert_eq!(SET.case_for("cases/tm-a/failing/missing.rs"), None);
}

#[test]
fn a_near_miss_path_is_not_found() {
    for path in [
        "./cases/tm-a/failing/src/a.rs",
        "cases/tm-a/failing/./src/a.rs",
        "cases/tm-a/failing/src/a.rs ",
        "cases/tm-a/passing/src/a.rs",
        "src/a.rs",
        "CASES/tm-a/failing/src/a.rs",
    ] {
        assert_eq!(SET.case_for(path), None, "{path}");
    }
}

#[test]
fn a_yaml_file_at_the_path_is_not_a_case_file() {
    assert_eq!(SET.case_for("cases/tm-a/passing/b.rs"), None);
}

#[test]
fn a_path_listed_twice_gives_the_first_entrys_text() {
    let twice = BaselineSet { cases: &[("cases/tm-x/failing/x.rs", "one\n"), ("cases/tm-x/failing/x.rs", "two\n")], ..SET };
    assert_eq!(twice.case_for("cases/tm-x/failing/x.rs"), Some("one\n"));
}

#[test]
fn a_set_carrying_nothing_gives_none() {
    let empty = BaselineSet { cases: &[], ..SET };
    assert_eq!(empty.case_for("cases/tm-a/failing/src/a.rs"), None);
}
