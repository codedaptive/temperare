//! `Harness::ALL`: every harness.

use super::Harness;

impl Harness {
    /// Every harness, in the order they are listed and offered.
    pub const ALL: [Harness; 3] = [Harness::Claude, Harness::Codex, Harness::Cursor];
}
