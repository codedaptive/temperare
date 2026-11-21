//! Tests of tm's real worlds: `CheckWorld::real()` (git, disk, engines, report), `RulesWorld::real()` (git, disk,
//! engines), and `BaselineWorld::real()` (git, disk, rules, engines), on folders the tests make under the system temp
//! folder and remove, or on paths under it that do not exist. This target holds both kinds beside their caller,
//! because there is no separate group for tests between crates: RITs (cli→procio/rules/engines/report) and ZITs. Each
//! file and case says which. `baseline_git_env` checks isolation from inherited git settings.

mod baseline_git_env;
mod baseline_world_real;
mod check_world_real;
mod rules_world_real;
