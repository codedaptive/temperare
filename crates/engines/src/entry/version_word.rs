//! `version_word`: the version in an engine's usual `--version` output.

use procio::Output;

/// For a probe that exited 0: on stdout's first non-empty line, the first whitespace-separated word that starts with
/// an ASCII digit, with any trailing `,` or `)` trimmed ("ruff 0.17.4" and "0.59.1" alike). `None` for any other
/// exit, or when that line has no such word.
pub fn version_word(out: &Output) -> Option<String> {
    if out.code != Some(0) {
        return None;
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let line = stdout.lines().find(|line| !line.trim().is_empty())?;
    let word = line.split_whitespace().find(|word| word.starts_with(|c: char| c.is_ascii_digit()))?;
    Some(word.trim_end_matches([',', ')']).to_string())
}
