//! `Finding::line`: the finding's line, None when it has none.

use super::Finding;

impl Finding {
    /// None when the stored line is 0, else the line. 0 is what `Finding::new` stores for None; the engines count
    /// lines from 1, though no adapter rejects a 0 an engine prints.
    pub fn line(&self) -> Option<u32> {
        (self.line != 0).then_some(self.line)
    }
}
