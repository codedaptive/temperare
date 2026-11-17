use std::ffi::OsString;
use std::path::PathBuf;

use crate::Cmd;

#[test]
fn everything_but_the_program_is_empty_or_none() {
    let cmd = Cmd::new("git");
    assert!(cmd.args.is_empty());
    assert_eq!(cmd.cwd, None);
    assert!(cmd.env.is_empty());
    assert!(cmd.env_remove.is_empty());
    assert!(cmd.stdin.is_empty());
    assert_eq!(cmd.timeout, None);
}

#[test]
fn program_from_str() {
    assert_eq!(Cmd::new("git").program, OsString::from("git"));
}

#[test]
fn program_from_path_buf() {
    assert_eq!(Cmd::new(PathBuf::from("/usr/bin/git")).program, OsString::from("/usr/bin/git"));
}
