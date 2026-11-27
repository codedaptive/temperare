//! ZUT for version_text.rs: the long name before the version and commit.

use super::version_text;

#[test]
fn names_cctao_before_the_version_and_the_commit() {
    assert_eq!(version_text("0.1.0", "9c1955e0a1b2"), "cctao 0.1.0 (9c1955e0a1b2)");
}
