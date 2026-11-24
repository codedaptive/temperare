//! `is_markdown_source`: whether a file is a Markdown file markdownlint-cli2 is given.

use std::ffi::OsStr;
use std::path::Path;

/// True when `path`'s extension is exactly `md` or `markdown`, the files markdownlint-cli2 maps `.` to
/// (`*.{md,markdown}`, its own `--help`).
pub(crate) fn is_markdown_source(path: &Path) -> bool {
    matches!(path.extension().and_then(OsStr::to_str), Some("md" | "markdown"))
}
