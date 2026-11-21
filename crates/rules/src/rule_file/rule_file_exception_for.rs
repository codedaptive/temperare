//! `RuleFile::exception_for`: the exception that covers a finding.

use super::RuleFile;
use crate::{Exception, Finding};

impl RuleFile {
    /// The first exception in file order that covers the finding (`Exception::covers_finding`: same rule and path,
    /// and `what`, when given, contained in the finding's `what`). Reading a rule file reports an empty or
    /// whitespace-only `what` as a Problem and drops that exception, so in a rule file an absent `what` is the one way
    /// to match every finding of the rule at the path. Expiry is not judged here; `exception_on` judges it.
    pub fn exception_for(&self, finding: &Finding) -> Option<&Exception> {
        self.exceptions.iter().find(|e| e.covers_finding(finding))
    }
}
