//! ZUT for dart_entry.rs: the entry's names, its check run's command (no pick, no config) and its version read from
//! dart 3.14.1's real `--version` line (fixtures/commands.txt).

use super::DART;
use crate::entry::{entry_cmd, Params};
use procio::Output;
use rules::Engine;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

#[test]
fn dart_entry_builds_the_command_and_reads_the_version() {
    let cmd = entry_cmd(&DART, OsStr::new("unused_local_variable"), &[], Some("x.yaml"), Path::new("/w/pkg"));
    assert_eq!((DART.name, DART.engine, DART.program), ("dart", Engine::Dart, "dart"));
    assert_eq!((DART.needs, DART.never), (&["dart"][..], None));
    assert!(matches!(DART.params, Params::Args(&[])));
    assert_eq!(cmd.program, OsString::from("dart"));
    let args = ["--suppress-analytics", "analyze", "--format", "machine", "."];
    assert_eq!(cmd.args, args.map(OsString::from));
    assert_eq!(cmd.cwd, Some(PathBuf::from("/w/pkg")));
    let probe = DART.version.unwrap();
    assert_eq!(probe.args, ["--suppress-analytics", "--version"]);
    let line = b"Dart SDK version: 3.14.1 (stable) (Tue Nov 17 00:01:24 2026 -0800) on \"macos_arm64\"\n";
    let out = Output { code: Some(0), stdout: line.to_vec(), stderr: vec![] };
    assert_eq!((probe.read)(&out), Some("3.14.1".to_string()));
    assert_eq!((DART.unknown)(&out, "unused_local_variable"), None);
}
