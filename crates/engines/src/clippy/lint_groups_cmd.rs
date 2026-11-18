//! `lint_groups_cmd`: the command that lists the toolchain's lints and lint groups.

use procio::Cmd;
use std::ffi::OsString;
use std::path::Path;

/// `clippy-driver -W help` run in `cargo_root`. `clippy-driver` is a rustup proxy, as `cargo` is, so run in the
/// Cargo root it picks the same toolchain (and `rust-toolchain.toml`) as the `cargo clippy` that follows: the lints
/// and groups it lists are the ones that clippy run knows, those added by a newer toolchain included.
pub(crate) fn lint_groups_cmd(cargo_root: &Path) -> Cmd {
    Cmd {
        args: ["-W", "help"].map(OsString::from).into(),
        cwd: Some(cargo_root.to_path_buf()),
        ..Cmd::new("clippy-driver")
    }
}
