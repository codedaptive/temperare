//! `unknown_code`: mypy's words saying it does not know the rule's error code.

use procio::Output;

/// The marker mypy 2.4.1 puts before the codes it does not know, as in
/// "mypy: error: Invalid error code(s): nonsense-code, other-bad" (it then exits 2 and checks nothing).
const INVALID: &str = "Invalid error code(s): ";

/// The first line of `out`'s stderr whose comma-separated list after `INVALID`, each item trimmed, holds `check`
/// exactly, trimmed; None when no line does. A line naming only other codes (ones in the repository's config, say)
/// is not about the check, so it is left to the reader.
pub(crate) fn unknown_code(out: &Output, check: &str) -> Option<String> {
    String::from_utf8_lossy(&out.stderr)
        .lines()
        .find(|line| {
            line.split_once(INVALID).is_some_and(|(_, codes)| codes.split(',').any(|code| code.trim() == check))
        })
        .map(|line| line.trim().to_string())
}
