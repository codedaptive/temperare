//! `Counts`: the report's counts.

use serde::Serialize;

/// The report's counts. `findings`, `errors`, `warnings`, `quality` and `security` count the kept findings (all of
/// them, by severity, by kind); the rest count the report's other lists.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Counts {
    pub findings: usize,
    pub errors: usize,
    pub warnings: usize,
    pub quality: usize,
    pub security: usize,
    pub silenced: usize,
    pub not_available: usize,
    pub failed: usize,
    pub not_applicable: usize,
}
