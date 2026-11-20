//! `NotApplicable`: a rule the check does not apply.

use rules::Engine;
use serde::Serialize;

/// A rule the check does not apply: never a finding, and never "not available" (nothing is missing; the rule does
/// not run here). Serialized with its fields in this order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NotApplicable {
    pub rule: String,
    pub engine: Engine,
    /// Why the rule does not apply.
    pub why: String,
}
