//! `Severity`: how much a rule's finding counts.

use serde::{Deserialize, Serialize};

/// A rule's severity; its serialized names are the rule-file names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Severity {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "watch")]
    Watch,
}
