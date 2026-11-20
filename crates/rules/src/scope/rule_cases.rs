//! `Rule::cases`: the failing and passing cases a baseline set carries for the rule.

use super::Rule;
use crate::RuleCases;

impl Rule {
    /// The rule's failing and passing cases when the baseline set it came from carries them in tm; None for a
    /// repository's own rule and for a rule built by `Rule::new`.
    pub fn cases(&self) -> Option<&RuleCases> {
        self.embedded_cases.as_ref()
    }
}
