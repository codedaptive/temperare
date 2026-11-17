//! `parse_locate_workspace`: the workspace root folder, read from `cargo locate-project --workspace` output.

use crate::printed_start::printed_start;
use crate::types::ParseError;
use procio::Output;
use std::path::{Path, PathBuf};

/// The folder holding the workspace's root `Cargo.toml`, read from `out`'s trimmed stdout. A non-zero exit, empty
/// output, a relative path or one whose last part is not `Cargo.toml` is a `ParseError` quoting what cargo printed.
pub fn parse_locate_workspace(out: &Output) -> Result<PathBuf, ParseError> {
    let fail = |what: &str| ParseError { why: format!("cargo locate-project: {what}: {:?}", printed_start(out)) };
    if out.code != Some(0) {
        return Err(fail("it did not exit 0"));
    }
    let printed = String::from_utf8_lossy(&out.stdout);
    let manifest = Path::new(printed.trim());
    if !manifest.is_absolute() || manifest.file_name() != Some("Cargo.toml".as_ref()) {
        return Err(fail("not an absolute path to a Cargo.toml"));
    }
    manifest.parent().map(Path::to_path_buf).ok_or_else(|| fail("no folder above the Cargo.toml"))
}
