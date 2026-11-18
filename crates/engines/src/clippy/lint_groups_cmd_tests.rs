//! ZUT for lint_groups_cmd.rs: the program, its arguments and where it runs.

use super::lint_groups_cmd;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[test]
fn runs_clippy_driver_w_help_in_the_cargo_root() {
    let cmd = lint_groups_cmd(Path::new("/w/member"));
    assert_eq!(cmd.program, OsString::from("clippy-driver"));
    assert_eq!(cmd.args, [OsString::from("-W"), OsString::from("help")]);
    assert_eq!(cmd.cwd, Some(PathBuf::from("/w/member")));
}
