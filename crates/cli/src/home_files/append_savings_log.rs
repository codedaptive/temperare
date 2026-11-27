//! `append_savings_log`: adds one line to the savings log under HOME.

use std::path::Path;

use super::append_under_home;

/// Appends `line` and a newline, as one `write_all`, to `<home>/.local/state/temperare/savings.jsonl`, making missing
/// folders. A symbolic link or the wrong kind of entry at `.local`, `.local/state`, `.local/state/temperare` or
/// `savings.jsonl` is refused and nothing is written; `append_under_home` has the checks and what they do not catch.
/// Does nothing when there is no HOME; any failure is silent.
pub fn append_savings_log(home: Option<&Path>, line: &str) {
    append_under_home(home, &[".local", "state", "temperare"], "savings.jsonl", line);
}
