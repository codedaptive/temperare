//! The hadolint adapter: its entry (`HADOLINT`), the flow it runs for one rule and its command; the refusal of a check
//! that is not one hadolint code, cctao's list of hadolint codes (`hadolint_codes.txt`) and the stop for a code not
//! on it, which files are Dockerfiles and the no-file outcome before availability, the config cctao pins and whether it
//! hides findings through `ignored`, the `HADOLINT_*` variables removed from the run, and the reader of hadolint's
//! SARIF output.

mod hadolint_cmd;
mod hadolint_config;
mod hadolint_entry;
mod hadolint_refusal;
mod hadolint_vars;
mod ignored_listed;
mod is_dockerfile;
mod not_listed;
mod parse_hadolint;
mod run_hadolint;

#[cfg(test)]
mod hadolint_cmd_tests;
#[cfg(test)]
mod hadolint_config_tests;
#[cfg(test)]
mod hadolint_entry_tests;
#[cfg(test)]
mod hadolint_refusal_tests;
#[cfg(test)]
mod hadolint_vars_tests;
#[cfg(test)]
mod ignored_listed_tests;
#[cfg(test)]
mod is_dockerfile_tests;
#[cfg(test)]
mod not_listed_tests;
#[cfg(test)]
mod parse_hadolint_tests;

pub(crate) use hadolint_cmd::hadolint_cmd;
pub(crate) use hadolint_config::hadolint_config;
pub use hadolint_entry::HADOLINT;
pub(crate) use hadolint_refusal::hadolint_refusal;
pub(crate) use hadolint_vars::hadolint_vars;
pub(crate) use ignored_listed::ignored_listed;
pub(crate) use is_dockerfile::is_dockerfile;
pub(crate) use not_listed::not_listed;
pub use parse_hadolint::parse_hadolint;
pub(crate) use run_hadolint::run_hadolint;
