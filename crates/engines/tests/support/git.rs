//! `git`: the one helper the engines tests use to run git in a throwaway repository.

use std::ffi::OsString;
use std::path::Path;

/// Variables that would point git at a repository, index, object store or shared object and ref store other than
/// the fixture's, put its refs under a namespace, or change how it reads pathspecs. git sets several of them when it
/// runs a hook, so a test run from a hook would inherit them.
const CLEARED: [&str; 12] = [
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
];

/// `LC_ALL=C` keeps git's messages the same in every locale; `GIT_TERMINAL_PROMPT=0` stops git waiting on a
/// credentials prompt; `GIT_OPTIONAL_LOCKS=0` stops read-only commands taking the index lock.
const SET: [(&str, &str); 3] = [("LC_ALL", "C"), ("GIT_TERMINAL_PROMPT", "0"), ("GIT_OPTIONAL_LOCKS", "0")];

/// Builds `git -C <repo> -c core.hooksPath=/dev/null <args…>` with the variables above cleared and set.
fn git_command(repo: &Path, args: &[&str]) -> procio::Cmd {
    let repo_args = [OsString::from("-C"), repo.into(), "-c".into(), "core.hooksPath=/dev/null".into()];
    procio::Cmd {
        args: repo_args.into_iter().chain(args.iter().map(OsString::from)).collect(),
        env: SET.iter().map(|(name, value)| (OsString::from(name), OsString::from(value))).collect(),
        env_remove: CLEARED.iter().map(OsString::from).collect(),
        ..procio::Cmd::new("git")
    }
}

/// Runs `git <args>` in the throwaway repository `repo` with `stdin`, panics with git's stderr unless the exit code
/// is 0, and returns stdout. It runs program `git` through `procio::run`, clearing the inherited git variables
/// (`GIT_DIR`, `GIT_INDEX_FILE`, `GIT_WORK_TREE` and the rest) so a test run from a git hook cannot touch the
/// caller's repository, and setting `core.hooksPath` to `/dev/null` so a global hook never runs on a fixture commit.
pub fn git(repo: &Path, args: &[&str], stdin: &[u8]) -> Vec<u8> {
    let mut cmd = git_command(repo, args);
    cmd.stdin = stdin.to_vec();
    let out = procio::run(&cmd).unwrap();
    assert_eq!(out.code, Some(0), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
    out.stdout
}
