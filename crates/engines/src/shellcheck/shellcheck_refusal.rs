//! `shellcheck_refusal`: why cctao cannot match a shellcheck check's findings back to it, if it cannot.

/// The reason `check` is refused; None when it is one ShellCheck code: `SC` then one or more ASCII digits, leading
/// zeros included. The check is never passed to shellcheck, only compared with each comment's `SC<code>`, so a check
/// of any other form would match nothing and read clean. A leading-zero code such as `SC02086` is well-formed, but
/// cctao's list holds codes as shellcheck prints them, with no leading zero, so it is not on the list and its
/// rule is not available, never clean.
pub fn shellcheck_refusal(check: &str) -> Option<String> {
    if check.strip_prefix("SC").is_some_and(|digits| !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit())) {
        return None;
    }
    Some(format!(
        "shellcheck check `{check}` is not one ShellCheck code such as `SC2086`, so cctao cannot tell which findings it selects"
    ))
}
