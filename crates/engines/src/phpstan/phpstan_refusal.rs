//! `phpstan_refusal`: why a phpstan check is not a level tm can pass, if it is not.

/// The reason `check` is refused when it is neither one or more ASCII digits nor exactly `max`; None when it is one.
/// A phpstan check is a level passed as `--level` (S-71). A number PHPStan lacks (11, say) is not refused here: the
/// installed PHPStan says so itself, and the rule is then not available (`unknown_level`).
pub(crate) fn phpstan_refusal(check: &str) -> Option<String> {
    if check == "max" || (!check.is_empty() && check.bytes().all(|byte| byte.is_ascii_digit())) {
        return None;
    }
    Some(format!("phpstan check `{check}` is not a PHPStan level (0 to 10, or max)"))
}
