//! Unit tests for utc_day.rs: expected days written out by hand from the calendar.

use crate::utc_day;

#[test]
fn the_epoch_is_1970_01_01() {
    assert_eq!(utc_day(0), (1970, 1, 1));
}

#[test]
fn a_leap_day_in_a_400_year_is_found() {
    assert_eq!(utc_day(951_782_400), (2000, 2, 29));
}

#[test]
fn the_first_and_last_second_of_a_day_give_that_day() {
    assert_eq!(utc_day(1_735_603_200), (2024, 12, 31));
    assert_eq!(utc_day(1_735_689_599), (2024, 12, 31));
}

#[test]
fn the_next_second_is_the_next_year() {
    assert_eq!(utc_day(1_735_689_600), (2025, 1, 1));
}

#[test]
fn a_century_not_divisible_by_400_has_no_leap_day() {
    assert_eq!(utc_day(4_107_456_000), (2100, 2, 28));
    assert_eq!(utc_day(4_107_542_400), (2100, 3, 1));
}

#[test]
fn a_2026_day() {
    assert_eq!(utc_day(1_795_132_800), (2026, 11, 20));
}

#[test]
fn the_last_day_held_is_9999_12_31() {
    assert_eq!(utc_day(253_402_214_400), (9999, 12, 31));
}

#[test]
fn seconds_past_9999_12_31_give_9999_12_31() {
    assert_eq!(utc_day(253_402_300_800), (9999, 12, 31));
    assert_eq!(utc_day(u64::MAX), (9999, 12, 31));
}
