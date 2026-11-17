//! `printed_start`: the start of what an engine printed, quoted when its output is not an answer.

use procio::Output;

/// The first 200 bytes of `out`'s stderr, or of its stdout when stderr is empty, as lossy UTF-8. Stderr comes first
/// because an engine that fails explains why there; 200 bytes keeps a quoted error to one readable line or two.
pub(crate) fn printed_start(out: &Output) -> String {
    let printed = if out.stderr.is_empty() { &out.stdout } else { &out.stderr };
    String::from_utf8_lossy(&printed[..printed.len().min(200)]).into_owned()
}
