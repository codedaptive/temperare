//! `sqlfluff_refusal`: why cctao will not pass a check to sqlfluff as one rule, if it will not.

/// The reason `check` is refused; None when it has the shape of one sqlfluff rule reference: non-empty, only ASCII
/// letters, digits, `_` and `.`, not starting with `.` (a code `LT01`, a name `layout.spacing`, a group `layout`, an
/// alias `L039`). sqlfluff's `--rules` takes a comma list, so a list would run several rules under one rule's id, and a
/// space or an empty check is no reference at all.
pub(crate) fn sqlfluff_refusal(check: &str) -> Option<String> {
    let one_reference = !check.is_empty()
        && !check.starts_with('.')
        && check.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.');
    if one_reference {
        return None;
    }
    Some(format!(
        "sqlfluff check `{check}` is not one sqlfluff rule code, name or group (as `LT01`, `layout.spacing` or `layout`), so cctao cannot pass it as one rule"
    ))
}
