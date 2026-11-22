//! `ignore_listed`: whether a `.gitleaksignore` lists anything.

/// True when some line of `text`, trimmed, is neither empty nor a `#` comment: an entry gitleaks would ignore.
pub(crate) fn ignore_listed(text: &str) -> bool {
    text.lines().map(str::trim).any(|line| !line.is_empty() && !line.starts_with('#'))
}
