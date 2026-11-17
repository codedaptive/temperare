//! `Rule::applies`: whether a rule covers a file.

use super::Rule;
use crate::RelPath;

impl Rule {
    /// True when the rule's scope covers `file`.
    pub fn applies(&self, file: &RelPath) -> bool {
        self.scope.matches(file)
    }
}
