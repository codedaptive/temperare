//! `run`: start a program, feed its stdin, capture both outputs, wait for its exit.
//!
//! No sibling unit tests: any test of `run` starts a process, which a unit test may not, so the runner is
//! proved by the integration tests in `tests/run.rs`. Its parts (`command_from`, `effective_path`,
//! `has_path_separator`, `kill_group_cmd`, `spawn_error`, `spawn_reader`, `spawn_writer`, `wait_deadline`, `which`)
//! each have their own unit tests.

use std::env;
use std::io;
#[cfg(unix)]
use std::num::NonZeroU32;
use std::process::{Child, ExitStatus};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crate::command_from::command_from;
use crate::effective_path::effective_path;
use crate::has_path_separator::has_path_separator;
#[cfg(unix)]
use crate::kill_group_cmd::kill_group_cmd;
use crate::spawn_error::spawn_error;
use crate::spawn_reader::spawn_reader;
use crate::spawn_writer::spawn_writer;
use crate::wait_deadline::wait_deadline;
use crate::which::which_in;
use crate::{Cmd, Output, ProcError};

/// Runs `cmd` to its end. Any exit code is `Ok`: a non-zero exit is the program's answer, not an error.
/// The stdin writer and both output readers start before the wait, so no pipe can fill and block the child.
/// Without a timeout, `run` waits for the child's exit, then for both readers and the stdin writer, which end when
/// every holder of the pipes (a grandchild too) has closed them. With one, the timeout bounds all of that, counted
/// from before the spawn: a grandchild holding a pipe open cannot keep `run` past it.
/// A bare program name is resolved once on the `Cmd`'s effective PATH (absolute entries only) and that path is
/// spawned, so the OS does not search PATH for it. On unix a name not found there is `NotFound` without spawning; off
/// unix it is spawned as given, since `which_in` does not try PATHEXT extensions such as `.exe`.
pub fn run(cmd: &Cmd) -> Result<Output, ProcError> {
    let start = Instant::now();
    let found = which_in(&cmd.program, effective_path(cmd, env::var_os("PATH").as_deref()).as_deref());
    let program = match (has_path_separator(&cmd.program), &found) {
        (false, Some(path)) => path.as_os_str(),
        (false, None) if cfg!(unix) => return Err(ProcError::NotFound { program: cmd.program.clone() }),
        _ => cmd.program.as_os_str(),
    };
    let mut child = command_from(cmd, program)
        .spawn()
        .map_err(|io| spawn_error(&cmd.program, io, found.is_some()))?;
    let (Some(stdin), Some(stdout), Some(stderr)) = (child.stdin.take(), child.stdout.take(), child.stderr.take())
    else {
        stop(&mut child, cmd);
        return Err(spawn_failed(cmd, io::Error::other("a standard stream was not piped")));
    };
    let writer = spawn_writer(stdin, cmd.stdin.clone());
    let out = spawn_reader(stdout);
    let err = spawn_reader(stderr);
    // On an error the three handles drop unjoined and the output read so far is dropped with them. The threads end
    // once the pipes close: on unix with a timeout, the child's group has just been killed, which closes them unless
    // a process left the group or the kill failed; otherwise when the last holder (a grandchild too) closes them.
    let status = match cmd.timeout {
        None => wait(&mut child, cmd)?,
        Some(after) => {
            let pipes_done = || out.is_finished() && err.is_finished() && writer.is_finished();
            wait_bounded(&mut child, cmd, after, after.saturating_sub(start.elapsed()), pipes_done)?
        }
    };
    let stdout = joined(out).map_err(|io| spawn_failed(cmd, io))?;
    let stderr = joined(err).map_err(|io| spawn_failed(cmd, io))?;
    joined(writer).map_err(|io| spawn_failed(cmd, io))?;
    Ok(Output { code: status.code(), stdout, stderr })
}

/// The child's exit status, waiting for as long as it runs. On a wait error the child is killed and reaped before
/// the error is returned.
fn wait(child: &mut Child, cmd: &Cmd) -> Result<ExitStatus, ProcError> {
    child.wait().map_err(|io| {
        stop(child, cmd);
        spawn_failed(cmd, io)
    })
}

/// The child's exit status once it has exited and `pipes_done` says the pipe threads have ended, waiting at most
/// `left` for both. The status is kept from the first poll that gives it, so later polls only wait on the pipes.
/// On a timeout (reported with the caller's `after`) or a poll error the child is stopped before the error is
/// returned.
fn wait_bounded(
    child: &mut Child,
    cmd: &Cmd,
    after: Duration,
    left: Duration,
    pipes_done: impl Fn() -> bool,
) -> Result<ExitStatus, ProcError> {
    let mut status = None;
    let poll = || -> io::Result<Option<ExitStatus>> {
        if status.is_none() {
            status = child.try_wait()?;
        }
        Ok(status.filter(|_| pipes_done()))
    };
    let error = match wait_deadline(poll, left) {
        Ok(Some(status)) => return Ok(status),
        Ok(None) => ProcError::TimedOut { program: cmd.program.clone(), after },
        Err(io) => spawn_failed(cmd, io),
    };
    stop(child, cmd);
    Err(error)
}

/// Kills and reaps the child; with a timeout set, its process group first (the group id is the child's pid, and
/// stays valid while any member of the group lives). Every result is dropped: this runs only on a path that is
/// returning an error of its own, and the child may have exited already.
fn stop(child: &mut Child, cmd: &Cmd) {
    if cmd.timeout.is_some() {
        kill_group(child);
    }
    let _ = child.kill();
    let _ = child.wait();
}

/// Kills every process in the child's group: the child started one of its own because the timeout is set.
#[cfg(unix)]
fn kill_group(child: &Child) {
    if let Some(pgid) = NonZeroU32::new(child.id()) {
        let _ = run(&kill_group_cmd(pgid));
    }
}

/// Off unix the child has no group of its own, so only the child is killed (by `stop`). The timeout still bounds
/// `run`, as the pipe threads are dropped unjoined, but a grandchild the child started may outlive it.
#[cfg(not(unix))]
fn kill_group(_child: &Child) {}

/// A pipe thread's result; a thread that panicked becomes an io error.
fn joined<T>(handle: JoinHandle<io::Result<T>>) -> io::Result<T> {
    handle.join().unwrap_or_else(|_| Err(io::Error::other("a pipe thread panicked")))
}

fn spawn_failed(cmd: &Cmd, io: io::Error) -> ProcError {
    ProcError::Spawn { program: cmd.program.clone(), io }
}
