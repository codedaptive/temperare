//! `utc_date`: the UTC calendar day holding a second since the Unix epoch, as a `rules::Date`.

use crate::utc_day;
use rules::Date;

/// The UTC day holding `secs_since_epoch`, as a `Date`; it reads no clock, so the caller passes the second. A second
/// past the end of 9999-12-31 gives that day. It cannot panic: `utc_day` clamps first and returns only real Gregorian
/// days in years 1970..=9999, and `Date::new` accepts every one of them.
pub fn utc_date(secs_since_epoch: u64) -> Date {
    let (year, month, day) = utc_day(secs_since_epoch);
    Date::new(year, month, day).expect("utc_day returns only real days in 1970..=9999 (see utc_day's doc)")
}
