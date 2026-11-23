//! `Finding::line`: the finding's line, None when it has none.

use super::Finding;

impl Finding {
    /// The finding's line, None when it has none.
    pub fn line(&self) -> Option<u32> {
        self.line
    }
}
