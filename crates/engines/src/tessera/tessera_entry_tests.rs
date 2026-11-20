//! ZUT for tessera_entry.rs: `TESSERA` is never available and has no version probe, its command is `ts slop --json` in
//! the folder, and its reader says it has none. Nothing runs.

use super::TESSERA;
use crate::entry::entry_cmd;
use crate::ParseError;
use procio::Output;
use rules::Engine;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

#[test]
fn the_entry_is_never_available_needs_nothing_and_reads_no_version() {
    assert_eq!((TESSERA.name, TESSERA.engine, TESSERA.program), ("tessera", Engine::Tessera, "ts"));
    assert_eq!(TESSERA.never, Some("Tessera's slop records are not defined yet"));
    assert!(TESSERA.needs.is_empty());
    assert!(TESSERA.version.is_none());
}

#[test]
fn runs_ts_slop_json_in_the_folder_and_nothing_else() {
    let cmd = entry_cmd(&TESSERA, OsStr::new("slop"), &[], Some("ts.toml"), Path::new("/w/repo/kit"));
    assert_eq!(cmd.program, OsString::from("ts"));
    assert_eq!(cmd.args, vec![OsString::from("slop"), OsString::from("--json")]);
    assert_eq!(cmd.cwd, Some(PathBuf::from("/w/repo/kit")));
    assert!(cmd.env.is_empty());
    assert!(cmd.env_remove.is_empty());
    assert!(cmd.stdin.is_empty());
    assert_eq!(cmd.timeout, None);
}

#[test]
fn the_reader_says_tessera_has_none() {
    let out = Output { code: Some(0), stdout: b"[]".to_vec(), stderr: Vec::new() };
    let got = (TESSERA.read)(&out, "slop", Path::new("/w"), Path::new("/w"));
    assert_eq!(got, Err(ParseError { why: "tessera has no reader".to_string() }));
}
