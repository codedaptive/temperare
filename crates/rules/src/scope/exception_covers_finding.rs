//! `Exception::covers_finding`: whether an exception matches a finding.

use super::Exception;
use crate::Finding;

impl Exception {
    /// True when the finding has the exception's rule and path and the exception's `what`, when given, is contained
    /// in the finding's `what`. Reading a rule file reports an empty or whitespace-only `what` as a Problem and drops
    /// that exception, so in a rule file an absent `what` is the one way to match every finding of the rule at the
    /// path. Expiry is not judged here (`holds_on` does that).
    pub(crate) fn covers_finding(&self, finding: &Finding) -> bool {
        self.rule == finding.rule
            && self.path == finding.path
            && self.what.as_deref().is_none_or(|what| finding.what.contains(what))
    }
}
