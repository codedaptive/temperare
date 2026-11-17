//! `clippy_cmd`: the `cargo clippy` command for one lint.

use crate::program::program;
use procio::Cmd;
use rules::Engine;
use std::ffi::OsString;
use std::path::Path;

/// `cargo clippy --message-format=json --quiet -- -W <check>` run in `cargo_root`. `-W` turns the lint on as a
/// warning so it is reported even when it is allowed by default.
pub fn clippy_cmd(check: &str, cargo_root: &Path) -> Cmd {
    Cmd {
        args: ["clippy", "--message-format=json", "--quiet", "--", "-W", check].map(OsString::from).into(),
        cwd: Some(cargo_root.to_path_buf()),
        ..Cmd::new(program(Engine::Clippy))
    }
}
