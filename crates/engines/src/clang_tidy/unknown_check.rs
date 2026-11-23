//! `unknown_check`: clang-tidy's words saying the rule's check enabled no check it knows.

use procio::Output;

/// Some("Error: no checks enabled.") when a line of `out`'s stderr, trimmed, is exactly that; else None. The run
/// enables only `-*,<check>`, so clang-tidy enabling nothing means it does not know the rule's check. `_check` is not
/// read: the words never name it, and the shape is `Entry.unknown`'s.
pub(crate) fn unknown_check(out: &Output, _check: &str) -> Option<String> {
    const SAID: &str = "Error: no checks enabled.";
    String::from_utf8_lossy(&out.stderr).lines().any(|line| line.trim() == SAID).then(|| SAID.to_string())
}
