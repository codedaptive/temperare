//! `RuleFile::exceptions_expired`: the exceptions past their date.

use super::RuleFile;
use crate::{Date, Exception};

impl RuleFile {
    /// Every exception that does not hold on `day`, in file order: those whose `expires` is before `day`. An
    /// exception holds through its `expires` day in UTC, so it is listed from the day after. An exception with no
    /// `expires` is never listed.
    pub fn exceptions_expired(&self, day: Date) -> Vec<&Exception> {
        self.exceptions.iter().filter(|e| !e.holds_on(day)).collect()
    }
}
