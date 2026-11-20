//! The shellcheck module's zone integration tests use the real filesystem and run in the integration group with
//! `--lib integration::`.

#[cfg(unix)]
mod read_head_tests;
