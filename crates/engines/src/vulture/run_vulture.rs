//! `run_vulture`: vulture's flow for one rule, the standard flow with Python's warnings silenced.

use crate::entry::{run_plain, Entry};
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::Cmd;
use rules::Rule;
use std::ffi::OsString;
use std::path::Path;

/// `run_plain` over a host whose `run` adds PYTHONWARNINGS=ignore to every command and runs it with `host.run`.
/// Without it, a SyntaxWarning (an invalid escape in a string, say) lands on stderr for a file vulture reads fine,
/// and `parse_vulture` takes any stderr as a failure; and a caller's PYTHONWARNINGS=error would turn such a warning
/// into a syntax error. `on_path`, `raw_chars` and `base` are
/// `host`'s: the output is not altered, so `check_run` counts it as vulture printed it.
pub(crate) fn run_vulture(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    let run = |cmd: &Cmd| {
        let mut cmd = cmd.clone();
        cmd.env.push((OsString::from("PYTHONWARNINGS"), OsString::from("ignore")));
        (host.run)(&cmd)
    };
    run_plain(entry, rule, folder, &Host { run: &run, on_path: host.on_path, raw_chars: host.raw_chars, base: host.base })
}
