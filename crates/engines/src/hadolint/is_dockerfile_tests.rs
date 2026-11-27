//! Unit test for is_dockerfile.rs: `Dockerfile`, `Containerfile` and `<name>.Dockerfile` are Dockerfiles, in any folder;
//! another case, a suffix after the name, or a bare `.Dockerfile` is not.

use super::is_dockerfile;
use std::path::Path;

#[test]
fn is_dockerfile_cases() {
    let rows = [
        ("Dockerfile", "Dockerfile", true),
        ("in a folder", "sub/Dockerfile", true),
        ("a named variant", "web.Dockerfile", true),
        ("a named variant in a folder", "deploy/api.Dockerfile", true),
        ("Containerfile", "Containerfile", true),
        ("no name before the suffix", ".Dockerfile", false),
        ("a suffix after the name", "Dockerfile.dev", false),
        ("lower case", "dockerfile", false),
        ("lower case suffix", "web.dockerfile", false),
        ("a folder named Dockerfile", "Dockerfile/a.txt", false),
        ("another file", "Makefile", false),
    ];
    for (label, path, expected) in rows {
        assert_eq!(is_dockerfile(Path::new(path)), expected, "{label}");
    }
}
