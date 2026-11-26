use std::ffi::OsString;
use std::num::NonZeroU32;

use crate::kill_group_cmd::kill_group_cmd;

fn pgid(n: u32) -> NonZeroU32 {
    NonZeroU32::new(n).unwrap()
}

#[test]
fn runs_kill_with_sigkill_to_the_negative_group_id() {
    let cmd = kill_group_cmd(pgid(4242));
    assert_eq!(cmd.program, OsString::from("kill"));
    let args: Vec<OsString> = vec!["-KILL".into(), "--".into(), "-4242".into()];
    assert_eq!(cmd.args, args);
}

#[test]
fn group_id_one_and_the_largest_are_written_whole() {
    assert_eq!(kill_group_cmd(pgid(1)).args[2], OsString::from("-1"));
    assert_eq!(kill_group_cmd(pgid(4294967295)).args[2], OsString::from("-4294967295"));
}

#[test]
fn sets_only_a_fixed_path_and_has_no_timeout_cwd_or_stdin() {
    let cmd = kill_group_cmd(pgid(4242));
    assert_eq!(cmd.timeout, None);
    assert_eq!(cmd.cwd, None);
    let env: Vec<(OsString, OsString)> = vec![("PATH".into(), "/bin:/usr/bin".into())];
    assert_eq!(cmd.env, env);
    assert!(cmd.env_remove.is_empty());
    assert!(cmd.stdin.is_empty());
}
