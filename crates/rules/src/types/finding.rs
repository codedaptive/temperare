//! `Finding`: one thing an engine found, with the fields a report gives each finding.

use super::{Engine, Kind, RelPath, Severity};
use serde::{Deserialize, Serialize};

/// One finding. The field names are the output names; `owner` is left out when the file has no owner, and `col`
/// when the engine gives no column.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    /// The rule id; also the `Reason:` tag.
    pub rule: String,
    pub engine: Engine,
    /// The rule's kind.
    pub kind: Kind,
    pub severity: Severity,
    /// The zone that owns the finding's path, from the rule file's kits or its top-level `owner`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    pub path: RelPath,
    pub line: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub col: Option<u32>,
    /// The engine's message.
    pub what: String,
    pub fix: String,
}
