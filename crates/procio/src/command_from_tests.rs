use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::command_from::command_from;
use crate::Cmd;

fn git() -> &'static OsStr {
    OsStr::new("git")
}

#[test]
fn the_given_program_is_spawned_with_args_as_given() {
    let cmd = Cmd { args: vec!["log".into(), "--format=%H %s".into()], ..Cmd::new("git") };
    let command = command_from(&cmd, OsStr::new("/usr/bin/git"));
    assert_eq!(command.get_program(), OsStr::new("/usr/bin/git"));
    let args: Vec<&OsStr> = command.get_args().collect();
    assert_eq!(args, [OsStr::new("log"), OsStr::new("--format=%H %s")]);
}

#[test]
fn cwd_is_set_only_when_given() {
    assert_eq!(command_from(&Cmd::new("git"), git()).get_current_dir(), None);
    let cmd = Cmd { cwd: Some(PathBuf::from("/tmp/repo")), ..Cmd::new("git") };
    assert_eq!(command_from(&cmd, git()).get_current_dir(), Some(Path::new("/tmp/repo")));
}

#[test]
fn no_environment_changes_by_default() {
    assert_eq!(command_from(&Cmd::new("git"), git()).get_envs().count(), 0);
}

#[test]
fn removed_names_are_none_and_added_names_have_their_value() {
    let cmd = Cmd {
        env: vec![("ADDED".into(), "1".into()), ("BOTH".into(), "kept".into())],
        env_remove: vec!["BOTH".into(), "GONE".into()],
        ..Cmd::new("git")
    };
    let command = command_from(&cmd, git());
    let envs: Vec<(&OsStr, Option<&OsStr>)> = command.get_envs().collect();
    assert_eq!(
        envs,
        [
            (OsStr::new("ADDED"), Some(OsStr::new("1"))),
            (OsStr::new("BOTH"), Some(OsStr::new("kept"))),
            (OsStr::new("GONE"), None),
        ]
    );
}
