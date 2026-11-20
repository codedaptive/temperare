//! Unit tests for harness_name.rs: each harness's `--target` name.

use super::Harness;

#[test]
fn each_harness_has_its_target_name() {
    assert_eq!(Harness::Claude.name(), "claude");
    assert_eq!(Harness::Codex.name(), "codex");
    assert_eq!(Harness::Cursor.name(), "cursor");
}
