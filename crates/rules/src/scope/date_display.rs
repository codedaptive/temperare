//! `Display for Date`: a calendar day as its "YYYY-MM-DD" text.

use super::Date;
use std::fmt;

impl fmt::Display for Date {
    /// "YYYY-MM-DD", each field zero padded, the form `Date::parse` reads.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}
