//! Unit tests for exception_holds_on.rs: the exception holds through its `expires` day, not the day after.

use crate::{Date, Exception};

#[test]
fn an_exception_holds_through_its_expires_day() {
    // (expires, day, holds)
    let rows = [
        (Date::new(2026, 12, 31), Date::new(2026, 12, 30), true),
        (Date::new(2026, 12, 31), Date::new(2026, 12, 31), true),
        (Date::new(2026, 12, 31), Date::new(2027, 1, 1), false),
        (None, Date::new(2099, 12, 31), true),
    ];
    for (expires, day, holds) in rows {
        let e = Exception { expires, ..Exception::new("no-unwrap", "src/a.rs", "legacy", "Bob, 2026-11-17") };
        let day = day.unwrap();
        assert_eq!(e.holds_on(day), holds, "expires {expires:?} on {day:?}");
    }
}
