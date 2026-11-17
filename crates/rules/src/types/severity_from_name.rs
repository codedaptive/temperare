//! `Severity::from_name`: a severity from its rule-file name.

use super::Severity;

impl Severity {
    /// The severity named `name` in a rule file ("error" or "warning"), or None for any other text.
    pub fn from_name(name: &str) -> Option<Severity> {
        match name {
            "error" => Some(Severity::Error),
            "warning" => Some(Severity::Warning),
            _ => None,
        }
    }
}
