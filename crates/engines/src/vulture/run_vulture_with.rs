//! `run_vulture_with`: vulture's flow for one rule, leaving out what a given probe says git ignores, with Python's
//! warnings silenced.

use super::exclude_arg;
use crate::entry::{run_plain, Entry};
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::Cmd;
use rules::Rule;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// `exclude_arg` of what `ignored` gives for `folder` (nothing when it gives `None`); its Err is `Failed` with no exit
/// code, before anything runs. Else `run_plain` over a host whose `run` adds PYTHONWARNINGS=ignore to every command,
/// puts the exclude argument, when there is one, just before the last argument (the `.` vulture scans), and runs it
/// with `host.run`. Without PYTHONWARNINGS=ignore, a SyntaxWarning (an invalid escape in a string, say) lands on stderr
/// for a file vulture reads fine, and `parse_vulture` takes any stderr as a failure; and a caller's
/// PYTHONWARNINGS=error would turn such a warning into a syntax error. `on_path`, `raw_chars` and `base` are `host`'s:
/// the output is not altered, so `check_run` counts it as vulture printed it.
pub(crate) fn run_vulture_with(
    ignored: &dyn Fn(&Path) -> Option<Vec<(PathBuf, bool)>>,
    entry: &Entry,
    rule: &Rule,
    folder: &Path,
    host: &Host,
) -> Result<Vec<Hit>, Stop> {
    let exclude = exclude_arg(&ignored(folder).unwrap_or_default()).map_err(|stderr| Stop::Failed { code: None, stderr })?;
    let run = |cmd: &Cmd| {
        let mut cmd = cmd.clone();
        cmd.env.push((OsString::from("PYTHONWARNINGS"), OsString::from("ignore")));
        if let Some(exclude) = &exclude {
            cmd.args.insert(cmd.args.len().saturating_sub(1), exclude.clone());
        }
        (host.run)(&cmd)
    };
    run_plain(entry, rule, folder, &Host { run: &run, on_path: host.on_path, raw_chars: host.raw_chars, base: host.base })
}
