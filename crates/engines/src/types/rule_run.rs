//! `RuleRun`: one rule's outcome with its engine, the characters its check run printed, and whether it ran nothing
//! because no file it checks exists.

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
    /// check refused or found unknown before its run, a run that could not start or timed out, or a rule that ran
    /// nothing because no file it checks exists), or its shared run was counted on another rule of its group.
    pub raw_chars: Option<usize>,
    /// `Some(why)`: the rule ran nothing because no file it checks exists, and `why` says which case (its paths match
    /// no file of the rule file's folder, or none of the files is one its engine checks). Its outcome is then `Findings` with none and
    /// `raw_chars` is `None`; the flag keeps it from reading as plain clean, and the report decides what it is called.
    /// `None`: the rule had files to check, or stopped for another reason its outcome gives.
    pub no_files: Option<String>,
}
