//! `CheckedRules`: every rule of a rule file as it was run, with the version of each engine they name.

use super::{EngineVersion, RuleRun};

/// One check of a rule file: each rule's run and each engine's version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedRules {
    /// Each rule's run, in rule-file order.
    pub runs: Vec<RuleRun>,
    /// Each distinct engine the rules name, once, in the order the rules first name it.
    pub engines: Vec<EngineVersion>,
}
