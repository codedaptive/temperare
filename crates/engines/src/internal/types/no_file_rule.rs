//! `NoFileRule`: one rule that `tm check` settles as no-file before any engine runs, with its engine and why.

use rules::Engine;

/// One rule of a rule file that `tm check` settles as no-file before any engine runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoFileRule {
    /// The rule's id.
    pub rule: String,
    /// The rule's engine.
    pub engine: Engine,
    /// Which case: its paths match no file of the rule file's folder, or none of the files is one its engine checks.
    /// The same text `RuleRun::no_files` carries for the rule.
    pub why: String,
}
