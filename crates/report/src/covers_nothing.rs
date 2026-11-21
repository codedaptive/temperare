//! `CoversNothing`: a repository's own rule that has nothing to check.

use rules::Engine;
use serde::Serialize;

/// A repository's own rule whose paths matched no file, or no file its engine checks: never a finding, never "not
/// available" (nothing is missing) and never clean (nothing was checked). Serialized with its fields in this order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CoversNothing {
    pub rule: String,
    pub engine: Engine,
}
