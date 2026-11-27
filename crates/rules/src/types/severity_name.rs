//! `Severity::name`: a severity's rule-file name.

use super::Severity;

impl Severity {
    /// The severity's rule-file name, the one `Severity::from_name` reads and serde writes.
    pub fn name(self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Watch => "watch",
        }
    }
}
