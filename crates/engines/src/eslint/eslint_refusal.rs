//! `eslint_refusal`: why a check is not an ESLint rule id tm can pass and match, if it is not.

/// The reason `check` is refused: it is empty or `@` alone, or holds a character other than an ASCII letter or digit, `-`, `_`,
/// `.` or `/`, with `@` allowed only first (a scoped plugin's rules: `@typescript-eslint/no-explicit-any`); None for a
/// rule id. tm passes the check to `--rule` as a JSON key and matches findings by their `ruleId`, so a space, `,`, `"`
/// or `:` could name something else in either place.
pub(crate) fn eslint_refusal(check: &str) -> Option<String> {
    let body = check.strip_prefix('@').unwrap_or(check);
    let rule_id = !body.is_empty() && body.chars().all(|c| c.is_ascii_alphanumeric() || "-_./".contains(c));
    (!rule_id).then(|| {
        format!("eslint check `{check}` is not an ESLint rule id; tm passes it to `--rule` and matches findings by it")
    })
}
