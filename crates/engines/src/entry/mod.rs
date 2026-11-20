//! The engine entry: one engine described as data (its program, how a rule's check and params reach it, how its
//! version is read, its reader and its flow), and the machinery every entry shares: the command, the param
//! arguments, the temporary config, the version word, the too-old why and the standard flow.

mod config_params;
mod entry;
mod entry_cmd;
mod param_arg;
mod param_args;
mod param_why;
mod params;
mod pick;
mod read_fn;
mod run_fn;
mod run_plain;
mod temp_config;
mod temp_file;
mod temp_file_drop;
mod temp_file_write;
mod too_old_why;
mod version_probe;
mod version_word;

#[cfg(test)]
mod entry_cmd_tests;
#[cfg(test)]
mod integration;
#[cfg(test)]
mod param_args_tests;
#[cfg(test)]
mod param_why_tests;
#[cfg(test)]
mod run_plain_tests;
#[cfg(test)]
mod temp_config_tests;
#[cfg(test)]
mod too_old_why_tests;
#[cfg(test)]
mod version_word_tests;

pub use config_params::ConfigParams;
pub use entry::Entry;
pub use entry_cmd::entry_cmd;
pub use param_arg::ParamArg;
pub(crate) use param_args::param_args;
pub(crate) use param_why::param_why;
pub use params::Params;
pub use pick::Pick;
pub use read_fn::ReadFn;
pub(crate) use run_fn::RunFn;
pub(crate) use run_plain::run_plain;
pub(crate) use temp_config::temp_config;
pub use temp_file::TempFile;
pub(crate) use too_old_why::too_old_why;
pub use version_probe::VersionProbe;
pub use version_word::version_word;
