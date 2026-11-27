//! Unit tests for shellcheck_refusal.rs: which checks are refused and the reason given, the cases written out from the
//! contract.

use super::shellcheck_refusal;

#[test]
fn one_code_is_not_refused() {
    for check in ["SC2086", "SC1", "SC01073", "SC0", "SC02086"] {
        assert_eq!(shellcheck_refusal(check), None, "{check}");
    }
}

#[test]
fn anything_but_one_code_is_refused() {
    for check in ["sc2086", "SC", "SC20x6", "2086", "SC2086,SC2034", "", "-SC2086"] {
        assert!(shellcheck_refusal(check).is_some(), "{check}");
    }
}

#[test]
fn the_reason_names_the_check() {
    assert_eq!(
        shellcheck_refusal("SC2086,SC2034"),
        Some(
            "shellcheck check `SC2086,SC2034` is not one ShellCheck code such as `SC2086`, so cctao cannot tell which findings it selects"
                .to_string()
        )
    );
}
