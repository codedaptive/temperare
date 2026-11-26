//! `program_for`: the program an entry runs, for availability, the version probe and the run. It reads the disk, so
//! its tests are the zone integration tests in integration/program_for_tests.rs and it has no unit-test sibling.

use crate::entry::Entry;
use rules::Engine;
use std::ffi::OsString;
use std::path::Path;

/// The program `entry` runs in `folder`, the rule file's canonical folder. ESLint's is
/// `<folder>/node_modules/.bin/eslint` when that is a file (`is_file` follows a symlink, as npm's `.bin` entries
/// are), run by that absolute path: a project's own eslint comes before PATH's. Every other case, ESLint with no such
/// file or no folder included, is `entry.program`, which the run finds on PATH. stylelint and markdownlint-cli2 are
/// PATH's only.
pub(crate) fn program_for(entry: &Entry, folder: Option<&Path>) -> OsString {
    match (entry.engine, folder.map(|folder| folder.join("node_modules/.bin/eslint"))) {
        (Engine::Eslint, Some(local)) if local.is_file() => local.into_os_string(),
        _ => OsString::from(entry.program),
    }
}
