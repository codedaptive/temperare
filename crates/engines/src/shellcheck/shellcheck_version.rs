//! `shellcheck_version`: the version in `shellcheck --version` output.

use procio::Output;

/// For a probe that exited 0: the text after `version:` on stdout's first line that starts with it (leading space
/// ignored), trimmed, as in shellcheck's `version: 0.11.0`. `None` for any other exit, or when there is no such line
/// or its text is empty. shellcheck's first line is its name, with no version, so `version_word` does not fit.
pub fn shellcheck_version(out: &Output) -> Option<String> {
    if out.code != Some(0) {
        return None;
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let version = stdout.lines().find_map(|line| line.trim_start().strip_prefix("version:"))?.trim();
    (!version.is_empty()).then(|| version.to_string())
}
