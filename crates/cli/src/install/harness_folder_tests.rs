//! Unit tests for harness_folder.rs: each harness's folder.

use super::Harness;

#[test]
fn each_harness_has_its_dot_folder() {
    assert_eq!(Harness::Claude.folder(), ".claude");
    assert_eq!(Harness::Codex.folder(), ".codex");
    assert_eq!(Harness::Cursor.folder(), ".cursor");
}
