//! Unit tests for harness_skill_file.rs: the skill file is under the harness folder in the base given.

use std::path::{Path, PathBuf};

use super::Harness;

#[test]
fn the_skill_file_is_under_the_folder_in_base() {
    assert_eq!(
        Harness::Claude.skill_file(Path::new("/home/ann")),
        PathBuf::from("/home/ann/.claude/skills/cctao/SKILL.md")
    );
    assert_eq!(
        Harness::Cursor.skill_file(Path::new("/work/project")),
        PathBuf::from("/work/project/.cursor/skills/cctao/SKILL.md")
    );
}
