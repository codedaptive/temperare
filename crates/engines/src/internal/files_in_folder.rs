//! `files_in_folder`: the files git lists for a rule file's folder, tracked plus untracked files git does not
//! ignore, relative to it, for its callers `run_rules_counted_with`, `no_file_rules` and ESLint's `run_eslint`. It runs
//! git, so its tests are the zone integration tests tests/no_file_flag.rs, tests/no_file_rules.rs and
//! integration/files_in_folder_tests.rs and it has no unit-test sibling; `folder_files`, which shapes the listing, has
//! its own.

use crate::checked_out::checked_out;
use crate::folder_files::folder_files;
use crate::is_regular_file::is_regular_file;
use crate::repo_top::repo_top;
use ccvcslib::{GitVcs, Vcs};
use rules::RelPath;
use std::path::Path;

/// The files under `folder` (canonical), relative to it: ccvcslib's `tracked_entries` as `checked_out` keeps them (a
/// file deleted from the work tree and every skip-worktree entry left out), then its `untracked_entries`, the
/// untracked files git does not ignore. The untracked ones count because an agent runs cctao check before `git add`, and
/// a violation in a new untracked file counts as new (Done 2), so a rule whose paths match only such a file still has
/// a file to check. An ignored file is in neither listing, and an untracked nested repository's files are not listed:
/// git gives it as one folder entry, which `folder_files` drops. A symlink, or a path under a symlinked folder, is not
/// listed (`is_regular_file`): there is no regular file there to check. Both listings are asked at the repository's
/// top (`repo_top`) and are relative to it, which `folder_files` cuts off. None when git cannot list them (the folder
/// is in no repository, git is missing or fails): then no rule is decided by its paths and every rule runs. The
/// listing runs git through ccvcslib, not the host, so it is not counted.
pub(crate) fn files_in_folder(folder: &Path) -> Option<Vec<RelPath>> {
    let top = repo_top(folder).ok()?;
    let vcs = GitVcs::default();
    let tracked = checked_out(vcs.tracked_entries(&top).ok()?);
    let mut files = folder_files(&top, folder, &tracked, &vcs.untracked_entries(&top).ok()?);
    files.retain(|path| is_regular_file(&top, &folder.join(path.as_str())));
    Some(files)
}
