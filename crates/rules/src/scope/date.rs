//! `Date`: a calendar day.

/// A calendar day, ordered by year, then month, then day (the field order). Which day is today, and in which time zone
/// (UTC), is the caller's business: no clock is read in this crate. `Date::new` and `Date::parse` build only real days.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    pub(super) year: u16,
    pub(super) month: u8,
    pub(super) day: u8,
}
