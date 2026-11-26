//! `markdownlint_version`: the markdownlint-cli2 version its `--help` prints.

use procio::Output;

/// The banner markdownlint-cli2 prints first, before its version (`markdownlint-cli2 v0.24.2 (markdownlint v0.42.2)`).
const BANNER: &str = "markdownlint-cli2 v";

/// On stdout's first non-empty line, when it starts with `markdownlint-cli2 v` (leading whitespace is skipped first):
/// the word after that `v` when it starts with an ASCII digit (`0.24.2`). Whatever the exit code: `--help`, the probe,
/// exits 2 after printing it, and `--version` is no flag to markdownlint-cli2 (it would lint and read the repository's
/// options). `None` for any other line.
pub(crate) fn markdownlint_version(out: &Output) -> Option<String> {
    let stdout = String::from_utf8_lossy(&out.stdout);
    let line = stdout.lines().find(|line| !line.trim().is_empty())?;
    let word = line.trim_start().strip_prefix(BANNER)?.split_whitespace().next()?;
    word.starts_with(|c: char| c.is_ascii_digit()).then(|| word.to_string())
}
