//! The clang-tidy adapter: its entry (`CLANG_TIDY`) and the flow it runs for one rule, the refusal of a check that is
//! not one check name, clang-tidy's no-checks and no-compile-database words, the reader of its text output, its
//! command, and the listing of the C-family sources in a repository (tracked, and untracked but not ignored; never a
//! symlink).

mod c_files;
mod clang_tidy_cmd;
mod clang_tidy_entry;
mod clang_tidy_refusal;
mod is_c_source;
mod no_compile_database;
mod parse_clang_tidy;
mod run_clang_tidy;
mod unknown_check;

#[cfg(test)]
mod clang_tidy_cmd_tests;
#[cfg(test)]
mod clang_tidy_entry_tests;
#[cfg(test)]
mod clang_tidy_refusal_tests;
#[cfg(test)]
mod is_c_source_tests;
#[cfg(test)]
mod no_compile_database_tests;
#[cfg(test)]
mod parse_clang_tidy_tests;
#[cfg(test)]
mod unknown_check_tests;

pub(crate) use c_files::c_files;
pub use clang_tidy_cmd::clang_tidy_cmd;
pub use clang_tidy_entry::CLANG_TIDY;
pub(crate) use clang_tidy_refusal::clang_tidy_refusal;
pub(crate) use is_c_source::is_c_source;
pub(crate) use no_compile_database::no_compile_database;
pub use parse_clang_tidy::parse_clang_tidy;
pub(crate) use run_clang_tidy::run_clang_tidy;
pub(crate) use unknown_check::unknown_check;
