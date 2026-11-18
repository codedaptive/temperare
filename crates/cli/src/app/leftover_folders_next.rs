//! `leftover_folders_next`: the `next:` line when uninstall fails after the record is removed.

use std::path::Path;

/// Names every folder in `left`, in the order given, joined by ", ", for removal by hand only if it is empty: a
/// recorded folder such as `.local` may hold other tools' files, so it is never to be removed whole.
pub(super) fn leftover_folders_next(left: &[&Path]) -> String {
    let names: Vec<String> = left.iter().map(|dir| dir.display().to_string()).collect();
    format!("The record is removed; remove each of these folders by hand only if it is empty: {}.", names.join(", "))
}
