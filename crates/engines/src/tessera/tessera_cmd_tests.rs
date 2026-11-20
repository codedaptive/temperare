//! Unit tests for tessera_cmd.rs: the command, its arguments and where it runs.

use super::tessera_cmd;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[test]
fn runs_ts_slop_json_in_the_folder_and_nothing_else() {
    let cmd = tessera_cmd(Path::new("/w/repo/kit"));
    assert_eq!(cmd.program, OsString::from("ts"));
    assert_eq!(cmd.args, vec![OsString::from("slop"), OsString::from("--json")]);
    assert_eq!(cmd.cwd, Some(PathBuf::from("/w/repo/kit")));
    assert!(cmd.env.is_empty());
    assert!(cmd.env_remove.is_empty());
    assert!(cmd.stdin.is_empty());
    assert_eq!(cmd.timeout, None);
}
