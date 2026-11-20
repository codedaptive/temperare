//! `rule_counts`: the rules' runs as the counts tm check's savings estimate reads.

use crate::output::serde_name;
use crate::RuleCount;
use engines::RuleRun;

/// One count per run, in order: the rule's id, its engine's serde name (`ruff`, `ast-grep`), the characters its
/// counted run printed (0 when there was none) and whether there was one.
pub(crate) fn rule_counts(runs: &[RuleRun]) -> Vec<RuleCount> {
    runs.iter()
        .map(|run| RuleCount {
            rule: run.rule.clone(),
            engine: serde_name(serde_json::to_value(run.engine)),
            chars: run.raw_chars.map_or(0, |chars| u64::try_from(chars).unwrap_or(u64::MAX)),
            ran: run.raw_chars.is_some(),
        })
        .collect()
}
