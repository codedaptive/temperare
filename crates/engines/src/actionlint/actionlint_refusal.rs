//! `actionlint_refusal`: why an actionlint check is not a kind tm can match back, if it is not.

/// The reason `check` is refused when it is not lowercase ASCII words joined by single `-` (`^[a-z]+(-[a-z]+)*$`), the
/// form of every actionlint kind (`expression`, `runner-label`); None when it is. The reader keeps errors whose `kind`
/// equals the check, so a check of another form could only ever read as clean. A well-formed kind the run did not
/// define is not refused here: the run's own `allKinds` says so (`unknown_kind`).
pub(crate) fn actionlint_refusal(check: &str) -> Option<String> {
    if check.split('-').all(|word| !word.is_empty() && word.bytes().all(|byte| byte.is_ascii_lowercase())) {
        return None;
    }
    Some(format!(
        "actionlint check `{check}` is not an actionlint check kind (lowercase words joined by -, such as expression)"
    ))
}
