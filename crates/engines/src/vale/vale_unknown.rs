//! `vale_unknown`: Vale's words saying no loaded rule is the check.

use super::vale_errors;
use procio::Output;

/// The Text of the first error object on `out`'s stderr (`vale_errors`) that starts `invalid operation:
/// int(unknown)` and holds `.Name == "<check>"`, trimmed; None when there is none. That object is what
/// `vale_filter`'s expression makes Vale print when it loaded no rule named `check`. Vale echoes the expression in
/// the Text, so a Text naming another rule is not about the check and the run is left to the reader.
pub(crate) fn vale_unknown(out: &Output, check: &str) -> Option<String> {
    let named = format!(".Name == \"{check}\"");
    vale_errors(&out.stderr)
        .into_iter()
        .map(|(_, text)| text)
        .find(|text| text.starts_with("invalid operation: int(unknown)") && text.contains(&named))
        .map(|text| text.trim().to_string())
}
