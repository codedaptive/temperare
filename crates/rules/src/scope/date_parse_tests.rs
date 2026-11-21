//! Unit tests for date_parse.rs: exactly "YYYY-MM-DD" naming a real day, and nothing else.

use super::Date;

#[test]
fn a_well_formed_real_day_parses_to_that_day() {
    assert_eq!(Date::parse("2026-12-31"), Date::new(2026, 12, 31));
    assert!(Date::parse("2026-12-31").is_some());
}

#[test]
fn an_impossible_day_is_refused() {
    assert_eq!(Date::parse("2026-02-30"), None);
}

#[test]
fn a_short_field_is_refused() {
    assert_eq!(Date::parse("2026-2-03"), None);
    assert_eq!(Date::parse("26-02-03"), None);
}

#[test]
fn another_separator_is_refused() {
    assert_eq!(Date::parse("2026/02/03"), None);
}

#[test]
fn anything_before_or_after_is_refused() {
    assert_eq!(Date::parse(" 2026-02-03"), None);
    assert_eq!(Date::parse("2026-02-03 "), None);
    assert_eq!(Date::parse("2026-02-03T00:00"), None);
}

#[test]
fn empty_text_is_refused() {
    assert_eq!(Date::parse(""), None);
}

#[test]
fn non_ascii_digits_are_refused() {
    assert_eq!(Date::parse("２０２６-02-03"), None);
    // Ten bytes, so only the digit check refuses it: 'é' is the two bytes C3 A9.
    assert_eq!(Date::parse("20é-02-03"), None);
}

#[test]
fn an_ascii_non_digit_in_a_field_is_refused() {
    // ':' is the byte after '9', so read as a digit it would give month 10.
    assert_eq!(Date::parse("2026-0:-03"), None);
}

#[test]
fn a_sign_in_place_of_a_digit_is_refused() {
    assert_eq!(Date::parse("+026-02-03"), None);
}
