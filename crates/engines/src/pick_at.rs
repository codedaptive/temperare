//! `pick_at`: where in a command the argument carrying a rule's check sits.

use crate::entry::{Entry, Pick};
use procio::Cmd;

/// The index in `cmd.args` of the argument that carries `check`, as `entry_cmd` placed it: for `Pick::Arg(flag)` the
/// first argument equal to `{flag}{check}`; for `Pick::File(option)` the argument right after the first one equal to
/// `option` (the check file's path, so `check` is not compared). `None` for `Pick::None`, and when `cmd` holds no
/// such argument: a command that is not the rule's check run, such as clippy's `clippy-driver -W help`.
pub(crate) fn pick_at(entry: &Entry, check: &str, cmd: &Cmd) -> Option<usize> {
    match entry.pick {
        Pick::Arg(flag) => {
            let picked = format!("{flag}{check}");
            cmd.args.iter().position(|arg| arg == picked.as_str())
        }
        Pick::File(option) => {
            cmd.args.iter().position(|arg| arg == option).map(|at| at + 1).filter(|after| *after < cmd.args.len())
        }
        Pick::None => None,
    }
}
