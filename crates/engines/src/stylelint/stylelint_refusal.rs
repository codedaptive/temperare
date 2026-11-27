//! `stylelint_refusal`: why a check is not a stylelint rule name cctao can match, if it is not.

/// The reason `check` is refused: it is empty or `@` alone, or holds a character other than an ASCII lowercase letter,
/// a digit, `-` or `/`, with `@` allowed only first (plugin rules: `scss/at-rule-no-unknown`,
/// `@stylistic/indentation`); None for a rule name. cctao matches findings by their `rule`, and stylelint reports a CSS
/// syntax error under `CssSyntaxError`, which is no rule and holds uppercase letters, so it is refused too.
pub(crate) fn stylelint_refusal(check: &str) -> Option<String> {
    let body = check.strip_prefix('@').unwrap_or(check);
    let rule_name =
        !body.is_empty() && body.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || "-/".contains(c));
    (!rule_name).then(|| format!("stylelint check `{check}` is not a stylelint rule name; cctao matches findings by it"))
}
