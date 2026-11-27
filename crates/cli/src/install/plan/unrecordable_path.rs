//! `unrecordable_path`: the first path an install would place or record that the install record cannot hold.

use std::path::{Path, PathBuf};

use super::unfit_for_record::unfit_for_record;
use crate::InstallPlan;

/// The first of `record`, `plan.place` and each skill in order that `unfit_for_record` rejects: not valid UTF-8, or
/// its text holds a newline, a carriage return or a tab; `None` when no path is. A path not UTF-8 would be written
/// lossily, one with a newline would read back as other entries and one ending in a carriage return loses it on
/// read, so the record would name a file install did not create; a tab or a carriage return inside a path reads back
/// as itself, and both are refused to keep each line one plain path.
/// `run_install` refuses the install on `Some`, before any disk write.
pub(crate) fn unrecordable_path(plan: &InstallPlan, record: &Path) -> Option<PathBuf> {
    std::iter::once(record)
        .chain(plan.place.as_deref())
        .chain(plan.skills.iter().map(PathBuf::as_path))
        .find(|path| unfit_for_record(path))
        .map(Path::to_path_buf)
}
