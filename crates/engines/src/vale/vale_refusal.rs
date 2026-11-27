//! `vale_refusal`: why a vale check is not a Vale rule name cctao can pass, if it is not.

/// The reason `check` is refused when it is not `Style.Rule`: exactly one `.`, both sides non-empty, every character
/// an ASCII letter, digit, `_` or `-`; None when it is. A vale check names one rule (`Vale.Spelling`). cctao
/// writes the check into Vale's `--filter` expression and the style into the wrapper config, so the narrow set keeps
/// a check from ending the expression's quoted string or a config line.
pub(crate) fn vale_refusal(check: &str) -> Option<String> {
    let part_ok = |part: &str| !part.is_empty() && part.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
    match check.split_once('.') {
        Some((style, rule)) if part_ok(style) && part_ok(rule) => None,
        _ => Some(format!(
            "vale check `{check}` is not a Vale rule name (Style.Rule, each part ASCII letters, digits, _ or -)"
        )),
    }
}
