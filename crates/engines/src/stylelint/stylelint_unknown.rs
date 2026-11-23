//! `stylelint_unknown`: stylelint's words saying its config names a rule it does not know.

use super::stylelint_report;
use procio::Output;

/// The text of the first warning in `out`'s report whose `rule` is `check` and whose text starts with
/// `Unknown rule ` (stylelint reports a rule its config names but does not know once per linted file, as
/// `Unknown rule <name>. Did you mean <x>?`); None when there is no report or no such warning. An unknown rule of
/// another name (one the config names besides the check) is not about the check, so the rule is never blamed for it.
pub(crate) fn stylelint_unknown(out: &Output, check: &str) -> Option<String> {
    stylelint_report(out)?
        .into_iter()
        .flat_map(|file| file.warnings)
        .find(|warning| warning.rule.as_deref() == Some(check) && warning.text.starts_with("Unknown rule "))
        .map(|warning| warning.text)
}
