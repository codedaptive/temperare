//! `known_advisory`: whether the advisory database holds an advisory id. Reads the disk, so it has no unit test:
//! tests/cargo_audit.rs (unknown advisory id) tests it.

use std::path::Path;

/// True when some `<db>/crates/<crate>/<id>.md` or `<db>/rust/<crate>/<id>.md` is a file: the two collections of the
/// RustSec database, one folder per crate. A folder that cannot be read counts as holding no advisory.
pub(crate) fn known_advisory(db: &Path, id: &str) -> bool {
    let file = format!("{id}.md");
    ["crates", "rust"]
        .iter()
        .filter_map(|collection| std::fs::read_dir(db.join(collection)).ok())
        .flatten()
        .filter_map(Result::ok)
        .any(|crate_folder| crate_folder.path().join(&file).is_file())
}
