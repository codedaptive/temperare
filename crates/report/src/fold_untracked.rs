//! `fold_untracked`: git's untracked entries folded into the added lines.

use std::collections::BTreeMap;
use std::ops::Range;
use std::path::PathBuf;

/// Inserts each untracked file into `lines` as wholly added (`1..u32::MAX`) and returns, in input order, every entry
/// whose text ends in '/': git lists an untracked nested repository that way (S-19), and it covers every path under
/// it. The slash is read from the raw text because `Path` equality and components ignore a trailing slash.
pub(crate) fn fold_untracked(lines: &mut BTreeMap<PathBuf, Vec<Range<u32>>>, untracked: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut prefixes = Vec::new();
    for path in untracked {
        if path.as_os_str().as_encoded_bytes().ends_with(b"/") {
            prefixes.push(path);
        } else {
            lines.insert(path, vec![1..u32::MAX]);
        }
    }
    prefixes
}
