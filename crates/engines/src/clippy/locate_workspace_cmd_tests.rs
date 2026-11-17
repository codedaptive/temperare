//! ZUT for locate_workspace_cmd.rs: the command, its arguments and where it runs.

use super::locate_workspace_cmd;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[test]
fn runs_cargo_locate_project_for_the_workspace_as_plain_text_in_the_cargo_root() {
    let cmd = locate_workspace_cmd(Path::new("/w/member"));
    assert_eq!(cmd.program, OsString::from("cargo"));
    let want: Vec<OsString> =
        ["locate-project", "--workspace", "--message-format", "plain"].map(OsString::from).into();
    assert_eq!(cmd.args, want);
    assert_eq!(cmd.cwd, Some(PathBuf::from("/w/member")));
    assert!(cmd.env.is_empty());
    assert!(cmd.env_remove.is_empty());
    assert!(cmd.stdin.is_empty());
    assert_eq!(cmd.timeout, None);
}
