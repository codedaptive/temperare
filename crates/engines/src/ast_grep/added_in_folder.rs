//! `added_in_folder`: the files under a rule file's folder that the change adds lines to, and the first line added.

use std::collections::BTreeMap;
use std::path::Path;

use crate::change_of::change_of;
use crate::types::Stop;
use rules::RelPath;

/// For each file under `folder` (canonical) that the working tree adds lines to against `base` and `keep` says yes
/// to, its first added line (1-based), keyed relative to `folder`: the lowest range start of the file's added lines
/// as `change_of` reads them, so a file with no added range is absent. So a tracked file with only removed lines is
/// absent; an untracked file git does not ignore gives 1, but an empty one (no lines) is absent, and so is a binary
/// one; an untracked symlink, a path under a symlinked folder or any other untracked path that is not a regular file
/// is skipped, not read; and a path `keep` says no to is dropped before it is read. A git error is `git_stop`'s
/// `Stop`, a `base` that names no revision is `Failed`, and a kept regular untracked file that cannot be read is
/// `Failed` naming it (`change_of`).
pub(crate) fn added_in_folder(
    folder: &Path,
    base: &str,
    keep: &dyn Fn(&RelPath) -> bool,
) -> Result<BTreeMap<RelPath, u32>, Stop> {
    Ok(change_of(folder, base, keep)?
        .into_iter()
        .filter_map(|(path, ranges)| Some((path, ranges.iter().map(|range| range.start).min()?)))
        .collect())
}
