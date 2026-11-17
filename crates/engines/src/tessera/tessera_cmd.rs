//! `tessera_cmd`: the `ts slop --json` command for a folder.

use crate::program::program;
use procio::Cmd;
use rules::Engine;
use std::ffi::OsString;
use std::path::Path;

/// `ts slop --json` run in `folder`.
pub fn tessera_cmd(folder: &Path) -> Cmd {
    Cmd {
        args: ["slop", "--json"].map(OsString::from).into(),
        cwd: Some(folder.to_path_buf()),
        ..Cmd::new(program(Engine::Tessera))
    }
}
