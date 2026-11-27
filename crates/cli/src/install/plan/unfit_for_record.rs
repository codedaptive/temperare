//! `unfit_for_record`: whether a path cannot be written into an install record line as the path it is.

use std::path::Path;

/// Whether `path` is not valid UTF-8, or its text holds `\n`, `\r` or `\t`. A path that is not UTF-8 would be
/// written lossily, so its line would name another file, which uninstall could then remove. The record is one entry
/// per line, so a path holding `\n` would read back as other entries, and a trailing `\r` is dropped on read. A
/// `\t` or a `\r` inside the path would read back intact; they are refused to keep each record line one plain path.
/// `unrecordable_path` and `record_text` both decide by it.
pub(super) fn unfit_for_record(path: &Path) -> bool {
    path.to_str().is_none_or(|text| text.contains(['\n', '\r', '\t']))
}
