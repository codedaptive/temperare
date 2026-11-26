//! The engine entry: one engine described as data (its program, how a rule's check, params and thresholds reach it,
//! how its version is read, its reader and its flow), and the machinery every entry shares: the command, the param
//! arguments, the threshold's arguments and its temporary file or folder, the version word, the too-old why, the list
//! floor and the standard flow.

mod entry;
mod entry_cmd;
mod list_floor;
mod list_version;
mod param_arg;
mod param_args;
mod param_why;
mod params;
mod pick;
mod reach;
mod read_fn;
mod run_fn;
mod run_plain;
mod temp_dir;
mod temp_dir_drop;
mod temp_dir_write;
mod temp_file;
mod temp_file_drop;
mod temp_file_write;
mod threshold;
mod threshold_of;
mod threshold_run;
mod threshold_run_type;
mod too_old_why;
mod version_probe;
mod version_word;
mod whole_in;

#[cfg(test)]
mod entry_cmd_tests;
#[cfg(test)]
mod integration;
#[cfg(test)]
mod list_floor_tests;
#[cfg(test)]
mod list_version_tests;
#[cfg(test)]
mod param_args_tests;
#[cfg(test)]
mod param_why_tests;
#[cfg(test)]
mod run_plain_tests;
#[cfg(test)]
mod threshold_of_tests;
#[cfg(test)]
mod threshold_run_tests;
#[cfg(test)]
mod too_old_why_tests;
#[cfg(test)]
mod version_word_tests;
#[cfg(test)]
mod whole_in_tests;

pub use entry::Entry;
pub use entry_cmd::entry_cmd;
pub(crate) use list_floor::list_floor;
pub(crate) use list_version::list_version;
pub use param_arg::ParamArg;
pub(crate) use param_args::param_args;
pub(crate) use param_why::param_why;
pub use params::Params;
pub use pick::Pick;
pub use reach::Reach;
pub use read_fn::ReadFn;
pub(crate) use run_fn::RunFn;
pub(crate) use run_plain::run_plain;
pub use temp_dir::TempDir;
pub use temp_file::TempFile;
pub use threshold::Threshold;
pub(crate) use threshold_of::threshold_of;
pub(crate) use threshold_run::threshold_run;
pub(crate) use threshold_run_type::ThresholdRun;
pub(crate) use too_old_why::too_old_why;
pub use version_probe::VersionProbe;
pub use version_word::version_word;
pub(crate) use whole_in::whole_in;
