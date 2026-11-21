//! `git`: how the engines tests run git in a throwaway repository, except `git_isolation.rs`, which calls
//! `procio::git_cmd` itself for the caller repository it guards.

use std::path::Path;

/// Runs `git <args>` in the throwaway repository `repo` with `stdin`, panics with git's stderr unless the exit code
/// is 0, and returns stdout. It builds the command with `procio::git_cmd`, which clears the inherited git variables
/// (`GIT_DIR`, `GIT_INDEX_FILE`, `GIT_WORK_TREE` and the rest), so a test run from a git hook cannot touch the
/// caller's repository.
pub fn git(repo: &Path, args: &[&str], stdin: &[u8]) -> Vec<u8> {
    let mut cmd = procio::git_cmd(repo, args.iter().map(|a| (*a).into()).collect());
    cmd.stdin = stdin.to_vec();
    let out = procio::run(&cmd).unwrap();
    assert_eq!(out.code, Some(0), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
    out.stdout
}
