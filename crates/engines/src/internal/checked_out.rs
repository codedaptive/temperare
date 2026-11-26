//! `checked_out`: the tracked paths an engine can be given, kept from ccvcslib's tracked listing.

use ccvcslib::{TrackedEntry, WorkTreeState};
use std::path::PathBuf;

/// The paths of the `CheckedOut` entries, in listing order. A tracked path with no file in the work tree (deleted
/// from it, a skip-worktree entry, a jj sparse folder's left-out path) is `Missing` or `LeftOut` and left out: there
/// is nothing on disk for a rule to check.
pub(crate) fn checked_out(entries: Vec<TrackedEntry>) -> Vec<PathBuf> {
    entries.into_iter().filter(|entry| entry.state == WorkTreeState::CheckedOut).map(|entry| entry.path).collect()
}
