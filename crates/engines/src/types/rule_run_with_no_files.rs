//! `RuleRun::with_no_files`: flag a rule run as having run nothing because no file it checks exists.

use super::RuleRun;

impl RuleRun {
    /// This run with `no_files` set to `why`, the reason no file it checks exists; every other field is kept as it
    /// is. A builder rather than a field set in place, so a flagged run is built in one expression from `new`.
    pub fn with_no_files(self, why: impl Into<String>) -> Self {
        Self { no_files: Some(why.into()), ..self }
    }
}
