//! `version_text`: the line `temperare --version` prints (S-32).

use super::version_after_name;

/// `temperare <version> (<commit>)`: the long name, then what `version_after_name` gives for the version and commit.
pub fn version_text(version: &str, commit: &str) -> String {
    format!("temperare {}", version_after_name(version, commit))
}
