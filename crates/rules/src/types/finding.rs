//! `Finding`: one thing an engine found, in codeteam's finding fields.

use super::{Engine, RelPath, Severity};
use serde::{Deserialize, Serialize};

/// One finding. The field names are the output names; `col` is left out when the engine gives no column.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    /// The rule id; also the `Reason:` tag.
    pub rule: String,
    pub engine: Engine,
    /// "rule" for a tool finding.
    pub kind: String,
    pub severity: Severity,
    pub path: RelPath,
    pub line: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub col: Option<u32>,
    /// The engine's message.
    pub what: String,
    pub fix: String,
}
