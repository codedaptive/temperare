//! `RuleRun::with_cause`: set a not-available rule run's cause.

use super::{NotAvailableCause, RuleRun};

impl RuleRun {
    /// This run with `not_available_cause` set to `cause`; every other field is kept as it is. A builder, as
    /// `with_no_files` is, so a not-available run is built in one expression from `new`.
    pub fn with_cause(self, cause: NotAvailableCause) -> Self {
        Self { not_available_cause: Some(cause), ..self }
    }
}
