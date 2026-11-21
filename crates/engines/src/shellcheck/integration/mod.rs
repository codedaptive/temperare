//! The shellcheck module's zone integration tests use the real filesystem and run in the integration group with
//! `--lib integration::`.

#[cfg(unix)]
mod is_regular_file_tests;
#[cfg(unix)]
mod read_head_tests;
#[cfg(unix)]
mod shell_files_tests;
#[cfg(unix)]
mod run_shellcheck_with_tests;
