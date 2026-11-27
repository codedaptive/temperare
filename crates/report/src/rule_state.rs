//! `RuleState`: what a rule came to in one check, for the telemetry record.

use serde::Serialize;

/// A rule's state in the telemetry record, serialized in snake_case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleState {
    /// The engine ran, with findings or none.
    Ran,
    /// The rule's engine is not available.
    NotAvailable,
    /// The rule's engine failed.
    Failed,
    /// The report lists the rule as not applicable.
    NotApplicable,
    /// The report lists the rule as covering nothing.
    CoversNothing,
}
