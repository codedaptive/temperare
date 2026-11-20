//! `RuleCount`: one rule's share of the raw engine output tm check's savings estimate counts.

/// One rule's count, in rule-file order: `rule` its id, `engine` its engine's name, `chars` the characters (Unicode
/// scalar values) of its counted engine run's stdout and stderr (0 when its engine did not run), and `ran` whether its
/// engine ran (findings, clean, or failed after it ran: true; not available: false). `ran` is kept apart because an
/// engine that ran and printed nothing also has `chars` 0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleCount {
    pub rule: String,
    pub engine: String,
    pub chars: u64,
    pub ran: bool,
}
