//! `markdownlint_refusal`: why a markdownlint check is not a rule name cctao can pass, if it is not.

/// The reason `check` is refused when it is empty or holds a character other than an ASCII letter, digit or `-`;
/// None when it does not. Every markdownlint rule name and alias is of that form (`MD013`, `line-length`). A name of
/// that form that is not in cctao's list (`MD999`) is not refused here: the flow says it is not listed.
pub(crate) fn markdownlint_refusal(check: &str) -> Option<String> {
    if !check.is_empty() && check.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'-') {
        return None;
    }
    Some(format!(
        "markdownlint check `{check}` is not a markdownlint rule name (letters, digits and `-`, as MD013 or \
         line-length)"
    ))
}
