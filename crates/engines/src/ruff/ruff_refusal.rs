//! `ruff_refusal`: why tm cannot match a ruff check's findings back to it, if it cannot.

/// The reason `check` is refused, naming its first comma part that is neither `ALL` nor a rule code or code prefix
/// (one or more ASCII uppercase letters, then zero or more ASCII digits: `E`, `E7`, `PLC0414`); None when every part
/// is one. ruff in preview mode also selects by rule name and category (`correctness`), whose findings carry codes
/// no prefix test can tie back to the check.
pub(crate) fn ruff_refusal(check: &str) -> Option<String> {
    let part = check.split(',').find(|part| *part != "ALL" && !is_code_prefix(part))?;
    Some(format!(
        "ruff check `{check}`: `{part}` is not a rule code, a code prefix or ALL, so tm cannot tell which findings it selects"
    ))
}

/// One or more ASCII uppercase letters followed by zero or more ASCII digits.
fn is_code_prefix(part: &str) -> bool {
    let digits = part.trim_start_matches(|c: char| c.is_ascii_uppercase());
    digits.len() < part.len() && digits.chars().all(|c| c.is_ascii_digit())
}
