//! `RuleFile::rules_in_order`: every rule of the file, applicable or not, in the resolved order.

use super::RuleFile;
use crate::Rule;

impl RuleFile {
    /// `rules` and `not_applicable_rules` together, sorted by `position`: after a parse, the taken sets' rules in
    /// `baseline` order with each not-applicable rule in its place, then the file's own rules. The sort is stable, so
    /// rules sharing a position (as built by `Rule::new`) keep `rules` first, then `not_applicable_rules`.
    pub fn rules_in_order(&self) -> Vec<&Rule> {
        let mut all: Vec<&Rule> = self.rules.iter().chain(&self.not_applicable_rules).collect();
        all.sort_by_key(|rule| rule.position);
        all
    }
}
