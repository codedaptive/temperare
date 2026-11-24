//! `eslint_version`: the version `eslint --version` prints.

use procio::Output;

/// For a probe that exited 0: stdout's first non-empty line, trimmed, with a leading `v` stripped (ESLint 10.15.0
/// prints `v10.15.0`), when it then starts with an ASCII digit. `None` for any other exit or line.
pub(crate) fn eslint_version(out: &Output) -> Option<String> {
    if out.code != Some(0) {
        return None;
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let line = stdout.lines().map(str::trim).find(|line| !line.is_empty())?;
    let version = line.strip_prefix('v').unwrap_or(line);
    version.starts_with(|c: char| c.is_ascii_digit()).then(|| version.to_string())
}
