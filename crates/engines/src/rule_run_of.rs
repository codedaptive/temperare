//! `rule_run_of`: one rule's run from what its engine's flow gave.

use crate::findings_from::findings_from;
use crate::run_rules_counted_with::stopped;
use crate::types::{EngineVersion, Hit, RuleRun, Stop};
use rules::{RelPath, Rule, RuleFile, RuleOutcome};

/// The run of `rule` whose flow gave `got`, its check run having printed `raw_chars` characters. Hits are its
/// `findings_from` (cut by `only`). `Stop::NoFiles` (no file its engine checks) is `Findings` with none and the flag
/// `no_files` set to its why, so it is never read as plain clean; a flow stops so before its check run, so its
/// count is the `None` it left. Any other stop is `stopped`'s outcome, a too-old check naming its version from
/// `versions`. Every path that settles a rule from its flow (a rule run alone, and both passes of `run_group`) comes
/// here, so a `NoFiles` from any flow carries its flag.
pub(crate) fn rule_run_of(
    file: &RuleFile,
    rule: &Rule,
    got: Result<Vec<Hit>, Stop>,
    only: &[RelPath],
    versions: &[EngineVersion],
    raw_chars: Option<usize>,
) -> RuleRun {
    let run = |outcome| RuleRun::new(rule.id.clone(), rule.engine, outcome, raw_chars);
    match got {
        Ok(hits) => run(RuleOutcome::Findings(findings_from(file, rule, hits, only))),
        Err(Stop::NoFiles { why }) => run(RuleOutcome::Findings(Vec::new())).with_no_files(why),
        Err(stop) => run(stopped(rule, stop, versions)),
    }
}
