//! `package_root`: the Swift package a Periphery rule runs on. It reads the disk, so its test is the zone integration
//! test tests/periphery.rs and it has no unit-test sibling.

use std::path::{Path, PathBuf};

/// The nearest ancestor of `folder`, `folder` itself included, that is at or below the repository top `top` and holds
/// a `Package.swift` file; None when no such ancestor does. It never looks above `top`, so a package outside the
/// checked repository is never built. Periphery builds and scans that whole package, and only results under `folder`
/// are kept.
pub(crate) fn package_root(folder: &Path, top: &Path) -> Option<PathBuf> {
    folder
        .ancestors()
        .take_while(|dir| dir.starts_with(top))
        .find(|dir| dir.join("Package.swift").is_file())
        .map(Path::to_path_buf)
}
