use std::ffi::OsString;
use std::path::Path;

use crate::git_cmd;

fn os(list: &[&str]) -> Vec<OsString> {
    list.iter().map(OsString::from).collect()
}

#[test]
fn runs_git() {
    assert_eq!(git_cmd(Path::new("/r"), Vec::new()).program, OsString::from("git"));
}

#[test]
fn args_start_with_dash_c_repo_then_the_callers_in_order() {
    let cmd = git_cmd(Path::new("/work/my repo"), os(&["diff", "--name-only", "-z", "HEAD", "--"]));
    assert_eq!(cmd.args, os(&["-C", "/work/my repo", "diff", "--name-only", "-z", "HEAD", "--"]));
}

#[test]
fn no_caller_args_leaves_only_dash_c_repo() {
    assert_eq!(git_cmd(Path::new("/r"), Vec::new()).args, os(&["-C", "/r"]));
}

#[test]
fn removes_the_inherited_git_variables_in_order() {
    let expected = os(&[
        "GIT_DIR",
        "GIT_WORK_TREE",
        "GIT_INDEX_FILE",
        "GIT_PREFIX",
        "GIT_OBJECT_DIRECTORY",
        "GIT_ALTERNATE_OBJECT_DIRECTORIES",
        "GIT_LITERAL_PATHSPECS",
        "GIT_GLOB_PATHSPECS",
        "GIT_NOGLOB_PATHSPECS",
        "GIT_ICASE_PATHSPECS",
        "GIT_COMMON_DIR",
        "GIT_NAMESPACE",
    ]);
    assert_eq!(git_cmd(Path::new("/r"), Vec::new()).env_remove, expected);
}

#[test]
fn sets_c_locale_no_prompt_and_no_optional_locks() {
    let expected: Vec<(OsString, OsString)> = [("LC_ALL", "C"), ("GIT_TERMINAL_PROMPT", "0"), ("GIT_OPTIONAL_LOCKS", "0")]
        .iter()
        .map(|(n, v)| (OsString::from(n), OsString::from(v)))
        .collect();
    assert_eq!(git_cmd(Path::new("/r"), Vec::new()).env, expected);
}

#[test]
fn no_cwd_empty_stdin_and_no_timeout() {
    let cmd = git_cmd(Path::new("/r"), os(&["status"]));
    assert_eq!(cmd.cwd, None);
    assert!(cmd.stdin.is_empty());
    assert_eq!(cmd.timeout, None);
}
