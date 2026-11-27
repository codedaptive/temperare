//! `pmd_version`: PMD's version, read from what `pmd --version` prints.

use procio::Output;

/// On exit 0, the second word of the first stdout line starting `PMD ` ("7.29.0" from `PMD 7.29.0 (<hash>, <date>)`);
/// None otherwise. `version_word` cannot read it: PMD prints an ASCII-art banner first and a `Java version:` line
/// after.
pub(crate) fn pmd_version(out: &Output) -> Option<String> {
    if out.code != Some(0) {
        return None;
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let line = stdout.lines().find(|line| line.starts_with("PMD "))?;
    line.split_whitespace().nth(1).map(str::to_string)
}
