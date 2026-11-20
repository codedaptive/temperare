//! Unit tests for date_text.rs: the exact text of hand-picked days, padding included.

use crate::date_text;

#[test]
fn a_two_digit_month_and_day_are_written_as_they_are() {
    assert_eq!(date_text(2026, 11, 20), "2026-11-20");
}

#[test]
fn the_epoch_day_pads_month_and_day() {
    assert_eq!(date_text(1970, 1, 1), "1970-01-01");
}

#[test]
fn a_leap_day_is_written_as_given() {
    assert_eq!(date_text(2000, 2, 29), "2000-02-29");
}

#[test]
fn the_last_day_rules_date_holds() {
    assert_eq!(date_text(9999, 12, 31), "9999-12-31");
}

#[test]
fn a_short_year_pads_to_four_digits() {
    assert_eq!(date_text(987, 6, 5), "0987-06-05");
}
