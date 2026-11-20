//! `RuleFile::exception_on`: the exception that covers a finding on a given day.

use super::RuleFile;
use crate::{Date, Exception, Finding};

impl RuleFile {
    /// The exception that covers `finding` on `day`. Expiry is not judged yet: `day` is not used, and the answer is
    /// `exception_for`'s for every day. Expiry is judged against `day` once `Exception` carries a required `expires`.
    pub fn exception_on(&self, finding: &Finding, _day: Date) -> Option<&Exception> {
        self.exception_for(finding)
    }
}
