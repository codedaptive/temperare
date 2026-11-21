//! `setup_heavier_hits`: the files the change adds lines to whose setup outweighs their tests.

use std::collections::BTreeMap;
use std::path::Path;

use rules::{RelPath, Scope};

use super::{count_file, CountMatch};
use crate::rel_to_folder::rel_to_folder;
use crate::Hit;

/// One hit per file whose setup lines exceed its test lines, in path order. `matches` are ast-grep's, with paths as
/// it printed them from `cwd`; a file not under `folder` is skipped. `added` maps each file the change adds lines to,
/// relative to `folder`, to its first added line (1-based), where the hit is placed; a file not in it is not
/// reported. A file `shared_setup` matches is exempt; None exempts nothing.
pub(crate) fn setup_heavier_hits(
    matches: &[CountMatch],
    cwd: &Path,
    folder: &Path,
    added: &BTreeMap<RelPath, u32>,
    shared_setup: Option<&Scope>,
) -> Vec<Hit> {
    let mut by_file: BTreeMap<RelPath, Vec<&CountMatch>> = BTreeMap::new();
    for found in matches {
        if let Some(path) = rel_to_folder(cwd, folder, &found.file) {
            by_file.entry(path).or_default().push(found);
        }
    }
    by_file
        .into_iter()
        .filter(|(path, _)| !shared_setup.is_some_and(|scope| scope.matches(path)))
        .filter_map(|(path, found)| {
            let line = *added.get(&path)?;
            let count = count_file(&found).filter(|count| count.setup > count.tests)?;
            let what = format!("{} lines of setup, {} lines of tests", count.setup, count.tests);
            Some(Hit { path, line: Some(line), col: None, what })
        })
        .collect()
}
