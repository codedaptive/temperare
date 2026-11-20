//! `is_regular_file`: whether a path under a repository's top is a regular file reached through no symlink. It
//! touches the real filesystem, so its tests are the zone integration tests in integration/is_regular_file_tests.rs
//! and it has no unit-test sibling.

use std::path::{Component, Path};

/// True when `path` lies under `top` and is a regular file reached with no symlink below `top`. Each component below
/// `top`, the last included, is read with `symlink_metadata`, which never follows a link: a symlink anywhere is false
/// whatever it points at, and so is any read error. The last must be a regular file, so `top` itself, a directory or
/// anything else is false. A path outside `top`, or with a component that is not a plain name (such as `..`), is
/// false. `top` is canonical, so it holds no link of its own.
pub(crate) fn is_regular_file(top: &Path, path: &Path) -> bool {
    let Ok(below) = path.strip_prefix(top) else {
        return false;
    };
    let mut walked = top.to_path_buf();
    let mut last_is_file = false;
    for component in below.components() {
        let Component::Normal(name) = component else {
            return false;
        };
        walked.push(name);
        match std::fs::symlink_metadata(&walked) {
            Ok(meta) if !meta.file_type().is_symlink() => last_is_file = meta.file_type().is_file(),
            _ => return false,
        }
    }
    last_is_file
}
