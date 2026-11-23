//! The rubocop adapter: the rubocop entry (its command, the checks it refuses, the unknown-cop line it reads), the
//! reader of its JSON report, its flow with the environment that keeps it local, and the probes that make a rule
//! `Failed` when the repository's rubocop config hides a file or a cop from it (its todo file, a cop's Exclude beyond
//! the stock one, the files left out of the stock target list).

mod cop_excludes;
mod inspected_files;
mod left_out;
mod parse_rubocop;
mod rubocop_entry;
mod rubocop_probe;
mod rubocop_refusal;
mod run_rubocop;
mod run_rubocop_with;
mod todo_why;
mod unrecognized_cop;
mod with_rubocop_env;

#[cfg(test)]
mod cop_excludes_tests;
#[cfg(test)]
mod inspected_files_tests;
#[cfg(test)]
mod left_out_tests;
#[cfg(test)]
mod parse_rubocop_tests;
#[cfg(test)]
mod rubocop_entry_tests;
#[cfg(test)]
mod rubocop_probe_tests;
#[cfg(test)]
mod rubocop_refusal_tests;
#[cfg(test)]
mod run_rubocop_with_tests;
#[cfg(test)]
mod todo_why_tests;
#[cfg(test)]
mod unrecognized_cop_tests;
#[cfg(test)]
mod with_rubocop_env_tests;

pub(crate) use cop_excludes::cop_excludes;
pub(crate) use inspected_files::inspected_files;
pub(crate) use left_out::left_out;
pub use parse_rubocop::parse_rubocop;
pub use rubocop_entry::RUBOCOP;
pub(crate) use rubocop_probe::rubocop_probe;
pub(crate) use rubocop_refusal::rubocop_refusal;
pub(crate) use run_rubocop::run_rubocop;
pub(crate) use run_rubocop_with::run_rubocop_with;
pub(crate) use todo_why::todo_why;
pub(crate) use unrecognized_cop::unrecognized_cop;
pub(crate) use with_rubocop_env::with_rubocop_env;
