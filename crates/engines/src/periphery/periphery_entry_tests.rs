use super::{periphery_refusal, PERIPHERY};
use crate::entry::Pick;
use rules::Engine;

#[test]
fn periphery_entry_fields() {
    assert_eq!(PERIPHERY.name, "periphery");
    assert_eq!(PERIPHERY.engine, Engine::Periphery);
    assert_eq!(PERIPHERY.needs, ["periphery"]);
    assert_eq!(PERIPHERY.pick, Pick::None);
    assert_eq!(PERIPHERY.version.unwrap().args, ["version"]);
    assert_eq!((PERIPHERY.refuse)("Unused"), periphery_refusal("Unused"));
}
