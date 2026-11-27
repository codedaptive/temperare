//! `severity_name`: a severity's rule-file name.

use rules::Severity;

/// The rule-file name of `severity`, as the variant's Debug text lowercased, so report builds whatever variants rules
/// adds without an exhaustive match. `severity_name_tests` pins each variant (listed by hand) through
/// `Severity::from_name`, which is what catches a variant whose Debug name is not its rule-file name.
pub(crate) fn severity_name(severity: Severity) -> String {
    format!("{severity:?}").to_ascii_lowercase()
}
