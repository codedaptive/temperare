//! `RuleFile::exceptions_in_force`: every exception the file grants, for the report's count.

use super::RuleFile;
use crate::Exception;

impl RuleFile {
    /// Every exception in the file, in file order, expired or not: it takes no day. `exceptions_expired` lists those
    /// past their date.
    pub fn exceptions_in_force(&self) -> &[Exception] {
        &self.exceptions
    }
}
