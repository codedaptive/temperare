//! `rule_counts`: the rules' runs as the counts cctao check's savings estimate reads.

use crate::output::serde_name;
use crate::RuleCount;
use engines::RuleRun;
use rules::{RuleFile, Severity};

/// One count per run, in order: the rule's id, its engine's serde name (`ruff`, `ast-grep`), the characters its
/// counted run printed (0 when there was none) and whether there was one. A run of a watch rule in `rf` has no
/// count: cctao prints none of its findings, so nobody would have read its raw output, and it is in neither the engine
/// runs nor the baseline characters. A rule run alone carries its own run's characters, so a lone watch rule's run
/// is left out here. Engines put a shared run's characters on the first rule it settled that is not a watch rule,
/// so it is counted once, there; a shared run that settled only watch rules is counted on none. A run whose id names
/// no rule in `rf` is counted.
pub(crate) fn rule_counts(runs: &[RuleRun], rf: &RuleFile) -> Vec<RuleCount> {
    runs.iter()
        .filter(|run| !rf.rules.iter().any(|rule| rule.id == run.rule && rule.severity == Severity::Watch))
        .map(|run| RuleCount {
            rule: run.rule.clone(),
            engine: serde_name(serde_json::to_value(run.engine)),
            chars: run.raw_chars.map_or(0, |chars| u64::try_from(chars).unwrap_or(u64::MAX)),
            ran: run.raw_chars.is_some(),
        })
        .collect()
}
