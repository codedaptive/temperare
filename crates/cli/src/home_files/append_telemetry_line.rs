//! `append_telemetry_line`: adds one line to a repository's telemetry file under HOME.
//! Tested in tests/world/telemetry_real.rs (a link refused, two worktrees appending to one file).

use std::path::Path;

use super::append_under_home;

/// Appends `line` and a newline, as one `write_all`, to `<home>/.local/state/temperare/telemetry/<root>.jsonl`, where
/// `root` is the id of HEAD's first-parent root commit, making missing folders. A symbolic link or the wrong kind of
/// entry on that path is refused and nothing is written; `append_under_home` has the checks. Does nothing when there
/// is no HOME; any failure is silent.
pub(crate) fn append_telemetry_line(home: Option<&Path>, root: &str, line: &str) {
    append_under_home(home, &[".local", "state", "temperare", "telemetry"], &format!("{root}.jsonl"), line);
}
