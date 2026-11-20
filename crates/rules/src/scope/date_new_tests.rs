//! ZUT for date_new.rs: only real Gregorian days are built.

use super::Date;

#[test]
fn february_29_exists_only_in_a_leap_year() {
    let d = Date::new(2024, 2, 29).unwrap();
    assert_eq!((d.year, d.month, d.day), (2024, 2, 29));
    assert!(Date::new(2026, 2, 29).is_none());
}

#[test]
fn a_century_is_a_leap_year_only_when_divisible_by_400() {
    let d = Date::new(2000, 2, 29).unwrap();
    assert_eq!((d.year, d.month, d.day), (2000, 2, 29));
    assert!(Date::new(1900, 2, 29).is_none());
}

#[test]
fn a_day_past_the_months_length_is_refused() {
    assert!(Date::new(2026, 2, 30).is_none());
    assert!(Date::new(2026, 4, 31).is_none());
    let d = Date::new(2026, 4, 30).unwrap();
    assert_eq!((d.year, d.month, d.day), (2026, 4, 30));
}

#[test]
fn month_and_day_out_of_range_are_refused() {
    assert!(Date::new(2026, 0, 1).is_none());
    assert!(Date::new(2026, 13, 1).is_none());
    assert!(Date::new(2026, 1, 0).is_none());
}

#[test]
fn year_out_of_range_is_refused() {
    assert!(Date::new(0, 1, 1).is_none());
    assert!(Date::new(10000, 1, 1).is_none());
}

#[test]
fn the_last_day_of_year_9999_exists() {
    let d = Date::new(9999, 12, 31).unwrap();
    assert_eq!((d.year, d.month, d.day), (9999, 12, 31));
}

#[test]
fn the_fields_come_out_as_given() {
    let d = Date::new(2026, 11, 20).unwrap();
    assert_eq!((d.year, d.month, d.day), (2026, 11, 20));
}
