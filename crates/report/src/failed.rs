//! `Failed`: a rule whose engine ran and failed.

use rules::Engine;
use serde::Serialize;

/// A rule whose engine ran and failed. `code` is always serialized, `null` when the engine had no exit code (S-18).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Failed {
    pub rule: String,
    pub engine: Engine,
    pub code: Option<i32>,
    /// The last lines of the engine's stderr (`STDERR_TAIL_LINES`, S-23).
    pub stderr_tail: String,
}
