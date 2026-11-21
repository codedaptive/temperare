//! `RuleCount`: one rule's share of the raw engine output tm check's savings estimate counts.

/// One rule's count, in rule-file order: `rule` its id, `engine` its engine's name, `chars` the characters (Unicode
/// scalar values) of its counted engine run's stdout and stderr (0 when it has no counted run), and `ran` whether it
/// has a counted run (false when its group's shared run was counted on another rule, or its check run did not run,
/// could not start or timed out). `ran` is kept apart because a counted run that printed nothing also has `chars` 0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleCount {
    pub rule: String,
    pub engine: String,
    pub chars: u64,
    pub ran: bool,
}
