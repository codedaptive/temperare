//! `unrecordable_path`: the first path an install would place or record that the install record cannot hold.

use std::path::{Path, PathBuf};

use super::unfit_for_record::unfit_for_record;
use crate::InstallPlan;

/// The first of `record`, `plan.place`, the link and then its target, and each skill in order, that
/// `unfit_for_record` rejects: not valid UTF-8, or its text holds a newline, a carriage return or a tab; `None` when
/// no path is. Such a path would be written lossily or read back as another entry, so the record would name a file
/// install did not create: `run_install` refuses the install on `Some`, before any disk write.
pub(crate) fn unrecordable_path(plan: &InstallPlan, record: &Path) -> Option<PathBuf> {
    let link = plan.link.iter().flat_map(|(link, target)| [link.as_path(), target.as_path()]);
    std::iter::once(record)
        .chain(plan.place.as_deref())
        .chain(link)
        .chain(plan.skills.iter().map(PathBuf::as_path))
        .find(|path| unfit_for_record(path))
        .map(Path::to_path_buf)
}
