//! `program_for`: the program an entry runs, for availability, the version probe and the run. It reads the disk, so
//! its tests are the zone integration tests in integration/program_for_tests.rs and it has no unit-test sibling.

use crate::entry::Entry;
use rules::Engine;
use std::ffi::{OsStr, OsString};
use std::path::Path;

/// The program `entry` runs in `folder`, the rule file's absolute canonical folder (a relative one finds no local
/// eslint). ESLint's is the executable file `procio::which_in_dir` finds for `eslint` in `<folder>/node_modules/.bin`
/// (on Windows with PATHEXT's extensions, so npm's `eslint.cmd`), run by that absolute path: a project's own eslint
/// comes before PATH's (executable means an execute bit on unix; elsewhere any regular file). A non-executable file
/// there, no such file, or no folder: `entry.program`, which the run finds on PATH. Every other engine reads no disk;
/// stylelint and markdownlint-cli2 are PATH's only.
pub(crate) fn program_for(entry: &Entry, folder: Option<&Path>) -> OsString {
    match (entry.engine, folder) {
        (Engine::Eslint, Some(folder)) => {
            procio::which_in_dir(&folder.join("node_modules/.bin"), OsStr::new("eslint"))
                .map_or_else(|| OsString::from(entry.program), |local| local.into_os_string())
        }
        _ => OsString::from(entry.program),
    }
}
