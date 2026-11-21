//! ZUT for only_with_base_why.rs: the why text, as the contract gives it.

use crate::ONLY_WITH_BASE_WHY;

#[test]
fn only_with_base_why_is_the_contract_text() {
    assert_eq!(ONLY_WITH_BASE_WHY, "reports only what a change adds; run with --base");
}
