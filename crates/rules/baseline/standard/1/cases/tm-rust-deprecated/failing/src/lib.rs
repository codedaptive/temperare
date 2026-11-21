//! Input cleaning.

/// `line` without the spaces it starts with.
pub fn clean(line: &str) -> &str {
    line.trim_left()
}
