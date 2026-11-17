//! `Kit::holds`: whether a file is in a kit.

use super::Kit;
use crate::RelPath;

impl Kit {
    /// True when `file` matches one of the kit's paths; a kit with no paths holds nothing.
    pub fn holds(&self, file: &RelPath) -> bool {
        self.scope.as_ref().is_some_and(|scope| scope.matches(file))
    }
}
