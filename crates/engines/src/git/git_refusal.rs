//! `git_refusal`: why tm refuses a git rule's check, if it does.

/// None for `test_ratio`, the one count the git entry makes; any other check is refused with a reason naming it, so a
/// misspelt check fails before anything runs rather than reading clean.
pub(crate) fn git_refusal(check: &str) -> Option<String> {
    if check == "test_ratio" {
        return None;
    }
    Some(format!("git check `{check}` is not one git counts: the git entry checks only `test_ratio`"))
}
