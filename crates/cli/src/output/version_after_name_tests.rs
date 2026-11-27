//! Unit test for version_after_name.rs: the version, then the commit in brackets.

use super::version_after_name;

#[test]
fn the_version_then_the_commit_in_brackets() {
    let rows = [("0.1.0", "abc1234", "0.1.0 (abc1234)"), ("1.2.3", "unknown", "1.2.3 (unknown)")];
    for (version, commit, expected) in rows {
        assert_eq!(version_after_name(version, commit), expected, "row ({version}, {commit})");
    }
}
