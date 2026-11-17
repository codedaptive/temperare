//! Validation (S-28): tests that run the built `tm` binary.
//!
//! `cli_check` and `cli_tm` are cli's runs of `tm` with the engines stubbed on PATH.
//!
//! Acceptance: the mission's Done list (MISSION-temperare-2026-11-16, Done 1-8), run against the built `tm` with the
//! real engines, each case in a throwaway repository it makes and removes. Validation, not a unit or integration test:
//! a run of the shipped binary (rule 11), written by the orchestrator (SPEC-cli, Tests). `make accept` runs it.
//! No codeteam variable reaches `tm` in any case (Done 6): `env.rs` builds its environment from nothing.

mod cli_check;
mod cli_tm;

mod env;
mod repo;
mod run_clean;
mod tm;

mod done_1_clippy;
mod done_2_new_only;
mod done_3_house_rule;
mod done_4_absent_engine;
mod done_5_exception;
mod done_6_alone;
mod done_7_security;
mod done_8_named_inside;
