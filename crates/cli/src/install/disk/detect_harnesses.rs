//! `detect_harnesses`: the harnesses whose folder is in HOME.
//!
//! No sibling unit test: it reads the disk, which a unit test may not, so the zone integration target
//! `tests/install` (harnesses.rs) proves it on a temporary HOME.

use std::path::Path;

use crate::Harness;

/// Each harness of `Harness::ALL`, in that order, whose folder (`.claude`, `.codex`, `.cursor`) is a folder in
/// `home`, following links.
pub(crate) fn detect_harnesses(home: &Path) -> Vec<Harness> {
    Harness::ALL.into_iter().filter(|harness| home.join(harness.folder()).is_dir()).collect()
}
