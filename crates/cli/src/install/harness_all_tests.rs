//! Unit tests for harness_all.rs: the three harnesses, in order.

use super::Harness;

#[test]
fn all_lists_the_three_harnesses_in_order() {
    assert_eq!(Harness::ALL, [Harness::Claude, Harness::Codex, Harness::Cursor]);
}
