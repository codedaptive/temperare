//! `Kind`: what a rule guards, carried to its findings.

use serde::{Deserialize, Serialize};

/// A rule's kind; its serialized names are the rule-file names. `RuleFile::rules_of_kind` selects the rules of one kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "quality")]
    Quality,
    #[serde(rename = "security")]
    Security,
}
