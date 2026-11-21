//! `Date::parse`: a calendar day from its "YYYY-MM-DD" text.

use super::Date;

impl Date {
    /// Some only when `text` is exactly "YYYY-MM-DD" (ASCII digits, '-' between, nothing before or after) and names a
    /// real day (`Date::new`).
    pub fn parse(text: &str) -> Option<Date> {
        let bytes = text.as_bytes();
        if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
            return None;
        }
        let month = u8::try_from(number(&bytes[5..7])?).ok()?;
        let day = u8::try_from(number(&bytes[8..10])?).ok()?;
        Date::new(number(&bytes[0..4])?, month, day)
    }
}

/// The value of `digits` when every byte is an ASCII digit; None otherwise. The caller passes at most four digits, so
/// the value fits a u16.
fn number(digits: &[u8]) -> Option<u16> {
    digits
        .iter()
        .try_fold(0u16, |value, byte| byte.is_ascii_digit().then(|| value * 10 + u16::from(byte - b'0')))
}
