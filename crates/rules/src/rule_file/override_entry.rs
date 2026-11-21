//! `OverrideEntry`: one `[[override]]` of the rule file, as written.

use crate::Severity;

/// A repository's change to one baseline rule. Each `None` (and `off` false) means the key was not given, so the
/// baseline's own value stands.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct OverrideEntry {
    /// The id of the rule it changes.
    pub rule: String,
    /// The byte offset of the `rule` string in the rule-file text.
    pub at: usize,
    pub severity: Option<Severity>,
    /// True when the override turns the rule off.
    pub off: bool,
    pub paths: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub params: Option<toml::Table>,
}
