use super::save_all;
use crate::store::MemoryStore;

#[test]
fn saves_every_line() {
    let mut store = MemoryStore::default();
    save_all(&mut store, &["a", "b"]);
    assert_eq!(store.lines(), ["a", "b"]);
}
