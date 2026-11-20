//! `files_in_folder`: the files git lists for a rule file's folder, tracked plus untracked files git does not
//! ignore, relative to it. It runs git, so its test is the zone integration test tests/no_file_flag.rs and it has no
//! unit-test sibling; `folder_files`, which shapes the listing, has its own.

use crate::folder_files::folder_files;
use rules::RelPath;
use std::path::Path;

/// The files under `folder` (canonical), relative to it: procio's `tracked_files` (a file deleted from the work tree
/// and every skip-worktree entry left out), then its `untracked_files`, the untracked files git does not ignore. The
/// untracked ones count because an agent runs tm check before `git add`, and a violation in a new untracked file
/// counts as new (Done 2), so a rule whose paths match only such a file still has a file to check. An ignored file is
/// in neither listing, and an untracked nested repository's files are not listed: git gives it as one folder entry,
/// which `folder_files` drops. git lists both relative to the repository's top, which is canonicalized so it shares `folder`'s
/// form before `folder_files` cuts it off. None when git cannot list them (the folder is in no repository, git is
/// missing or fails): then no rule is decided by its paths and every rule runs. The listing runs git through procio,
/// not the host, so it is not counted.
pub(crate) fn files_in_folder(folder: &Path) -> Option<Vec<RelPath>> {
    let top = std::fs::canonicalize(procio::toplevel(folder).ok()?).ok()?;
    let tracked = procio::tracked_files::<&str>(folder, &[]).ok()?;
    Some(folder_files(&top, folder, &tracked, &procio::untracked_files(folder).ok()?))
}
