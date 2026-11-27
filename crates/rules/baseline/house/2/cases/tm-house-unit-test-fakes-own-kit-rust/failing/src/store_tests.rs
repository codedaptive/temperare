use super::save_all;

struct FakeStore {
    saved: Vec<String>,
}

impl crate::store::Store for FakeStore {
    fn save(&mut self, line: &str) {
        self.saved.push(line.to_string());
    }
}

#[test]
fn saves_every_line() {
    let mut store = FakeStore { saved: Vec::new() };
    save_all(&mut store, &["a", "b"]);
    assert_eq!(store.saved, ["a", "b"]);
}
