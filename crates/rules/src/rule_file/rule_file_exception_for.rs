//! `RuleFile::exception_for`: the exception that covers a finding.

use super::RuleFile;
use crate::{Exception, Finding};

impl RuleFile {
    /// The first exception in file order with the finding's rule and path whose `what`, when given, is contained in
    /// the finding's `what`. Reading a rule file reports an empty or whitespace-only `what` as a Problem and
    /// drops that exception, so in a rule file an absent `what` is the one way to match every finding of the rule at
    /// the path.
    pub fn exception_for(&self, finding: &Finding) -> Option<&Exception> {
        self.exceptions.iter().find(|e| {
            e.rule == finding.rule
                && e.path == finding.path
                && e.what.as_deref().is_none_or(|what| finding.what.contains(what))
        })
    }
}
