//! RulesWorld is plain data with no behaviour of its own; RulesWorld::real's disk and git seams are tested in
//! tests/world/rules_world_real.rs and tests/world/rules_loosened_real.rs (cross-crate integration tests, and zone integration tests where a case reaches no lower
//! kit), its Tessera seam in integration/rules_world_real_tests.rs. Its `now` reads the real clock, so no test asserts
//! its value.
