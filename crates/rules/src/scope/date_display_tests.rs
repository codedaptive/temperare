//! Unit tests for date_display.rs: "YYYY-MM-DD", zero padded, read back by `Date::parse`.

use super::Date;

#[test]
fn a_date_is_written_yyyy_mm_dd_zero_padded_and_parses_back() {
    let rows = [
        (Date::new(2026, 12, 31), "2026-12-31"),
        (Date::new(1, 2, 3), "0001-02-03"),
        (Date::new(2024, 2, 29), "2024-02-29"),
    ];
    for (date, text) in rows {
        let date = date.unwrap();
        assert_eq!(date.to_string(), text);
        assert_eq!(Date::parse(text), Some(date), "{text}");
    }
}
