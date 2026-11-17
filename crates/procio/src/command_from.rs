//! `command_from`: the `std::process::Command` a `Cmd` describes, ready to spawn.

#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

use crate::Cmd;

/// The program run directly (no shell) with the args, cwd and environment changes of `cmd`, all three standard
/// streams piped. Removals go in before additions, so a name both removed and added keeps the added value.
/// On unix, a `cmd` with a timeout starts in a new process group of its own (group id = its pid), so `run` can kill
/// it with every process in that group once the timeout passes. Without a timeout it stays in the caller's group, so a
/// Ctrl-C at the caller's terminal still reaches it.
pub(crate) fn command_from(cmd: &Cmd) -> Command {
    let mut command = Command::new(&cmd.program);
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
