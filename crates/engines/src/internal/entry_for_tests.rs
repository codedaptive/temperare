//! ZUT for entry_for.rs: every engine returns the same entry as entry_of.

use crate::entry_for::entry_for;
use crate::entry_of::entry_of;
use rules::Engine;

#[test]
fn entry_for_returns_the_entry_of_every_engine() {
    for engine in Engine::ALL {
        let entry = entry_for(engine).unwrap_or_else(|| panic!("missing entry for {engine:?}"));
        assert!(std::ptr::eq(entry, entry_of(engine)), "{engine:?}");
    }
}
