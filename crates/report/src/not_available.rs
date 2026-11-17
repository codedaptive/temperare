//! `NotAvailable`: a rule the report could not check.

use rules::Engine;
use serde::Serialize;

/// A rule whose engine, or what the rule needs, is absent: never "clean". Serialized with its fields in this order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NotAvailable {
    pub rule: String,
    pub engine: Engine,
    /// Why it is not available, as the engine adapter said it.
    pub why: String,
}
