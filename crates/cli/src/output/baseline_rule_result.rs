//! `BaselineRuleResult`: one rule's result under `cctao baseline test`.

use super::case_verdict::CaseVerdict;

/// One rule of a baseline set: `set` the set it came from (`name@version`), `rule` its id, `engine` its engine, and
/// the verdicts of its failing and its passing case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BaselineRuleResult {
    pub set: String,
    pub rule: String,
    pub engine: rules::Engine,
    pub failing: CaseVerdict,
    pub passing: CaseVerdict,
}
