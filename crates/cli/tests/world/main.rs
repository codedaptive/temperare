//! Tests of tm's real worlds: `CheckWorld::real()` (git, disk, engines, report) and `RulesWorld::real()` (git, disk,
//! engines), on folders the tests make under the system temp folder and remove, or on paths under it that do not
//! exist. This target holds both kinds beside their caller, because there is no separate group for tests between
//! crates: RITs (cli→procio/rules/engines/report) and ZITs. Each file and case says which.

mod check_world_real;
mod rules_world_real;
