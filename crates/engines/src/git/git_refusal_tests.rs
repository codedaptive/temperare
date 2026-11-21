//! Unit tests for git_refusal.rs: `test_ratio` is the one check not refused; the reason is written out from the contract.

use super::git_refusal;

#[test]
fn only_test_ratio_is_not_refused() {
    let cases = [
        ("test_ratio", None),
        ("other", Some("git check `other` is not one git counts: the git entry checks only `test_ratio`")),
        ("test-ratio", Some("git check `test-ratio` is not one git counts: the git entry checks only `test_ratio`")),
        ("", Some("git check `` is not one git counts: the git entry checks only `test_ratio`")),
    ];
    for (check, want) in cases {
        assert_eq!(git_refusal(check), want.map(str::to_string), "{check}");
    }
}
