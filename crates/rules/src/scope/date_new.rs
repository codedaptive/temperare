//! `Date::new`: a calendar day from its numbers.

use super::Date;

impl Date {
    /// Some only for a real Gregorian day: year 1 to 9999, month 1 to 12, day 1 to the month's length. February has
    /// 29 days in a leap year: one divisible by 4, except a century not divisible by 400.
    pub fn new(year: u16, month: u8, day: u8) -> Option<Date> {
        if !(1..=9999).contains(&year) || day == 0 || day > month_length(year, month)? {
            return None;
        }
        Some(Date { year, month, day })
    }
}

/// The number of days in `month` of `year`; None for a month outside 1 to 12.
fn month_length(year: u16, month: u8) -> Option<u8> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => Some(29),
        2 => Some(28),
        _ => None,
    }
}
