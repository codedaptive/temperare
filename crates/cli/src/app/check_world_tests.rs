//! CheckWorld is plain data with no behaviour of its own; CheckWorld::real's wiring of its git, disk, engine and
//! report seams is tested in tests/world/check_world_real.rs (an interface test with the lower crates, and a zone integration test where a case reaches no impure lower-crate
//! code). Its `home` and `now` read the real environment and clock, so no test asserts their values.
