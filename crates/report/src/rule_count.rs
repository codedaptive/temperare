//! `RuleCount`: how many silenced findings one rule has.

use serde::Serialize;

/// One rule's silenced findings in a report: the rule id and how many of its findings an exception covered.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RuleCount {
    pub rule: String,
    pub count: usize,
}
