//! Validation: tests that run the built `cctao` binary, the command of Code Chi Tao.
//!
//! `cli_check` and `cli_cctao` run `cctao` with engines stubbed on PATH, except cli_check's `real_ruff_` cases;
//! `savings_real_engine`, `done_10_shared_run` and those cases run the real ruff; `check_text_na_count_real_engines`
//! runs the real ruff and ast-grep.
//! `house_2_old_citation_not_reported` runs the real ast-grep.
//! `baseline_test_real_ruff` runs `run_baseline_test` with a const test set in a child of this test binary, with HOME
//! a folder the test made.
//!
//! Acceptance: the mission's Done list (Done 1-8, and Done 9 for
//! golangci-lint, pylint, clang-tidy, vulture, Periphery, gitleaks, cargo-audit, eslint, mypy, PMD, detekt, RuboCop,
//! PHPStan, dart analyze, markdownlint-cli2, yamllint, actionlint, hadolint, Vale, stylelint, SQLFluff, ShellCheck
//! and git, and Done 10's shared run and baseline test), run against the built `cctao` with the real engines,
//! each case in a throwaway folder it makes and removes.
//! Validation, not a unit or integration test: a run of the shipped binary, written by the cli crate's
//! authors to the project's measures.
//! `make accept` runs it.
//! No team-harness variable reaches `cctao` in any case (Done 6): each case passes only the variables it sets (`env.rs`
//! for the acceptance cases, `install_temp_home.rs` its own).
//!
//! The report: each acceptance case records what it set up and saw in a `Check`, which writes the
//! case's part when ACCEPT_PARTS names a folder; `scripts/accept.sh` sets it and assembles the parts into the report,
//! one entry for every case checks.txt lists.

mod baseline_test_cmd;
mod baseline_test_real_ruff;
mod check_text_na_count_real_engines;
mod cli_cctao;
// Unix only: its engine stubs are sh scripts and its PATH is the stubs folder alone.
#[cfg(unix)]
mod cli_check;
mod house_2_old_citation_not_reported;
mod savings_real_engine;

mod absent_engine_reason;
mod cctao;
mod cctao_lines;
mod cctao_path_folder;
mod cctao_path_for;
mod check;
mod check_drop;
mod check_new;
mod check_not_run;
mod check_saw_cctao;
mod clean_cmd;
mod env;
mod not_available_pairs;
mod on_cctao_path;
mod probed_version;
mod repo;
mod run_clean;
mod temp_home;
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
mod done_9_golangci_lint;
mod done_9_pylint;
mod done_9_clang_tidy;
mod done_9_vulture;
mod done_9_periphery;
mod done_9_gitleaks;
mod done_9_cargo_audit;
mod done_9_eslint;
mod done_9_mypy;
mod done_9_pmd;
mod done_9_detekt;
mod done_9_rubocop;
mod done_9_phpstan;
mod done_9_dart;
mod done_9_markdownlint;
mod done_9_yamllint;
mod done_9_actionlint;
mod done_9_hadolint;
mod done_9_vale;
mod done_9_stylelint;
mod done_9_sqlfluff;
mod done_9_shellcheck;
mod done_9_git;
mod done_10_shared_run;
mod done_10_baseline_test;

// The install on the built binary, into a throwaway HOME.
// Unix only: it checks the installed binary's file mode.
#[cfg(unix)]
mod install_temp_home;

// A clippy check line with allowed lints, on the built cctao.
mod clippy_line_reports_test_code_not_allowed_lints;
