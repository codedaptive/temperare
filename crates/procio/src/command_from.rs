//! `command_from`: the `std::process::Command` a `Cmd` describes, ready to spawn.

use std::ffi::OsStr;
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

use crate::Cmd;

/// `program` run directly (no shell) with the args, cwd and environment changes of `cmd`, all three standard
/// streams piped. `program` is spawned as given: `run` passes a bare name only as the full path it found, never
/// unresolved, so the OS does not search for it. On unix the child's argv[0] is still `cmd.program`, the name the
/// caller gave; elsewhere it is the program as spawned: the full path `run` found for a bare name, the path as given
/// for one holding a separator.
/// Removals go in before additions, so a name both removed and added keeps the added value.
/// On unix, a `cmd` with a timeout starts in a new process group of its own (group id = its pid), so `run` can kill
/// it with every process in that group once the timeout passes. Without a timeout it stays in the caller's group, so a
/// Ctrl-C at the caller's terminal still reaches it.
pub(crate) fn command_from(cmd: &Cmd, program: &OsStr) -> Command {
    let mut command = Command::new(program);
    #[cfg(unix)]
    command.arg0(&cmd.program);
    command.args(&cmd.args);
    if let Some(cwd) = &cmd.cwd {
        command.current_dir(cwd);
    }
    for name in &cmd.env_remove {
        command.env_remove(name);
    }
    command.envs(cmd.env.iter().map(|(name, value)| (name, value)));
    command.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());
    #[cfg(unix)]
    if cmd.timeout.is_some() {
        command.process_group(0);
    }
    command
}
