//! Tests of tm's real worlds: `CheckWorld::real()` (git, disk, engines, report), `RulesWorld::real()` (git, disk,
//! engines), and `BaselineWorld::real()` (git, disk, rules, engines), on folders the tests make under the system temp
//! folder and remove, or on paths under it that do not exist. This target holds both kinds beside their caller,
//! because there is no separate group for tests between crates: RITs (cli→procio/rules/engines/report) and ZITs. Each
//! file and case says which. `baseline_git_env` checks isolation from inherited git settings; `check_base_real` runs
//! the binary against a git rule with and without a base; `baseline_change_real` runs a change-only baseline rule
//! against real bases. `rules_loosened_real` compares rule files against real git bases; `rules_not_applicable_real`
//! compares the rules listing with a check in a throwaway repository.

mod baseline_change_real;
mod baseline_git_env;
mod baseline_world_real;
mod check_base_real;
mod check_world_real;
// Unix only: engines take node_modules/.bin/eslint only as that extensionless file and procio runs a path as given,
// so a Windows `.cmd` stub cannot stand in.
#[cfg(unix)]
mod local_eslint_real;
mod rules_loosened_real;
mod rules_not_applicable_real;
mod rules_world_real;
