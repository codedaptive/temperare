//! `clippy_cmd`: the `cargo clippy` command for one check and its allowed lints.

use crate::program::program;
use procio::Cmd;
use rules::Engine;
use std::ffi::OsString;
use std::path::Path;

/// `cargo clippy --all-targets --message-format=json --quiet -- -W <check> [-A <lint>]...` run in `cargo_root`.
/// `--all-targets` lints tests, examples and benches too: a rule's scope is its paths, not cargo's choice of
/// targets, and without it test code is never linted and reads as clean. `-W` turns the check on as a warning so it
/// is reported even when it is allowed by default; each `-A` comes after it because the later flag wins, so an
/// allowed lint stays off even when the check (`warnings`, say) covers it.
pub fn clippy_cmd(check: &str, allows: &[String], cargo_root: &Path) -> Cmd {
    let head = ["clippy", "--all-targets", "--message-format=json", "--quiet", "--", "-W", check];
    let tail = allows.iter().flat_map(|lint| ["-A", lint.as_str()]);
    Cmd {
        args: head.into_iter().chain(tail).map(OsString::from).collect(),
        cwd: Some(cargo_root.to_path_buf()),
        ..Cmd::new(program(Engine::Clippy))
    }
}
