//! `hadolint_refusal`: why cctao cannot match a hadolint check's findings back to it, if it cannot.

/// The reason `check` is refused; None when it has the form of one hadolint code: `DL` or `SC`, then one or more ASCII
/// digits. cctao keeps the findings whose rule id equals the check, so a comma list, `ALL`, a lower-case or an empty
/// check would select findings cctao cannot tell apart, or none, and read clean. A well-formed code is not yet a known
/// one: cctao's list of hadolint codes settles that, after this.
pub fn hadolint_refusal(check: &str) -> Option<String> {
    let digits = check.strip_prefix("DL").or_else(|| check.strip_prefix("SC"));
    if digits.is_some_and(|digits| !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit())) {
        return None;
    }
    Some(format!(
        "hadolint check `{check}` is not one hadolint code such as `DL3006`, so cctao cannot tell which findings it selects"
    ))
}
