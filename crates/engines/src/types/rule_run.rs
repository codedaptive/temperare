//! `RuleRun`: one rule's outcome with its engine and the characters its check run printed.

use rules::{Engine, RuleOutcome};

/// One rule of a rule file as it was run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleRun {
    /// The rule's id.
    pub rule: String,
    /// The rule's engine.
    pub engine: Engine,
    /// What the run came to.
    pub outcome: RuleOutcome,
    /// `Some(n)`: the rule's check run, or its group's shared run (`run_group`) counted on it, ran and printed n
    /// characters, stdout plus stderr, each read as lossy UTF-8 and counted in chars; `Some(0)` when it printed
    /// nothing. A check the engine's run said it does not know is `NotAvailable` after its counted run, so it is `Some`
    /// unless that run was counted on another rule. `None`: its engine made no counted run (an engine not available, a
    /// check refused or found unknown before its run, or a run that could not start or timed out), or its shared run
    /// was counted on another rule of its group.
    pub raw_chars: Option<usize>,
}
