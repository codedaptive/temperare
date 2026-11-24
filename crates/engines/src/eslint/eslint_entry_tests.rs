//! ZUT for eslint_entry.rs: the fields of `ESLINT` that carry the contract, through the command `entry_cmd` builds
//! from it (the check is not passed there: `run_eslint_with` adds the `--rule` pair) and its version probe.

use super::ESLINT;
use crate::entry::{entry_cmd, Pick};
use procio::Output;
use rules::Engine;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

#[test]
fn eslint_entry_builds_the_command_and_reads_the_version() {
    assert_eq!((ESLINT.name, ESLINT.engine, ESLINT.program), ("eslint", Engine::Eslint, "eslint"));
    assert_eq!((ESLINT.needs, ESLINT.never, ESLINT.pick), (&["eslint"][..], None, Pick::None));
    let cmd = entry_cmd(&ESLINT, OsStr::new("no-var"), &[], Some("cfg/eslint.config.mjs"), Path::new("/w/js"));
    let want = [
        "--format",
        "json",
        "--no-inline-config",
        "--no-ignore",
        "--pass-on-unpruned-suppressions",
        "--no-error-on-unmatched-pattern",
        "--config=cfg/eslint.config.mjs",
        ".",
    ];
    assert_eq!((cmd.program, cmd.cwd), (OsString::from("eslint"), Some(PathBuf::from("/w/js"))));
    assert_eq!(cmd.args, want.map(OsString::from));
    assert!(cmd.env.is_empty(), "{:?}", cmd.env);
    let probe = ESLINT.version.unwrap();
    let out = Output { code: Some(0), stdout: b"v10.15.0\n".to_vec(), stderr: Vec::new() };
    assert_eq!((probe.args, (probe.read)(&out)), (&["--version"][..], Some("10.15.0".to_string())));
}
