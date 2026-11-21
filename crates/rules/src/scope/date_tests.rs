//! Unit tests for date.rs: dates order by year, then month, then day.

use super::Date;

fn date(year: u16, month: u8, day: u8) -> Date {
    Date::new(year, month, day).unwrap()
}

#[test]
fn a_later_year_is_later_whatever_the_month_and_day() {
    assert!(date(2026, 12, 31) < date(2027, 1, 1));
}

#[test]
fn a_later_month_is_later_whatever_the_day() {
    assert!(date(2026, 1, 31) < date(2026, 2, 1));
}

#[test]
fn a_later_day_is_later() {
    assert!(date(2026, 2, 1) < date(2026, 2, 2));
}

#[test]
fn the_same_day_is_equal() {
    assert_eq!(date(2026, 2, 1), date(2026, 2, 1));
}
