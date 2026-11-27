//! `run_uninstall`: `cctao uninstall` from the install record to the exit code (S-32).
//!
//! `failed`, the two lines it writes on a failure, is unit-tested in the sibling run_uninstall_tests.rs; the disk steps
//! it runs are covered by the zone integration target `tests/install` (uninstall.rs) on a temporary HOME. The removal
//! order is the pure `uninstall_order`, which has its own unit tests.

use std::fs;
use std::io::{ErrorKind, Write};
use std::path::Path;

use crate::install::{apply_uninstall, parse_record, read_record_text, uninstall_order};
use crate::{InstallHost, InstallPaths, RecordEntry};

use super::leftover_folders_next::leftover_folders_next;

/// Removes what the record at `.local/state/cctao/installed` names (`apply_uninstall`: files by path, folders install
/// made once they are empty), one line on `out` per removal, in `uninstall_order`'s two stages. The first removes the
/// files, the folders that do not hold the record and then the record itself: a failure there leaves the record, and
/// the `next:` line says to run uninstall again. The second removes the folders that hold the record: a failure
/// there comes after the record is gone, and the `next:` line (`leftover_folders_next`) names every folder of this
/// stage still there (`left_folders`), else the state folder, to remove by hand only if it is empty. No record: nothing
/// is removed, a line on `out` says so, and it returns 0.
/// The record is read through `read_record_text`, so a symbolic link or anything else that is not a plain file at its
/// path is refused, nothing is removed and nothing it points at is read. Returns 0 when done; 1 when the record is
/// refused or cannot be read or a removal fails, with the `failed` text on `err`. Write errors on `out` and `err` are
/// ignored.
pub fn run_uninstall(host: &InstallHost, out: &mut dyn Write, err: &mut dyn Write) -> u8 {
    let paths = InstallPaths::under(&host.home);
    let text = match read_record_text(&paths.record) {
        Ok(Some(text)) => text,
        Ok(None) => {
            let _ = writeln!(out, "nothing to remove: no install record at {}", paths.record.display());
            return 0;
        }
        Err(e) => return failed(&e.to_string(), AGAIN, err),
    };
    let (first, second) = uninstall_order(&parse_record(&text), &paths.record);
    if let Err(e) = apply_uninstall(&first, out) {
        return failed(&e.to_string(), AGAIN, err);
    }
    match apply_uninstall(&second, out) {
        Ok(()) => 0,
        Err(e) => {
            let mut left = left_folders(&second);
            if left.is_empty() {
                left.push(paths.state_dir.as_path());
            }
            failed(&e.to_string(), &leftover_folders_next(&left), err)
        }
    }
}

/// Every folder of the second stage still there, in `second`'s order (deepest first). A folder counts as left unless
/// `fs::symlink_metadata` says it is not found: one whose state cannot be read is still named, since it may be there.
fn left_folders(second: &[RecordEntry]) -> Vec<&Path> {
    second
        .iter()
        .filter_map(|entry| match entry {
            RecordEntry::Dir(dir) => Some(dir.as_path()),
            _ => None,
        })
        .filter(|dir| !matches!(fs::symlink_metadata(dir), Err(e) if e.kind() == ErrorKind::NotFound))
        .collect()
}

/// The `next:` line while the record is still there to uninstall again from.
const AGAIN: &str = "Fix the cause above and run cctao uninstall again.";

/// Writes `cctao: cannot uninstall: <what>` and `next: <next>` on `err`; returns 1.
pub(super) fn failed(what: &str, next: &str, err: &mut dyn Write) -> u8 {
    let _ = writeln!(err, "cctao: cannot uninstall: {what}\nnext: {next}");
    1
}
