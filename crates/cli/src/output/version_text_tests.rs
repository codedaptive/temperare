//! Unit tests for version_text.rs: the long name, the version, then the commit in brackets.

use super::version_text;

#[test]
fn names_temperare_the_version_and_the_commit() {
    assert_eq!(version_text("0.1.0", "9c1955e0a1b2"), "temperare 0.1.0 (9c1955e0a1b2)");
}

#[test]
fn an_unknown_commit_is_said_as_such() {
    assert_eq!(version_text("2.3.4", "unknown"), "temperare 2.3.4 (unknown)");
}
