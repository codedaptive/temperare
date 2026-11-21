//! `baseline_exit`: the exit code of `tm baseline test` from its results.

use super::baseline_rule_result::BaselineRuleResult;

/// 1 when any rule is broken; else 3 when any is not applicable or not available; else 0, no results at all included.
pub(crate) fn baseline_exit(results: &[BaselineRuleResult]) -> u8 {
    if results.iter().any(|r| r.verdict() == "broken") {
        1
    } else if results.iter().any(|r| matches!(r.verdict(), "not applicable" | "not available")) {
        3
    } else {
        0
    }
}
