//! `utc_day`: the UTC calendar day holding a second since the Unix epoch.

/// The last second of 9999-12-31 UTC, the last day `rules::Date` holds.
const LAST_SECOND: u64 = 253_402_300_799;

/// The UTC (year, month, day) holding `secs_since_epoch`; a second past the end of 9999-12-31 gives that day. The
/// clamp comes first, so even `u64::MAX` cannot overflow the arithmetic, and every answer is a real Gregorian day in
/// years 1970..=9999. The numbers come back as a tuple rather than a `rules::Date` because `Date` has no accessors.
///
/// Days to civil date by eras of 400 years (146097 days), counted from 0000-03-01 so the leap day ends each year.
pub(crate) fn utc_day(secs_since_epoch: u64) -> (u16, u8, u8) {
    let days = secs_since_epoch.min(LAST_SECOND) / 86_400;
    // 719468 days from 0000-03-01 to 1970-01-01.
    let from_march_0 = days + 719_468;
    let era = from_march_0 / 146_097;
    let day_of_era = from_march_0 % 146_097;
    let year_of_era = (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    // Months from March: 0 is March, 10 and 11 are January and February of the next year.
    let month_from_march = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_from_march + 2) / 5 + 1;
    let (month, next_year) = if month_from_march < 10 { (month_from_march + 3, 0) } else { (month_from_march - 9, 1) };
    // The clamp bounds the year to 9999, the month to 12 and the day to 31, so the narrowing loses nothing.
    ((era * 400 + year_of_era + next_year) as u16, month as u8, day as u8)
}
