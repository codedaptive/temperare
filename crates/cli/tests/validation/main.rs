//! Validation: tests that run the built `temperare` binary.
//!
//! `cli_check` and `cli_tm` run `temperare` with engines stubbed on PATH; `savings_real_engine` checks savings with ruff.
//!
//! Acceptance: the mission's Done list (MISSION-temperare-2026-11-16, Done 1-8), run against the built `temperare` with the
//! real engines, each case in a throwaway repository it makes and removes. Validation, not a unit or integration test:
//! a run of the shipped binary, written by the orchestrator (SPEC-cli, Tests). `make accept` runs it.
//! No codeteam variable reaches `temperare` in any case (Done 6): each case passes only the variables it sets (`env.rs`
//! for the acceptance cases, `install_temp_home.rs` its own).
//!
//! The report (SPEC-accept): each acceptance case records what it set up and saw in a `Check`, which writes the
//! case's part when ACCEPT_PARTS names a folder; `scripts/accept.sh` sets it and assembles the parts into the report,
//! one entry for every case checks.txt lists.

mod cli_check;
mod cli_tm;
mod savings_real_engine;

mod check;
mod check_drop;
mod check_new;
mod check_not_run;
mod check_saw_tm;
mod clean_cmd;
mod env;
mod on_tm_path;
mod repo;
mod run_clean;
mod temp_home;
mod tm;
mod tm_lines;
mod write_head_once;

mod done_1_clippy;
mod done_1_swiftlint;
mod done_2_new_only;
mod done_3_house_rule;
mod done_4_absent_engine;
mod done_5_exception;
mod done_6_alone;
mod done_7_security;
mod done_8_named_inside;

// The install on the built binary, into a throwaway HOME.
mod install_temp_home;

// Objective tatertot Stage 2 cases on the built temperare.
mod stage2_mortar_clippy_line;
