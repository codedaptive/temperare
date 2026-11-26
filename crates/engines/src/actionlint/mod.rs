//! The actionlint adapter: its entry (`ACTIONLINT`) and the flow it runs for one rule, the refusal of a check that is
//! not an actionlint kind's form, which files are workflows (`is_workflow`) and the no-file outcome before
//! availability (settled by `engine_listed`, in `engine_hits_with`'s step 1), the config tm passes and the `ignore`
//! in it tm cannot turn off (read with `yaml_entries`), the ShellCheck and pyflakes integrations a check turns on, a
//! kind the run did not define, and the reader of its JSON.

mod actionlint_config;
mod actionlint_entry;
mod actionlint_refusal;
mod config_suppression;
mod integration_args;
mod is_workflow;
mod parse_actionlint;
mod run_actionlint;
mod unknown_kind;

#[cfg(test)]
mod actionlint_config_tests;
#[cfg(test)]
mod actionlint_entry_tests;
#[cfg(test)]
mod actionlint_refusal_tests;
#[cfg(test)]
mod config_suppression_tests;
#[cfg(test)]
mod integration_args_tests;
#[cfg(test)]
mod is_workflow_tests;
#[cfg(test)]
mod parse_actionlint_tests;
#[cfg(test)]
mod unknown_kind_tests;

pub(crate) use actionlint_config::actionlint_config;
pub use actionlint_entry::ACTIONLINT;
pub(crate) use actionlint_refusal::actionlint_refusal;
pub(crate) use config_suppression::config_suppression;
pub(crate) use integration_args::integration_args;
pub(crate) use is_workflow::is_workflow;
pub use parse_actionlint::parse_actionlint;
pub(crate) use run_actionlint::run_actionlint;
pub(crate) use unknown_kind::unknown_kind;
