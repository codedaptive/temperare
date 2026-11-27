//! `version_text`: the line `cctao --version` prints (S-32).

use super::version_after_name;

/// `cctao <version> (<commit>)`: the program's name, then what `version_after_name` gives for the version and commit.
pub fn version_text(version: &str, commit: &str) -> String {
    format!("cctao {}", version_after_name(version, commit))
}
