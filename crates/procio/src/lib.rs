//! procio: the one place that runs another program. Depends on nothing.

mod cmd;
mod cmd_new;
mod command_from;
mod effective_path;
mod first_in_dir;
mod has_path_separator;
mod is_executable;
#[cfg(unix)]
mod kill_group_cmd;
mod names_to_try;
mod output;
mod pathext;
mod proc_error;
mod proc_error_display;
mod proc_error_source;
mod run;
mod spawn_error;
mod spawn_reader;
mod spawn_writer;
mod wait_deadline;
mod which;
mod which_in_dir;

pub use cmd::Cmd;
pub use output::Output;
pub use proc_error::ProcError;
pub use run::run;
pub use which::which;
pub use which_in_dir::which_in_dir;

#[cfg(test)]
mod cmd_new_tests;
#[cfg(test)]
mod cmd_tests;
#[cfg(test)]
mod command_from_tests;
#[cfg(test)]
mod effective_path_tests;
#[cfg(test)]
mod has_path_separator_tests;
#[cfg(all(test, unix))]
mod kill_group_cmd_tests;
#[cfg(test)]
mod names_to_try_tests;
#[cfg(test)]
mod output_tests;
#[cfg(test)]
mod proc_error_display_tests;
#[cfg(test)]
mod proc_error_source_tests;
#[cfg(test)]
mod proc_error_tests;
#[cfg(test)]
mod spawn_error_tests;
#[cfg(test)]
mod spawn_reader_tests;
#[cfg(test)]
mod spawn_writer_tests;
#[cfg(test)]
mod wait_deadline_tests;
#[cfg(test)]
mod which_tests;
