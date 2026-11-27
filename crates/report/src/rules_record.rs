//! `rules_record`: the telemetry record's rules list.

use crate::{CoversNothing, NotApplicable, RecordFinding, RuleRecord, RuleState, Silenced};
use rules::{Finding, Rule, RuleFile};

/// One entry per rule of `rules.rules_in_order()`, in that order. A rule's state is, by the first that holds:
/// not applicable, with that entry's why, when its id is in `out_of_play`'s not-applicable list; covers nothing, with
/// the why of the first `no_files` pair for its id (the pair `out_of_play` reads), when it is in the covers-nothing
/// list; else the state and why of the first `outcomes` entry for its id. A rule with none of these is left out (in a
/// run none is: engines runs every rule of the file and cli passes each as (rule id, outcome)). See `out_of_play`
/// for which rules the two lists hold. `severity` is the rule's own (`Severity::name`); `findings` are the `kept`
/// findings of the rule, in `kept`'s order; `silenced` counts the `silenced` entries of the rule. Pure.
pub(crate) fn rules_record(
    rules: &RuleFile,
    outcomes: &[(String, RuleState, Option<String>)],
    out_of_play: (&[CoversNothing], &[NotApplicable]),
    no_files: &[(String, String)],
    kept: &[Finding],
    silenced: &[Silenced],
) -> Vec<RuleRecord> {
    let (covers_nothing, not_applicable) = out_of_play;
    rules
        .rules_in_order()
        .into_iter()
        .filter_map(|rule| {
            let (state, why) = if let Some(entry) = not_applicable.iter().find(|entry| entry.rule == rule.id) {
                (RuleState::NotApplicable, Some(entry.why.clone()))
            } else if covers_nothing.iter().any(|entry| entry.rule == rule.id) {
                let why = no_files.iter().find(|(id, _)| *id == rule.id).map(|(_, why)| why.clone());
                (RuleState::CoversNothing, why)
            } else {
                let (_, state, why) = outcomes.iter().find(|(id, _, _)| *id == rule.id)?;
                (*state, why.clone())
            };
            Some(record(rule, state, why, kept, silenced))
        })
        .collect()
}

/// The entry of `rule` in `state` with `why`, its findings taken from `kept` and its silenced count from `silenced`.
fn record(rule: &Rule, state: RuleState, why: Option<String>, kept: &[Finding], silenced: &[Silenced]) -> RuleRecord {
    RuleRecord {
        rule: rule.id.clone(),
        engine: rule.engine,
        severity: rule.severity.name().to_string(),
        state,
        why,
        findings: kept
            .iter()
            .filter(|finding| finding.rule == rule.id)
            .map(|finding| RecordFinding { path: finding.path.clone(), line: finding.line(), what: finding.what.clone() })
            .collect(),
        silenced: silenced.iter().filter(|entry| entry.finding.rule == rule.id).count(),
    }
}
