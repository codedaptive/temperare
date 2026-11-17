//! `Scope::matches`: whether a file is in scope.

use super::Scope;
use crate::RelPath;

impl Scope {
    /// True when `file` is in the paths (or the paths are empty), not excluded, and in the kit union when kits are set.
    pub fn matches(&self, file: &RelPath) -> bool {
        let path = file.as_str();
        self.include.as_ref().is_none_or(|set| set.is_match(path))
            && !self.exclude.is_match(path)
            && self.kits.as_ref().is_none_or(|set| set.is_match(path))
    }
}
