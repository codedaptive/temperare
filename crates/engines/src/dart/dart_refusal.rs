//! `dart_refusal`: why tm cannot match a dart check's findings back to it, if it cannot.

/// The reason `check` is refused; None when it is one diagnostic code or lint name as the analyzer's own
/// `analysis_options.yaml` and `// ignore:` comments spell it: a lower-case ASCII letter, then lower-case ASCII
/// letters, digits and `_`. The check is refused before running because `dart analyze` has no option that selects a
/// diagnostic: it reports every one, and the reader keeps those whose code is the check, so a check of any other
/// shape (upper case, a list, a glob) would read clean.
pub(crate) fn dart_refusal(check: &str) -> Option<String> {
    let mut chars = check.chars();
    let starts = chars.next().is_some_and(|first| first.is_ascii_lowercase());
    if starts && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
        return None;
    }
    Some(format!(
        "dart check `{check}` is not one code: a dart check is one diagnostic code or lint name, lower case \
         (unused_local_variable, prefer_single_quotes)"
    ))
}
