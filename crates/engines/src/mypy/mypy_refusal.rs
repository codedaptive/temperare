//! `mypy_refusal`: why tm cannot match a mypy check's findings back to it, if it cannot.

/// The reason `check` is refused; None when it has the shape of one mypy error code: lowercase words of ASCII letters
/// and digits joined by single `-`, the first starting with a letter (`arg-type`, `misc`, `no-untyped-def`). tm keeps
/// the findings whose code equals the check, so a list, a spaced or a capitalised check would select nothing.
pub(crate) fn mypy_refusal(check: &str) -> Option<String> {
    let one_code = check.starts_with(|c: char| c.is_ascii_lowercase())
        && check
            .split('-')
            .all(|word| !word.is_empty() && word.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
    if one_code {
        return None;
    }
    Some(format!(
        "mypy check `{check}` is not one mypy error code (lowercase words joined by `-`, as `arg-type`), so tm cannot tell which findings it selects"
    ))
}
