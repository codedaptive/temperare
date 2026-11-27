//! `rule_run_of`: one rule's run from what its engine's flow gave.

use crate::findings_from::findings_from;
use crate::not_available_cause_of::not_available_cause_of;
use crate::stopped::stopped;
use crate::types::{EngineVersion, Hit, RuleRun, Stop};
use rules::{RelPath, Rule, RuleFile, RuleOutcome};

/// The run of `rule` whose flow gave `got`, its check run having printed `raw_chars` characters. Hits are its
/// `findings_from` (cut by `only`). `Stop::NoFiles` (no file its engine checks) is `Findings` with none and the flag
/// `no_files` set to its why, so it is never read as plain clean; a flow that settles so before its check run
/// leaves the count `None`, and one that settles so after it keeps that run's count. Any other stop is `stopped`'s
/// outcome, a too-old check naming its version from `versions`, with the cause `not_available_cause_of` gives it set
/// by `with_cause` when it is not available. Every
/// path that settles a rule from its flow (a rule refused, a rule run alone, and both passes of `run_group`) comes
/// here, so a `NoFiles` from any flow carries its flag and a not-available run from any flow its cause.
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
        Err(stop) => {
            let cause = not_available_cause_of(&stop, rule.engine);
            let stopped_run = run(stopped(rule, stop, versions));
            match cause {
                Some(cause) => stopped_run.with_cause(cause),
                None => stopped_run,
            }
        }
    }
}
