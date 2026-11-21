//! `RuleFile::exception_on`: the exception that covers a finding on a given day.

use super::RuleFile;
use crate::{Date, Exception, Finding};

impl RuleFile {
    /// The first exception in file order that covers `finding` and holds on `day`. An exception holds through its
    /// `expires` day in UTC; from the day after, it covers nothing and a later exception may answer instead. An
    /// exception with no `expires` holds every day.
    pub fn exception_on(&self, finding: &Finding, day: Date) -> Option<&Exception> {
        self.exceptions.iter().find(|e| e.covers_finding(finding) && e.holds_on(day))
    }
}
