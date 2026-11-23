//! The cargo-audit adapter: its entry (`CARGO_AUDIT`) and the flow it runs for one rule, the refusal of a check that
//! is not an advisory id or kind, the reader of its JSON report, its command, where its advisory database is, whether
//! that database holds an id, and the no-file outcome of a folder without a Cargo.lock in the rule's paths.

mod advisory_db;
mod cargo_audit_cmd;
mod cargo_audit_entry;
mod cargo_audit_refusal;
mod has_lock;
mod known_advisory;
mod lock_listed;
mod parse_cargo_audit;
mod run_cargo_audit;

#[cfg(test)]
mod advisory_db_tests;
#[cfg(test)]
mod cargo_audit_cmd_tests;
#[cfg(test)]
mod cargo_audit_entry_tests;
#[cfg(test)]
mod cargo_audit_refusal_tests;
#[cfg(test)]
mod lock_listed_tests;
#[cfg(test)]
mod parse_cargo_audit_tests;

pub(crate) use advisory_db::advisory_db;
pub use cargo_audit_cmd::cargo_audit_cmd;
pub use cargo_audit_entry::CARGO_AUDIT;
pub(crate) use cargo_audit_refusal::cargo_audit_refusal;
pub(crate) use has_lock::has_lock;
pub(crate) use known_advisory::known_advisory;
pub(crate) use lock_listed::lock_listed;
pub use parse_cargo_audit::parse_cargo_audit;
pub(crate) use run_cargo_audit::run_cargo_audit;
