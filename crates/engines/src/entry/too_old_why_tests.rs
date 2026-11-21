//! Unit tests for too_old_why.rs: with a version and without one.

use super::too_old_why;

#[test]
fn the_why_names_the_engine_its_version_and_the_check_then_the_engines_words() {
    let got = too_old_why("ruff", Some("0.17.4"), "ZZZ999", "error: invalid value ...");
    assert_eq!(got, "ruff 0.17.4 does not know the check `ZZZ999`: error: invalid value ...");
}

#[test]
fn a_version_not_read_is_unknown() {
    let got = too_old_why("swiftlint", None, "no_such_rule", "not a valid rule identifier");
    assert_eq!(got, "swiftlint (version unknown) does not know the check `no_such_rule`: not a valid rule identifier");
}
