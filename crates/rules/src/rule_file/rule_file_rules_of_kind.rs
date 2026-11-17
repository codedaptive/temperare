//! `RuleFile::rules_of_kind`: the rules of one kind, e.g. the security rules alone.

use super::RuleFile;
use crate::{Kind, Rule};

impl RuleFile {
    /// Every rule whose kind is `kind`, in file order.
    pub fn rules_of_kind(&self, kind: Kind) -> Vec<&Rule> {
        self.rules.iter().filter(|rule| rule.kind == kind).collect()
    }
}
