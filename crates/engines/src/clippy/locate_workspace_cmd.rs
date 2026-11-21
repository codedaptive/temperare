//! `locate_workspace_cmd`: the command that names the Cargo workspace a crate belongs to.

use super::CLIPPY;
use procio::Cmd;
use std::ffi::OsString;
use std::path::Path;

/// `cargo locate-project --workspace --message-format plain` run in `cargo_root`, `cargo` being the program
/// `CLIPPY`'s check run runs: it prints the absolute path of the workspace's root `Cargo.toml`, the folder cargo prints
/// clippy's span paths relative to.
pub fn locate_workspace_cmd(cargo_root: &Path) -> Cmd {
    Cmd {
        args: ["locate-project", "--workspace", "--message-format", "plain"].map(OsString::from).into(),
        cwd: Some(cargo_root.to_path_buf()),
        ..Cmd::new(CLIPPY.program)
    }
}
