//! `parse_added_lines`: the lines each file gained, read from the output of `git diff -U0`.

use std::collections::BTreeMap;
use std::ops::Range;
use std::path::PathBuf;

use super::{parse_hunk_header, path_from_bytes, unquote_c_path};

/// The lines each file gained, read from the stdout of
/// `git diff -U0 --no-color --no-renames --no-ext-diff --no-prefix --no-textconv <base> -- [paths]`.
///
/// Ranges are 1-based, half-open line numbers in the new file, in file order. Each `+++` line names a file and
/// puts it in the map, so a file whose hunks only remove lines is present with no ranges. Off unix, non-UTF-8
/// paths that read the same after U+FFFD merge their ranges under one key. A deleted file
/// (`+++ /dev/null`) and files git prints with no `+++` line (binary, mode-only, new and empty) are absent.
/// `None` when the output is malformed: a bad hunk header, a bad quoted path, or a hunk body cut short.
pub(crate) fn parse_added_lines(diff: &[u8]) -> Option<BTreeMap<PathBuf, Vec<Range<u32>>>> {
    let mut files: BTreeMap<PathBuf, Vec<Range<u32>>> = BTreeMap::new();
    let mut current: Option<PathBuf> = None;
    let mut lines = diff.split(|byte| *byte == b'\n');
    while let Some(line) = lines.next() {
        if let Some(name) = line.strip_prefix(b"+++ ") {
            current = file_path(name)?;
            if let Some(path) = &current {
                files.entry(path.clone()).or_default();
            }
        } else if line.starts_with(b"@@ ") {
            let (old_count, new_start, new_count) = parse_hunk_header(line)?;
            if let Some(ranges) = current.as_ref().and_then(|path| files.get_mut(path)).filter(|_| new_count > 0) {
                ranges.push(new_start..new_start.checked_add(new_count)?);
            }
            skip_body(&mut lines, old_count.checked_add(new_count)?)?;
        }
    }
    Some(files)
}

/// The path on a `+++ ` line: `Some(None)` for `/dev/null` (a deleted file), `None` for a bad quoted path.
///
/// Git appends a tab to a name that contains a space, so one trailing tab is dropped. With `--no-prefix` the
/// name carries no `b/`.
fn file_path(name: &[u8]) -> Option<Option<PathBuf>> {
    let name = name.strip_suffix(b"\t").unwrap_or(name);
    if name == b"/dev/null" {
        return Some(None);
    }
    let bytes = if name.starts_with(b"\"") { unquote_c_path(name)? } else { name.to_vec() };
    Some(Some(path_from_bytes(bytes)))
}

/// Steps past a hunk's `count` body lines without reading them as headers: an added line can itself read
/// `+++ x` or `@@ `. A `\ No newline at end of file` line is not a body line. With `-U0` every body line starts
/// `+` or `-`, so an empty line (the end of the input) means the body was cut short: `None`.
fn skip_body<'a>(lines: &mut impl Iterator<Item = &'a [u8]>, count: u32) -> Option<()> {
    let mut left = count;
    while left > 0 {
        let line = lines.next().filter(|line| !line.is_empty())?;
        if !line.starts_with(b"\\") {
            left -= 1;
        }
    }
    Some(())
}
