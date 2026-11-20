//! Unit tests for rules_world_real.rs: the Tessera seam of RulesWorld::real() answers "not available" without a PATH lookup
//! (crates/engines/src/availability.rs:10 builds the `procio::which` closure; :17-18 return for Tessera before it
//! runs). The other seams are tested in tests/world/rules_world_real.rs (cross-crate integration tests, and a zone
//! integration test for `exists`).

use super::RulesWorld;
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
