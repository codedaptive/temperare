//! `cargo_root`: the folder `cargo clippy` runs in for a rule file's folder.

use std::path::{Path, PathBuf};

/// The nearest ancestor of `folder`, `folder` itself included, that holds a `Cargo.toml`; None when no ancestor does.
/// For a member crate this is the member's folder, not the workspace root: clippy runs here so only this crate is
/// linted, and `hits_clippy` reads the span paths, which cargo prints relative to the workspace root, against the
/// root `cargo locate-project --workspace` names.
pub fn cargo_root(folder: &Path) -> Option<PathBuf> {
    folder.ancestors().find(|dir| dir.join("Cargo.toml").is_file()).map(Path::to_path_buf)
}
