//! `RuleCases`: the failing and passing cases a baseline set carries for one of its rules.

use super::{CaseFile, ExpectedFinding};

/// A baseline rule's cases: run on `failing`, the rule gives `expected`; run on `passing`, it gives no finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleCases {
    /// The files of the case the rule must report, in the order the set lists them.
    pub failing: Vec<CaseFile>,
    /// The files of the case the rule must not report, in the order the set lists them.
    pub passing: Vec<CaseFile>,
    /// The finding the failing case must give.
    pub expected: ExpectedFinding,
}
