//! `run_stylelint`: stylelint's flow for one rule, the entry's `RunFn`, given the CSS files git lists. It is
//! `run_stylelint_with` over the real `source_files`, which runs git and reads the filesystem, so it has no unit-test
//! sibling; the zone integration test tests/stylelint.rs covers it.

use super::{is_css_source, run_stylelint_with};
use crate::entry::Entry;
use crate::host::Host;
use crate::source_files::source_files;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::Path;

/// `run_stylelint_with` given `source_files` with `is_css_source`, so stylelint is given the `.css` files git lists
/// under `folder` that the rule applies to.
pub(crate) fn run_stylelint(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    run_stylelint_with(&|folder: &Path| source_files(folder, is_css_source), entry, rule, folder, host)
}
