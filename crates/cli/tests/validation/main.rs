//! Validation: tests that run the built `temperare` binary.
//!
//! `cli_check` and `cli_tm` run `temperare` with engines stubbed on PATH, except cli_check's `real_ruff_` cases;
//! `savings_real_engine`, `done_10_shared_run` and those cases run the real ruff.
//! `baseline_test_real_ruff` runs `run_baseline_test` with a const test set in a child of this test binary, with HOME
//! a folder the test made.
//!
//! Acceptance: the acceptance cases (one per feature, the shared run and the baseline test), run against the built
//! `temperare` with the real engines, each case in a throwaway folder it makes and removes. Validation, not a unit or
//! integration test: a run of the shipped binary.
//! `make accept` runs it.
//! No team-tool variable reaches `temperare` in any case: each case passes only the variables it sets (`env.rs`
//! for the acceptance cases, `install_temp_home.rs` its own).
//!
//! The report: each acceptance case records what it set up and saw in a `Check`, which writes the
//! case's part when ACCEPT_PARTS names a folder; `scripts/accept.sh` sets it and assembles the parts into the report,
//! one entry for every case checks.txt lists.

mod baseline_test_cmd;
mod baseline_test_real_ruff;
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
mod done_10_shared_run;
mod done_10_baseline_test;

// The install on the built binary, into a throwaway HOME.
mod install_temp_home;

// A clippy check line with allowed lints, on the built temperare.
mod clippy_line_reports_test_code_not_allowed_lints;
