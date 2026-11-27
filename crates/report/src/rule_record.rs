//! `RuleRecord`: one rule's entry in the telemetry record's rules list.

use crate::{RecordFinding, RuleState};
use rules::Engine;
use serde::Serialize;

/// One rule of the check in the telemetry record. No fix, owner or column. Serialized with its fields in this order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RuleRecord {
    /// The rule id.
    pub rule: String,
    pub engine: Engine,
    /// The rule's rule-file severity name after the rule file's overrides.
    pub severity: String,
    pub state: RuleState,
    /// The report's why for `not_available` and `not_applicable`; for `covers_nothing`, the why the engines gave with
    /// the rule's no-file flag (the report does not hold it); None for `ran` and `failed`.
    pub why: Option<String>,
    /// The rule's kept findings, in the report's order.
    pub findings: Vec<RecordFinding>,
    /// How many of the rule's findings an exception covered.
    pub silenced: usize,
}
