//! `advisory_db`: the advisory database cctao names to cargo-audit: cargo-audit's own default path.

use std::ffi::OsStr;
use std::path::PathBuf;

/// `<cargo_home>/advisory-db` when `cargo_home` is set and non-empty, else `<home>/.cargo/advisory-db` when `home` is,
/// else None: cargo-audit's own default database path, given the values of CARGO_HOME and HOME.
pub(crate) fn advisory_db(cargo_home: Option<&OsStr>, home: Option<&OsStr>) -> Option<PathBuf> {
    let set = |value: Option<&OsStr>| value.filter(|value| !value.is_empty()).map(PathBuf::from);
    set(cargo_home).or_else(|| set(home).map(|home| home.join(".cargo"))).map(|base| base.join("advisory-db"))
}
