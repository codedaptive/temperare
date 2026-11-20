//! Tests of tm's real worlds: `CheckWorld::real()` and `RulesWorld::real()` wired to real disk, git, engines and
//! report, on folders the tests make under the system temp folder and remove. This target holds both kinds beside
//! their caller, since there is no separate group for cross-crate tests: cross-crate integration tests
//! (cli→procio/rules/engines) and zone integration tests (inside cli). Each file and case says which.

mod check_world_real;
mod rules_world_real;
