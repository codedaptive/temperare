//! `RuleFile::exceptions_in_force`: every exception the file grants, for the report's count.

use super::RuleFile;
use crate::Exception;

impl RuleFile {
    /// Every exception in the file, in file order.
    pub fn exceptions_in_force(&self) -> &[Exception] {
        &self.exceptions
    }
}
