//! `is_workflow`: whether a file is a workflow actionlint is given.

use std::ffi::OsStr;
use std::path::Path;

/// True when `path` (relative to the rule file's folder) sits directly in `.github/workflows` and its extension is
/// exactly `yml` or `yaml`: the form of the files GitHub runs as workflows.
pub(crate) fn is_workflow(path: &Path) -> bool {
    path.parent() == Some(Path::new(".github/workflows"))
        && matches!(path.extension().and_then(OsStr::to_str), Some("yml" | "yaml"))
}
