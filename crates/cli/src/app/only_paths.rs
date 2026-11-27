//! `only_paths`: the paths given to `cctao check`, as paths relative to the rule file's folder.

use crate::UsageError;
use rules::RelPath;
use std::path::{Component, Path, PathBuf};

/// Each of `paths` (relative ones taken from `cwd`) as a `RelPath` under `folder`, in the order given. Paths are
/// normalised lexically (`.` dropped, `..` pops a component) and nothing is read from disk. Empty `paths`, or any
/// path that is the folder itself, asks for the whole tree: `Ok(vec![])`. A path outside the folder, or one that is
/// not UTF-8, is a usage error, even beside the folder itself.
pub(crate) fn only_paths(paths: &[PathBuf], cwd: &Path, folder: &Path) -> Result<Vec<RelPath>, UsageError> {
    let top = normalised(folder);
    let texts = paths
        .iter()
        .map(|path| relative_text(&normalised(&cwd.join(path)), &top).ok_or_else(|| not_under(path, folder)))
        .collect::<Result<Vec<String>, UsageError>>()?;
    if texts.iter().any(String::is_empty) {
        return Ok(Vec::new());
    }
    Ok(texts.iter().map(|text| RelPath::new(text)).collect())
}

/// `path` with `.` dropped and each `..` popping the component before it.
fn normalised(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

/// `path` relative to `top`, components joined with '/'; empty for `top` itself; None when not under `top` or not
/// UTF-8.
fn relative_text(path: &Path, top: &Path) -> Option<String> {
    let rest = path.strip_prefix(top).ok()?;
    let parts: Option<Vec<&str>> = rest.components().map(|c| c.as_os_str().to_str()).collect();
    Some(parts?.join("/"))
}

/// The usage error for `path`, which cannot be made a path under `folder`.
fn not_under(path: &Path, folder: &Path) -> UsageError {
    UsageError {
        what: format!("{} is not under the rule file's folder {}", path.display(), folder.display()),
        fix: format!("Pass paths inside {}, or pass --rules with the rule file that covers them.", folder.display()),
    }
}
