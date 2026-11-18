//! `Harness::name`: the name `--target` uses for each harness.

use super::Harness;

impl Harness {
    /// The name `--target` takes and messages show: `claude`, `codex` or `cursor`.
    pub fn name(self) -> &'static str {
        match self {
            Harness::Claude => "claude",
            Harness::Codex => "codex",
            Harness::Cursor => "cursor",
        }
    }
}
