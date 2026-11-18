//! `Harness::folder`: where a harness keeps its user files.

use super::Harness;

impl Harness {
    /// The harness's folder, under HOME (or the project with `--location local`): `.claude`, `.codex` or `.cursor`.
    /// A harness is detected when this folder exists in HOME.
    pub fn folder(self) -> &'static str {
        match self {
            Harness::Claude => ".claude",
            Harness::Codex => ".codex",
            Harness::Cursor => ".cursor",
        }
    }
}
