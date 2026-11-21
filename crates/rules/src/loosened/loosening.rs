//! `Loosening`: one way a change to a rule file loosened it.

use super::LooseningKind;
use serde::Serialize;

/// One loosening: its kind, the rule id or set name it is about, and the values before and after where the kind has
/// them (`LooseningKind` says which). An absent value is left out of the serialized form.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Loosening {
    pub kind: LooseningKind,
    pub about: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}
