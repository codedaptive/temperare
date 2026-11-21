//! Unit tests for check_host.rs: the production host carries the base it is given and counts into the cell it is given.
//! Nothing is run or looked up.

use crate::check_host::check_host;
use std::cell::Cell;

#[test]
fn check_host_carries_the_base_and_the_raw_chars_cell_it_is_given() {
    for (base, want) in [(Some("abc123"), Some("abc123")), (None, None)] {
        let cell = Cell::new(None);
        let host = check_host(base, &cell);
        assert_eq!(host.base, want, "base {base:?}");
        host.raw_chars.set(Some(7));
        assert_eq!(cell.get(), Some(7), "base {base:?}");
    }
}
