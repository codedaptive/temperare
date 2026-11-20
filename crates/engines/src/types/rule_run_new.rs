//! `RuleRun::new`: a rule run from its four fields.

use super::RuleRun;
use rules::{Engine, RuleOutcome};

impl RuleRun {
    /// The run of rule `rule` on `engine` that came to `outcome`, its check run having printed `raw_chars` characters
    /// (`None` when no counted run was made; see `RuleRun::raw_chars`).
    pub fn new(rule: String, engine: Engine, outcome: RuleOutcome, raw_chars: Option<usize>) -> Self {
        Self { rule, engine, outcome, raw_chars }
    }
}
