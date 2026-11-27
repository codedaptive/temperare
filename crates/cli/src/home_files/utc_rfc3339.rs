//! `utc_rfc3339`: a Unix time as UTC RFC 3339 text, to the second.

const SECS_PER_DAY: u64 = 86_400;

/// `unix_secs` as `YYYY-MM-DDTHH:MM:SSZ`, in UTC.
pub(crate) fn utc_rfc3339(unix_secs: u64) -> String {
    let (year, month, day) = civil_from_days(unix_secs / SECS_PER_DAY);
    let secs = unix_secs % SECS_PER_DAY;
    let (hour, minute, second) = (secs / 3600, secs % 3600 / 60, secs % 60);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

/// The proleptic Gregorian (year, month, day) of `days` after 1970-01-01: Howard Hinnant's `civil_from_days`,
/// unsigned since a Unix time here is never before the epoch. Eras are 400-year cycles counted from 0000-03-01, so
/// the leap day falls at the end of the shifted year.
fn civil_from_days(days: u64) -> (u64, u64, u64) {
    let shifted = days + 719_468;
    let era = shifted / 146_097;
    let day_of_era = shifted - era * 146_097;
    let year_of_era = (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_from_march = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_from_march + 2) / 5 + 1;
    let month = if month_from_march < 10 { month_from_march + 3 } else { month_from_march - 9 };
    let year = year_of_era + era * 400 + u64::from(month <= 2);
    (year, month, day)
}
