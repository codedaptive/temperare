//! `kill_group_cmd`: the `Cmd` that kills a whole process group.

use std::num::NonZeroU32;

use crate::Cmd;

/// `kill -KILL -- -<pgid>`: SIGKILL to every process in group `pgid`. std has no killpg, so `run` stops the group
/// of a child run with a timeout by running the system's `kill` through procio itself. The `--` keeps the negative
/// group id from being read as an option. Non-zero because `kill -- -0` would signal the caller's own group. No
/// timeout, so running it never starts a group of its own and never stops a group in turn.
pub(crate) fn kill_group_cmd(pgid: NonZeroU32) -> Cmd {
    Cmd { args: vec!["-KILL".into(), "--".into(), format!("-{pgid}").into()], ..Cmd::new("kill") }
}
