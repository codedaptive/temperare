//! `RuleOutcome`: what running one rule's engine came to. Built by engines, read by report; it lives here because
//! report must not depend on engines.

use super::{Engine, Finding};
use serde::{Deserialize, Serialize};

/// The outcome of one rule. Serde's external tagging with snake_case tags: `{"findings":[...]}`,
/// `{"not_available":{...}}`, `{"failed":{...}}`. Internal tagging cannot carry the newtype `Findings`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleOutcome {
    /// The engine ran; these are its findings, possibly none.
    Findings(Vec<Finding>),
    /// The engine, or what the rule needs, is absent. Never "clean".
    NotAvailable { rule: String, engine: Engine, why: String },
    /// The engine ran and failed: its exit code if it had one (`null` when it had none), its stderr as captured.
    Failed { rule: String, engine: Engine, code: Option<i32>, stderr: String },
}
