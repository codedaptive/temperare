//! `RuleFile::owner_for`: the zone that owns a file's findings.

use super::RuleFile;
use crate::RelPath;

impl RuleFile {
    /// The owner of the first kit, in file order, whose paths hold `file`; else the top-level owner. Read literally: a
    /// holding kit with no owner of its own gives the top-level owner, even when a later kit holding `file` has one.
    pub fn owner_for(&self, file: &RelPath) -> Option<&str> {
        self.kits
            .iter()
            .find(|kit| kit.holds(file))
            .and_then(|kit| kit.owner.as_deref())
            .or(self.owner.as_deref())
    }
}
