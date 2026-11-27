//! `BaselineWorld::real`: the world wired to the sets built into cctao, case folders under the system temp folder, the
//! real disk, the case's git repository made, staged, committed and diffed through ccvcslib, and the real engines.

use super::BaselineWorld;
use ccvcslib::{GitEnvPolicy, GitVcs, Revision, Vcs};
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, ErrorKind};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::LazyLock;

/// Every set built into cctao with its ready mark, from `rules::baseline_sets`: read once, as `default_sets` borrows it
/// for the whole process.
static SETS: LazyLock<Vec<(String, bool)>> = LazyLock::new(rules::baseline_sets);

/// The next case folder number: shared by the whole process, so no two cases of one run get the same folder name,
/// even after the first folder is removed.
static NEXT: AtomicU64 = AtomicU64::new(0);

impl BaselineWorld<'static> {
    /// The real world: the sets and ready marks of `rules::baseline_sets`, a set's rules from `rules::baseline_rules`,
    /// each case folder a new git repository under the system temp folder (ccvcslib's `make_repository`), `fs` for
    /// writing and removing, staging, a change case's base commit and added lines through ccvcslib (`stage_all`,
    /// `commit_index`, `added_lines`), and `engines::run_rules_versioned_at`. Tests pass closures.
    pub fn real() -> BaselineWorld<'static> {
        BaselineWorld {
            default_sets: &SETS,
            rules_of: &rules::baseline_rules,
            make_folder: &make_folder,
            write_file: &write_file,
            commit_all: &commit_all,
            added_since: &added_since,
            remove_folder: &|folder| fs::remove_dir_all(folder),
            run_rules: &engines::run_rules_versioned_at,
        }
    }
}

/// A new empty folder `tm-baseline-<pid>-<n>` under the system temp folder, `n` the process's next number (the one
/// after when that name is already there), made a git repository with `git init -q` through ccvcslib's
/// `make_repository`, no hook run (it passes `core.hooksPath=/dev/null`, `NUL` on Windows): an engine that cctao hands
/// its files (ShellCheck) lists them with git. A git that cannot run or fails is an error naming the step
/// ("git init: ..."), the folder removed first.
fn make_folder() -> io::Result<PathBuf> {
    let folder = loop {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let folder = std::env::temp_dir().join(format!("cctao-baseline-{}-{n}", std::process::id()));
        match fs::create_dir(&folder) {
            Ok(()) => break folder,
            Err(e) if e.kind() == ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e),
        }
    };
    VCS.make_repository(&folder).map_err(|e| io::Error::other(format!("git init: {e}"))).inspect_err(|_| {
        let _ = fs::remove_dir_all(&folder);
    })?;
    Ok(folder)
}

/// Stages every file in `folder` through ccvcslib's `stage_all` (no hook runs; `core.autocrlf=false`, so line
/// endings are staged as written; `VCS`'s Lifecycle policy, so no inherited variable such as `GIT_DIR` points it at
/// another repository) and commits it as `base` through its `commit_index`: cctao's own name, no signing so no user
/// setting is needed, no hook, and an empty commit when there is nothing to commit. The new commit's id.
fn commit_all(folder: &Path) -> io::Result<String> {
    VCS.stage_all(folder).map_err(|e| io::Error::other(format!("git add: {e}")))?;
    VCS.commit_index(folder, "cctao", "cctao@localhost", "base")
        .map(|revision| revision.0)
        .map_err(|e| io::Error::other(format!("git commit: {e}")))
}

/// Stages every file in `folder` (`stage_all`, as `commit_all` does), so a new file's lines count as added, then the
/// lines each file gained since `base`, from ccvcslib's `added_lines` over the whole folder: each file's path relative
/// to the folder, its added lines as 1-based half-open ranges (none when it only lost lines).
fn added_since(folder: &Path, base: &str) -> io::Result<BTreeMap<PathBuf, Vec<Range<u32>>>> {
    VCS.stage_all(folder).map_err(|e| io::Error::other(format!("git add: {e}")))?;
    let added = VCS.added_lines(folder, &Revision(base.to_string()), &[]);
    let added = added.map_err(|e| io::Error::other(format!("git diff: {e}")))?;
    Ok(added.into_iter().map(|file| (file.path, file.lines.iter().map(|l| l.start..l.end).collect())).collect())
}

/// The git every case folder is made, staged, committed and diffed with, under the environment policy the rest of cctao
/// uses.
const VCS: GitVcs = GitVcs { policy: GitEnvPolicy::Lifecycle };

/// Writes `text` at `path`, making its parent folders first. The path comes from `case_path`, which keeps it inside
/// the case folder.
fn write_file(path: &Path, text: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, text)
}
