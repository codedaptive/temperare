//! `Harness::skill_file`: where install writes the temperare skill for a harness.

use std::path::{Path, PathBuf};

use super::Harness;

impl Harness {
    /// The skill file install writes for this harness: `<base>/<folder>/skills/temperare/SKILL.md`.
    pub fn skill_file(self, base: &Path) -> PathBuf {
        base.join(self.folder()).join("skills").join("temperare").join("SKILL.md")
    }
}
