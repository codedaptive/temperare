//! `yamllint_refusal`: why a yamllint check is not one rule id cctao can write into yamllint's config, if it is not.

/// The reason `check` is refused when it is not one yamllint rule id: an ASCII lower-case letter, then ASCII lower-case
/// letters, digits and `-`; None when it is one. The check is written as a key into the YAML config cctao hands yamllint
/// (`config_data`), so nothing but a rule id may reach that text: a comma, colon, quote, space or capital could change
/// what the config says. An id of that shape that yamllint lacks is not refused here: yamllint says so itself, and the
/// rule is then not available (`no_such_rule`).
pub(crate) fn yamllint_refusal(check: &str) -> Option<String> {
    let mut bytes = check.bytes();
    let starts = bytes.next().is_some_and(|byte| byte.is_ascii_lowercase());
    if starts && bytes.all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-') {
        return None;
    }
    Some(format!("yamllint check `{check}` is not a yamllint rule id; a yamllint check is one rule id, such as `line-length`"))
}
