//! `Exception`: one `[[exception]]` of the rule file.

use crate::RelPath;

/// An approved exception to one rule at one path; `what`, when given, limits it to findings whose `what` contains it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exception {
    pub rule: String,
    pub path: RelPath,
    pub what: Option<String>,
    pub reason: String,
    pub approved: String,
}
