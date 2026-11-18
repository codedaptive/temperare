//! `Check::not_run`: records that the case could not run, and why.

use crate::check::Check;

impl Check {
    /// The case could not run; `why` says what was missing.
    pub fn not_run(&mut self, why: impl Into<String>) {
        self.not_run = Some(why.into());
    }
}
