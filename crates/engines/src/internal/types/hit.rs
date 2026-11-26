//! `Hit`: one finding as an engine reported it, before the rule's own fields are added.

use rules::RelPath;

/// One engine finding: where it is and the engine's message. The rule, kind, severity, owner and fix come from the
/// rule that ran the engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hit {
    /// Relative to the rule file's folder.
    pub path: RelPath,
    /// 1-based; None when the engine gives no line.
    pub line: Option<u32>,
    /// 1-based; None when the engine gives no column.
    pub col: Option<u32>,
    /// The engine's message.
    pub what: String,
}
