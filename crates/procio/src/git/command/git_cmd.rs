//! `git_cmd`: the command every git operation runs, pointed at one repository and nothing else.

use std::ffi::OsString;
use std::path::Path;

use crate::Cmd;

/// Variables that would point git at a repository, index, object store or shared object and ref store other than
/// `repo`'s, put its refs under a namespace, or change how it resolves paths and pathspecs. git sets several of them
/// when it runs a hook, so a `tm` started from a hook would inherit them.
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

/// `git -C <repo> <args…>` with the inherited git variables above removed and the three above set. No cwd, since
/// `-C` names the repository; empty stdin; no timeout, so every git operation waits for git as long as it runs.
pub(crate) fn git_cmd(repo: &Path, args: Vec<OsString>) -> Cmd {
    Cmd {
        args: [OsString::from("-C"), repo.as_os_str().to_os_string()].into_iter().chain(args).collect(),
        env: SET.iter().map(|(name, value)| (OsString::from(name), OsString::from(value))).collect(),
        env_remove: CLEARED.iter().map(OsString::from).collect(),
        ..Cmd::new("git")
    }
}
