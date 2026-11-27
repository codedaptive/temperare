//! `run_markdownlint`: markdownlint-cli2's flow for one rule over git's listing and the real filesystem. It runs git
//! and the engine and writes a temporary options module, so its test is the zone integration test
//! tests/markdownlint_cli2.rs and it has no unit-test sibling.

use super::{is_markdown_source, run_markdownlint_with};
use crate::entry::Entry;
use crate::host::Host;
use crate::source_files::source_files;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::Path;

/// `run_markdownlint_with` listing the Markdown files with `source_files` and `is_markdown_source`, and reading the
/// real filesystem; a `RunFn`, the flow of `MARKDOWNLINT`.
pub(crate) fn run_markdownlint(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    run_markdownlint_with(
        &|folder: &Path| source_files(folder, is_markdown_source),
        &|path: &Path| path.is_file(),
        &|path: &Path| std::fs::read_to_string(path),
        entry,
        rule,
        folder,
        host,
    )
}
