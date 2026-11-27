//! Unit test for detekt_entry.rs: `DETEKT`'s fields, the bare command `entry_cmd` builds from it (the flow adds the
//! arguments), and the version read from detekt 1.23.8's real `--version` text. Nothing runs.

use super::DETEKT;
use crate::entry::entry_cmd;
use procio::Output;
use rules::Engine;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

#[test]
fn the_entry_is_detekt_needing_detekt() {
    assert_eq!((DETEKT.name, DETEKT.engine, DETEKT.program), ("detekt", Engine::Detekt, "detekt"));
    assert_eq!(DETEKT.needs, ["detekt"]);
    assert_eq!(DETEKT.never, None);
    assert_eq!(DETEKT.config, None);
}

#[test]
fn entry_cmd_is_bare_detekt_in_the_folder_whatever_the_config() {
    let cmd = entry_cmd(&DETEKT, OsStr::new("MagicNumber"), &[], Some("lint/detekt.yml"), Path::new("/w/kt"));
    assert_eq!(cmd.program, OsString::from("detekt"));
    assert!(cmd.args.is_empty(), "{:?}", cmd.args);
    assert_eq!(cmd.cwd, Some(PathBuf::from("/w/kt")));
}

#[test]
fn the_version_is_read_from_detekt_dash_dash_version() {
    let probe = DETEKT.version.unwrap();
    assert_eq!(probe.args, ["--version"]);
    let out = Output { code: Some(0), stdout: b"1.23.8\n".to_vec(), stderr: Vec::new() };
    assert_eq!((probe.read)(&out), Some("1.23.8".to_string()));
}
