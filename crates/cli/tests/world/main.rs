//! Tests of tm's real worlds: `CheckWorld::real()` and `RulesWorld::real()` wired to real disk, git, engines and
//! report, on folders the tests make under the system temp folder and remove. This target holds both kinds beside
//! their caller, because there is no separate group for tests between crates: RITs (cli→procio/rules/engines) and
//! ZITs. Each file and case says which.

mod check_world_real;
mod rules_world_real;
