//! ZUT for cargo_audit_cmd.rs: the program, the exact argument list and the working folder. Nothing runs.

use super::cargo_audit_cmd;
use procio::Cmd;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[test]
fn cargo_audit_cmd_cases() {
    let expected = Cmd {
        args: ["audit", "--no-fetch", "--no-yanked", "--json", "--db", "/c/advisory-db", "--file", "/w/Cargo.lock"]
            .iter()
            .map(OsString::from)
            .collect(),
        cwd: Some(PathBuf::from("/w")),
        ..Cmd::new("cargo-audit")
    };
    assert_eq!(cargo_audit_cmd(Path::new("/c/advisory-db"), Path::new("/w")), expected, "a database and a folder");
}
