//! `engine_version`: one engine's version, read by its entry's probe.

use crate::entry::Entry;
use crate::host::Host;
use crate::program_for::program_for;
use crate::ran::ran;
use procio::Cmd;
use std::path::Path;

/// The version `entry`'s probe reads: the program `program_for` gives for `folder` (the one the run runs) with the
/// probe's args, run in `folder`, with the probe's `wrap` applied to the command when it has one, through `ran`
/// (never `check_run`: a probe is not a check run, so `host.raw_chars` is left untouched), its output read by the
/// probe's `read`. `None` when the entry has no probe, the probe could not be run, or `read` gives no version.
pub(crate) fn engine_version(entry: &Entry, folder: &Path, host: &Host) -> Option<String> {
    let probe = entry.version?;
    let cmd = Cmd {
        args: probe.args.iter().map(Into::into).collect(),
        cwd: Some(folder.to_path_buf()),
        ..Cmd::new(program_for(entry, Some(folder)))
    };
    let cmd = match probe.wrap {
        Some(wrap) => wrap(cmd),
        None => cmd,
    };
    ran(&cmd, host).ok().and_then(|out| (probe.read)(&out))
}
