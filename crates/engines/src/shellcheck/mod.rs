//! The shellcheck adapter: the reader of `shellcheck -f json1` output, the command, the version reader, the check
//! refusal, the listing of the shell scripts in a repository (tracked, and untracked but not ignored; by extension,
//! or by a shell shebang read from an extensionless file's first line; never a symlink), temperare's list of shellcheck codes (`shellcheck_codes.txt`, with its
//! `CodeList` reader and why) and the run of one rule. No entry runs them yet, because `rules::Engine` has no
//! ShellCheck engine, so only their tests call them; `run_shellcheck` is not re-exported until an entry names it.

mod code_list;
mod git_stop;
mod is_regular_file;
mod is_shell_shebang;
mod not_in_list_why;
mod parse_shellcheck;
mod read_code_list;
mod read_head;
mod run_shellcheck;
mod run_shellcheck_with;
mod shell_files;
mod shell_files_with;
mod shellcheck_cmd;
mod shellcheck_refusal;
mod shellcheck_version;

#[cfg(test)]
mod git_stop_tests;
#[cfg(test)]
mod integration;
#[cfg(test)]
mod is_shell_shebang_tests;
#[cfg(test)]
mod not_in_list_why_tests;
#[cfg(test)]
mod parse_shellcheck_tests;
#[cfg(test)]
mod read_code_list_tests;
#[cfg(test)]
mod run_shellcheck_tests;
#[cfg(test)]
mod run_shellcheck_with_tests;
#[cfg(test)]
mod shell_files_with_tests;
#[cfg(test)]
mod shellcheck_cmd_tests;
#[cfg(test)]
mod shellcheck_refusal_tests;
#[cfg(test)]
mod shellcheck_version_tests;

pub(crate) use code_list::CodeList;
pub(crate) use git_stop::git_stop;
pub(crate) use is_regular_file::is_regular_file;
pub(crate) use is_shell_shebang::is_shell_shebang;
pub(crate) use not_in_list_why::not_in_list_why;
pub use parse_shellcheck::parse_shellcheck;
pub(crate) use read_code_list::read_code_list;
pub(crate) use read_head::read_head;
pub(crate) use run_shellcheck_with::run_shellcheck_with;
pub(crate) use shell_files::shell_files;
pub(crate) use shell_files_with::shell_files_with;
pub use shellcheck_cmd::shellcheck_cmd;
pub use shellcheck_refusal::shellcheck_refusal;
pub use shellcheck_version::shellcheck_version;
