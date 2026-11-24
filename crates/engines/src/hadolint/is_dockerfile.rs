//! `is_dockerfile`: whether a file is a Dockerfile hadolint is given.

use std::path::Path;

/// True when `path`'s file name is exactly `Dockerfile` or `Containerfile`, or ends with `.Dockerfile` with something
/// before it (`web.Dockerfile`). Case-sensitive, as the names are written: `dockerfile` and `Dockerfile.dev` are not
/// Dockerfiles here. A bare `.Dockerfile` is a hidden file with no name, not a variant of one.
pub(crate) fn is_dockerfile(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    matches!(name, "Dockerfile" | "Containerfile") || name.strip_suffix(".Dockerfile").is_some_and(|stem| !stem.is_empty())
}
