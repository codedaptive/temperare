//! RIT for rules_world_real.rs: the Tessera seam of RulesWorld::real() is the real `engines::availability`, which
//! answers "not available" without a PATH lookup (`engines::availability` gives `availability_from` a `procio::which`
//! lookup; `availability_from` answers Tessera from its entry's `never` before any lookup). The other seams are tested
//! in tests/world/rules_world_real.rs (RIT, and ZIT for `exists`).

use super::super::RulesWorld;
use engines::Availability;
use rules::Engine;

#[test]
fn availability_says_tessera_is_not_available() {
    let world = RulesWorld::real();
    assert_eq!(
        (world.availability)(Engine::Tessera),
        Availability::NotAvailable { why: "Tessera's slop records are not defined yet".to_string() }
    );
}
