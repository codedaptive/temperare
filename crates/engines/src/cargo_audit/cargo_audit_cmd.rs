//! `cargo_audit_cmd`: the cargo-audit run for one folder, as a command.

use procio::Cmd;
use std::path::Path;

/// `cargo-audit audit --no-fetch --no-yanked --json --db <db> --file <folder>/Cargo.lock`, run in `folder`.
/// `--no-fetch` reads the database as it is and `--no-yanked` skips the crates.io index, so nothing reaches the
/// network. `folder` is absolute, and `db` is when CARGO_HOME or HOME is, so no value reads as an option.
pub fn cargo_audit_cmd(db: &Path, folder: &Path) -> Cmd {
    Cmd {
        args: vec![
            "audit".into(),
            "--no-fetch".into(),
            "--no-yanked".into(),
            "--json".into(),
            "--db".into(),
            db.into(),
            "--file".into(),
            folder.join("Cargo.lock").into(),
        ],
        cwd: Some(folder.to_path_buf()),
        ..Cmd::new("cargo-audit")
    }
}
