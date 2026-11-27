//! `clang_tidy_refusal`: why cctao cannot match a clang-tidy check's findings back to it, if it cannot.

/// None when `check` is one check name: an ASCII letter, then ASCII letters, digits, `-`, `.` and `_`
/// (`bugprone-branch-clone`, `clang-analyzer-core.NullDereference`). Anything else is refused with the reason: a glob
/// (`*`), a list (`,`) or a leading `-` selects checks whose findings cctao cannot tie back to the rule.
pub(crate) fn clang_tidy_refusal(check: &str) -> Option<String> {
    let one_name = check.starts_with(|c: char| c.is_ascii_alphabetic())
        && check.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '.' | '_'));
    (!one_name).then(|| {
        format!(
            "clang-tidy check `{check}` is not one check name (letters, digits, `-`, `.` and `_`, starting with a letter), so cctao cannot tell which findings it selects"
        )
    })
}
