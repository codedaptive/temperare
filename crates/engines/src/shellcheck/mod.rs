//! The shellcheck adapter: its entry (`SHELLCHECK`) and the flow it runs for one rule, the reader of `shellcheck -f
//! json1` output, the command, the version reader, the check refusal, the listing of the shell scripts in a
//! repository (tracked, and untracked but not ignored; by extension, or by a shell shebang read from an extensionless
//! file's first line; never a symlink) and cctao's list of shellcheck codes (`shellcheck_codes.txt`; its
//! `CodeList` reader is the crate's internal `read_code_list`).

mod is_shell_shebang;
mod parse_shellcheck;
mod read_head;
mod run_shellcheck;
mod run_shellcheck_with;
mod shell_files;
mod shell_files_with;
mod shellcheck_cmd;
mod shellcheck_entry;
mod shellcheck_refusal;
mod shellcheck_version;

#[cfg(test)]
mod integration;
#[cfg(test)]
mod is_shell_shebang_tests;
#[cfg(test)]
mod parse_shellcheck_tests;
#[cfg(test)]
mod run_shellcheck_tests;
#[cfg(test)]
mod run_shellcheck_with_tests;
#[cfg(test)]
mod shell_files_with_tests;
#[cfg(test)]
mod shellcheck_cmd_tests;
#[cfg(test)]
mod shellcheck_entry_tests;
#[cfg(test)]
mod shellcheck_refusal_tests;
#[cfg(test)]
mod shellcheck_version_tests;

pub(crate) use is_shell_shebang::is_shell_shebang;
pub use parse_shellcheck::parse_shellcheck;
pub(crate) use read_head::read_head;
pub(crate) use run_shellcheck::run_shellcheck;
pub(crate) use run_shellcheck_with::run_shellcheck_with;
pub(crate) use shell_files::shell_files;
pub(crate) use shell_files_with::shell_files_with;
pub use shellcheck_cmd::shellcheck_cmd;
pub use shellcheck_entry::SHELLCHECK;
pub use shellcheck_refusal::shellcheck_refusal;
pub use shellcheck_version::shellcheck_version;
