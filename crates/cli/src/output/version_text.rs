//! `version_text`: the line `temperare --version` prints.

/// `temperare <version> (<commit>)`: the workspace version and the commit the binary was built from (`unknown` when
/// the build could not read it).
pub fn version_text(version: &str, commit: &str) -> String {
    format!("temperare {version} ({commit})")
}
