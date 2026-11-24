//! `pubspec_root`: the Dart package a dart rule runs in. It reads the disk, so its test is the zone integration test
//! tests/dart.rs and it has no unit-test sibling.

use std::path::{Path, PathBuf};

/// The nearest ancestor of `folder`, `folder` itself included, that is at or below the repository top `top` and holds
/// a `pubspec.yaml` file; None when no such ancestor does. It never looks above `top`, so a package outside the
/// checked repository is never the one the rule runs in.
pub(crate) fn pubspec_root(folder: &Path, top: &Path) -> Option<PathBuf> {
    folder
        .ancestors()
        .take_while(|dir| dir.starts_with(top))
        .find(|dir| dir.join("pubspec.yaml").is_file())
        .map(Path::to_path_buf)
}
