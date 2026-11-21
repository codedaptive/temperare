//! Unit tests for utc_date.rs: expected days written out by hand from the calendar.

use crate::utc_date;
use rules::Date;

#[test]
fn utc_date_gives_the_utc_day_holding_the_second() {
    let rows = [
        (86_399, Date::new(1970, 1, 1).unwrap(), "the last second of the epoch day"),
        (86_400, Date::new(1970, 1, 2).unwrap(), "the first second of the next day"),
        (951_782_400, Date::new(2000, 2, 29).unwrap(), "a leap day in a 400 year"),
        (951_868_800, Date::new(2000, 3, 1).unwrap(), "the day after the leap day"),
        (1_735_689_599, Date::new(2024, 12, 31).unwrap(), "the last second of 2024"),
        (1_735_689_600, Date::new(2025, 1, 1).unwrap(), "the first second of 2025"),
        (u64::MAX, Date::new(9999, 12, 31).unwrap(), "a second past 9999-12-31 gives that day"),
    ];
    for (secs, expected, label) in rows {
        assert_eq!(utc_date(secs), expected, "{label}");
    }
}
