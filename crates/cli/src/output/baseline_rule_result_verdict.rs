//! `BaselineRuleResult::verdict`: one rule's verdict from its two cases.

use super::baseline_rule_result::BaselineRuleResult;
use super::case_verdict::CaseVerdict;

impl BaselineRuleResult {
    /// "broken" when either case is broken; else "not applicable" when either is not applicable; else "not available"
    /// when either is not available; else "holds".
    pub(crate) fn verdict(&self) -> &'static str {
        let cases = [&self.failing, &self.passing];
        if cases.iter().any(|c| matches!(c, CaseVerdict::Broken { .. })) {
            "broken"
        } else if cases.iter().any(|c| matches!(c, CaseVerdict::NotApplicable { .. })) {
            "not applicable"
        } else if cases.iter().any(|c| matches!(c, CaseVerdict::NotAvailable { .. })) {
            "not available"
        } else {
            "holds"
        }
    }
}
