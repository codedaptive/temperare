//! `RecordFinding`: one kept finding as the telemetry record carries it.

use rules::RelPath;
use serde::Serialize;

/// A kept finding in the telemetry record: no fix, owner or column. Serialized with its fields in this order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecordFinding {
    /// Relative to the rule file's folder, as the finding holds it.
    pub path: RelPath,
    /// None when the engine gives no line; null in JSON.
    pub line: Option<u32>,
    /// The engine's message.
    pub what: String,
}
