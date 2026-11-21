//! `Exception`: one exception, as an `[[exception]]` of the rule file or built by `Exception::new`.

use crate::{Date, RelPath};

/// An approved exception to one rule at one path; `what`, when given, limits it to findings whose `what` contains it,
/// and `expires`, when given, ends it after that day.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exception {
    pub rule: String,
    pub path: RelPath,
    pub what: Option<String>,
    /// The last day the exception holds, in UTC; None: no end date.
    pub expires: Option<Date>,
    pub reason: String,
    pub approved: String,
}
