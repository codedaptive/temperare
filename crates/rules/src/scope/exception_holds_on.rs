//! `Exception::holds_on`: whether an exception is still in force on a day.

use super::{Date, Exception};

impl Exception {
    /// True when the exception has no `expires`, or `day` is on or before it. The exception holds through its
    /// `expires` day in UTC: that day it still covers, the day after it does not. `day` is a UTC day the caller chose.
    pub(crate) fn holds_on(&self, day: Date) -> bool {
        self.expires.is_none_or(|expires| day <= expires)
    }
}
