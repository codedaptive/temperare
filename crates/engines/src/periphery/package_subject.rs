//! `package_subject`: how a Periphery why names a package the manifest walk read.

use std::path::Path;

/// "the package" for `root`, else "the local package <its path relative to `top`>" (its full path when it is not
/// under `top`). A package at `top` itself is "the local package .": "." names the repository top as a path relative
/// to itself, as every other subject is named, where the stripped path would be empty.
pub(crate) fn package_subject(package: &Path, root: &Path, top: &Path) -> String {
    if package == root {
        return "the package".to_string();
    }
    let relative = package.strip_prefix(top).unwrap_or(package);
    if relative.as_os_str().is_empty() {
        return "the local package .".to_string();
    }
    format!("the local package {}", relative.display())
}
