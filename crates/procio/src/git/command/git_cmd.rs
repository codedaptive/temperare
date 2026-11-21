//! `git_cmd`: the command every git operation runs, pointed at one repository and nothing else.

use std::ffi::OsString;
use std::path::Path;

use crate::Cmd;

/// Variables that would point git at a repository, index, object store or shared object and ref store other than
/// `repo`'s, put its refs under a namespace, or change how it resolves paths and pathspecs. git sets several of them
/// when it runs a hook, so any process started from a hook, a caller of `git_cmd` included, would inherit them.
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

/// `LC_ALL=C` keeps the messages the operations read the same in every locale; `GIT_TERMINAL_PROMPT=0` stops git
/// waiting on a prompt for credentials; `GIT_OPTIONAL_LOCKS=0` stops read-only commands taking the index lock to
/// refresh it, so a check never contends with the user's own git.
const SET: [(&str, &str); 3] = [("LC_ALL", "C"), ("GIT_TERMINAL_PROMPT", "0"), ("GIT_OPTIONAL_LOCKS", "0")];

/// Builds, without running it, `git -C <repo> <args…>` as a [`Cmd`] to pass to [`run`](crate::run()).
///
/// It removes twelve inherited git variables that would point git at another repository, work tree, index, object
/// store or namespace, or change how it reads pathspecs; git sets several of them inside a hook. It sets `LC_ALL=C`,
/// so git's messages read the same in every locale; `GIT_TERMINAL_PROMPT=0`, so git never waits on a terminal
/// prompt for credentials; and `GIT_OPTIONAL_LOCKS=0`, so a read-only command never takes the index lock from the
/// user's own git. No cwd, since `-C` names the repository; empty stdin; no timeout: set `cmd.timeout` before `run`
/// if one is wanted.
pub fn git_cmd(repo: &Path, args: Vec<OsString>) -> Cmd {
    Cmd {
        args: [OsString::from("-C"), repo.as_os_str().to_os_string()].into_iter().chain(args).collect(),
        env: SET.iter().map(|(name, value)| (OsString::from(name), OsString::from(value))).collect(),
        env_remove: CLEARED.iter().map(OsString::from).collect(),
        ..Cmd::new("git")
    }
}
