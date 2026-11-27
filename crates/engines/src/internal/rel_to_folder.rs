//! `rel_to_folder`: an engine's printed path, made relative to the rule file's folder.

use crate::strip_verbatim_disk::strip_verbatim_disk;
use rules::RelPath;
use std::path::{Component, Path, PathBuf};

/// `raw`, a path as the engine printed it (absolute, or relative to `cwd`, the engine's working directory), relative
/// to `folder`; None when it is not under `folder`. `.` is dropped and `..` is folded lexically before the check, so a
/// path whose `..` climbs out of `folder` as text is None. Pure: it reads no file and resolves no link, so a `..` after a
/// symlink is folded as text, not as the kernel would resolve it; callers pass canonical `cwd` and `folder`. On Windows
/// a verbatim disk prefix (`\\?\X:\`) is dropped from `folder` and from the joined path first, so a path printed as
/// `C:\...` is placed under a canonical `\\?\C:\...` folder; the drive letter is then compared as std's Windows `Path`
/// compares it, which matches `c:` with `C:` (a cfg(windows) row in rel_to_folder_tests.rs pins it).
pub(crate) fn rel_to_folder(cwd: &Path, folder: &Path, raw: &str) -> Option<RelPath> {
    let full = fold(&one_form(&cwd.join(raw)));
    let inside = full.strip_prefix(one_form(folder)).ok()?;
    Some(RelPath::new(&inside.to_string_lossy()))
}

/// `path` in plain disk form on Windows; as given elsewhere, where `\\?\` is ordinary text.
fn one_form(path: &Path) -> PathBuf {
    if cfg!(windows) { strip_verbatim_disk(path) } else { path.into() }
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
