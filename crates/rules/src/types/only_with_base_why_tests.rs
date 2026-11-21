//! Unit tests for only_with_base_why.rs: the why text a base-only rule reports when the check has no base.

use super::ONLY_WITH_BASE_WHY;

#[test]
fn the_why_names_what_the_rule_reports_and_the_base_flag() {
    assert_eq!(ONLY_WITH_BASE_WHY, "reports only what a change adds; run with --base");
}
