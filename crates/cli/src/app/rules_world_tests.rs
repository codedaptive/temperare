//! RulesWorld is plain data with no behaviour of its own; RulesWorld::real's disk and git seams are tested in
//! tests/world/rules_world_real.rs and tests/world/rules_loosened_real.rs (RIT, and ZIT where a case reaches no lower
//! kit), its availability seam (`availability_at`) in integration/availability_at_tests.rs and end to end in
//! tests/world/local_eslint_real.rs, its no_file_rules seam through the binary in
//! tests/world/rules_not_applicable_real.rs (ZIT). Its `now` reads the real clock, so no test asserts its value.
