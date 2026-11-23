//! `rel_to_folder`: an engine's printed path, made relative to the rule file's folder.

use rules::RelPath;
use std::path::{Component, Path, PathBuf};

/// `raw`, a path as the engine printed it (absolute, or relative to `cwd`, the engine's working directory), relative
/// to `folder`; None when it is not under `folder`. `.` is dropped and `..` is folded lexically before the check, so a
/// path whose `..` climbs out of `folder` as text is None. Pure: it reads no file and resolves no link, so a `..` after a
/// symlink is folded as text, not as the kernel would resolve it; callers pass canonical `cwd` and `folder`, the same
/// form the engine prints.
pub(crate) fn rel_to_folder(cwd: &Path, folder: &Path, raw: &str) -> Option<RelPath> {
    let full = fold(&cwd.join(raw));
    let inside = full.strip_prefix(folder).ok()?;
    Some(RelPath::new(&inside.to_string_lossy()))
}

/// `path` with `.` dropped and each `..` popping the component before it. A `..` at the root stays at the root, as
/// `/..` is `/`; one with nothing to pop in a relative path is kept.
fn fold(path: &Path) -> PathBuf {
    let mut parts: Vec<Component> = Vec::new();
    for part in path.components() {
        match (part, parts.last()) {
            (Component::CurDir, _) | (Component::ParentDir, Some(Component::RootDir)) => {}
            (Component::ParentDir, Some(Component::Normal(_))) => {
                parts.pop();
            }
            _ => parts.push(part),
        }
    }
    parts.into_iter().collect()
}
