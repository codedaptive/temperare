//! `unfit_for_record`: whether a path cannot be written into an install record line as the path it is.

use std::path::Path;

/// Whether `path` is not valid UTF-8, or its text holds `\n`, `\r` or `\t`. A path that is not UTF-8 would be
/// written lossily, so its line would name another file, which uninstall could then remove. The record is one entry
/// per line and a link line splits at a tab, so a path holding a separator would read back as other entries.
/// `unrecordable_path` and `record_text` both decide by it.
pub(super) fn unfit_for_record(path: &Path) -> bool {
    path.to_str().is_none_or(|text| text.contains(['\n', '\r', '\t']))
}
