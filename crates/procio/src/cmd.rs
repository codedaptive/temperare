//! `Cmd`: one program to run, described as data. Never a shell: `program` is run directly with `args` as given.

use std::ffi::OsString;
use std::path::PathBuf;
use std::time::Duration;

/// What `run` starts: the program, its arguments, where and with what environment, what it reads, how long to wait.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cmd {
    /// A program name looked up on the effective PATH (the `Cmd`'s own when `env` sets it, none when `env_remove`
    /// removes it, else the caller's; absolute entries only), or a path to the program.
    pub program: OsString,
    /// The arguments, each passed to the program as one argument, unquoted and unsplit.
    pub args: Vec<OsString>,
    /// The directory the program runs in; `None` runs it in the caller's current directory.
    pub cwd: Option<PathBuf>,
    /// Variables added to the inherited environment, in order.
    pub env: Vec<(OsString, OsString)>,
    /// Names removed from the inherited environment. Applied before `env`, so a name in both ends with `env`'s value.
    pub env_remove: Vec<OsString>,
    /// Bytes always written to the program's stdin through a pipe, which is then closed: empty means an immediate
    /// end of input. The caller's own stdin is never inherited, so a program that reads stdin cannot hang on it.
    pub stdin: Vec<u8>,
    /// How long all of `run` may take: the spawn, the program's exit, writing its stdin and reading both outputs.
    /// Past it the program is killed; on unix its process group too (what it started, unless that left the group),
    /// when the system's `kill` succeeds. `None` waits for as long as it runs and its pipes stay open; the caller
    /// chooses.
    pub timeout: Option<Duration>,
}
