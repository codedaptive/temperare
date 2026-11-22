//! `change_of`: the lines a change adds under a rule file's folder, against a base commit, for the files a rule
//! keeps. It runs git and reads the real filesystem, so its tests are the zone integration tests in
//! integration/change_of_tests.rs and it has no unit-test sibling.

use crate::git_stop::git_stop;
use crate::is_regular_file::is_regular_file;
use crate::rel_to_folder::rel_to_folder;
use crate::types::Stop;
use rules::RelPath;
use std::ops::Range;
use std::path::Path;

/// How far into an untracked file a NUL byte marks it binary: the span git itself looks at to call a file binary.
const BINARY_SPAN: usize = 8000;

/// Each file under `folder` (canonical) that the change adds lines to and `keep` says yes to, relative to `folder`,
/// with its added lines as 1-based half-open ranges, sorted by path:
/// - a tracked file: procio's `added_lines` against `base` (a file git shows without a text diff, binary among them,
///   is absent; a file with only removed lines has no ranges);
/// - an untracked file git does not ignore (procio's `untracked_files`): all of it is added, `1..n+1` for its `n`
///   lines, a last line without a newline counted; an empty file has no ranges, and a file with a NUL byte in its
///   first 8000 bytes is skipped, as git skips a binary tracked file.
///
/// `keep` is asked with each path relative to `folder`, and a path it says no to is dropped before its contents are
/// read, tracked or untracked, so an unreadable file outside a rule's paths never stops the rule. An untracked path
/// is read only when `is_regular_file` says it is a regular file reached with no symlink below the repository's top;
/// anything else (a symlink, dangling or not, a path under a symlinked folder, a FIFO, a folder git lists as one entry
/// such as an untracked nested repository `inner/`) is skipped, not an error.
///
/// git keys both listings from the repository's top, which is canonicalized so it shares `folder`'s form before
/// `rel_to_folder` cuts it off; a file outside `folder` is dropped. git runs through procio, not the host, so nothing
/// is counted. A git error is `git_stop`'s `Stop`; a kept regular untracked file that cannot be read is `Failed` with
/// no exit code, naming it.
pub(crate) fn change_of(
    folder: &Path,
    base: &str,
    keep: &dyn Fn(&RelPath) -> bool,
) -> Result<Vec<(RelPath, Vec<Range<u32>>)>, Stop> {
    let top = procio::toplevel(folder).map_err(git_stop)?;
    let top = std::fs::canonicalize(&top)
        .map_err(|err| failed(format!("cannot resolve the repository top {}: {err}", top.display())))?;
    let mut change: Vec<(RelPath, Vec<Range<u32>>)> = procio::added_lines(folder, base, &[])
        .map_err(git_stop)?
        .into_iter()
        .filter_map(|(path, ranges)| Some((rel_to_folder(&top, folder, &path.to_string_lossy())?, ranges)))
        .filter(|(rel, _)| keep(rel))
        .collect();
    for path in procio::untracked_files(folder).map_err(git_stop)? {
        let Some(rel) = rel_to_folder(&top, folder, &path.to_string_lossy()).filter(|rel| keep(rel)) else { continue };
        let full = top.join(&path);
        if !is_regular_file(&top, &full) {
            continue;
        }
        if let Some(ranges) = untracked_ranges(&full)? {
            change.push((rel, ranges));
        }
    }
    change.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(change)
}

/// The whole of the untracked file at `full` as added lines: `[1..n+1]`, or none when it is empty; `None` when it is
/// binary.
fn untracked_ranges(full: &Path) -> Result<Option<Vec<Range<u32>>>, Stop> {
    let bytes = std::fs::read(full).map_err(|err| failed(format!("cannot read {}: {err}", full.display())))?;
    if bytes[..bytes.len().min(BINARY_SPAN)].contains(&0) {
        return Ok(None);
    }
    let ends_open = bytes.last().is_some_and(|last| *last != b'\n');
    let lines = bytes.iter().filter(|byte| **byte == b'\n').count() + usize::from(ends_open);
    let end = u32::try_from(lines + 1)
        .map_err(|_| failed(format!("{} has more lines than tm counts", full.display())))?;
    Ok(Some(if lines == 0 { Vec::new() } else { vec![1..end] }))
}

fn failed(stderr: String) -> Stop {
    Stop::Failed { code: None, stderr }
}
