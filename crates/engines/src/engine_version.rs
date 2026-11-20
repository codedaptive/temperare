//! `engine_version`: one engine's version, read by its entry's probe.

use crate::entry::Entry;
use crate::host::Host;
use crate::ran::ran;
use procio::Cmd;
use std::path::Path;

/// The version `entry`'s probe reads: `entry.program` with the probe's args, run in `folder` through `ran` (never
/// `check_run`: a probe is not a check run, so `host.raw_chars` is left untouched), its output read by the probe's
/// `read`. `None` when the entry has no probe, the probe could not be run, or `read` gives no version.
pub(crate) fn engine_version(entry: &Entry, folder: &Path, host: &Host) -> Option<String> {
    let probe = entry.version?;
    let cmd = Cmd {
        args: probe.args.iter().map(Into::into).collect(),
        cwd: Some(folder.to_path_buf()),
        ..Cmd::new(entry.program)
    };
    ran(&cmd, host).ok().and_then(|out| (probe.read)(&out))
}
