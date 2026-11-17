//! `rel_to_folder`: an engine's printed path, made relative to the rule file's folder.

use rules::RelPath;
use std::path::{Component, Path, PathBuf};

/// `raw`, a path as the engine printed it (absolute, or relative to `cwd`, the engine's working directory), relative
/// to `folder`; None when it is not under `folder`. Pure: it reads no file and resolves no link, so callers pass
/// canonical `cwd` and `folder`, the same form the engine prints.
pub(crate) fn rel_to_folder(cwd: &Path, folder: &Path, raw: &str) -> Option<RelPath> {
    let full: PathBuf = cwd.join(raw).components().filter(|part| *part != Component::CurDir).collect();
    let inside = full.strip_prefix(folder).ok()?;
    Some(RelPath::new(&inside.to_string_lossy()))
}
