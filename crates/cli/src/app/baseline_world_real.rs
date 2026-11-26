//! `BaselineWorld::real`: the world wired to the sets built into tm, case folders under the system temp folder, the
//! real disk, `git init` and staging through procio, commit and added lines through ccvcslib, and the real engines.

use super::BaselineWorld;
use ccvcslib::{GitEnvPolicy, GitVcs, Revision, Vcs};
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, ErrorKind};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::LazyLock;

/// Every set built into tm with its ready mark, from `rules::baseline_sets`: read once, as `default_sets` borrows it for
/// the whole process.
static SETS: LazyLock<Vec<(String, bool)>> = LazyLock::new(rules::baseline_sets);

/// The next case folder number: shared by the whole process, so no two cases of one run get the same folder name,
/// even after the first folder is removed.
static NEXT: AtomicU64 = AtomicU64::new(0);

impl BaselineWorld<'static> {
    /// The real world: the sets and ready marks of `rules::baseline_sets`, a set's rules from `rules::baseline_rules`,
    /// each case folder a new git repository under the system temp folder (`git init` through procio), `fs` for writing
    /// and removing, staging through procio, a change case's base commit and added lines through ccvcslib, and
    /// `engines::run_rules_versioned_at`. Tests pass closures.
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
/// after when that name is already there), made a git repository with `git init -q` (`git()`, no hook runs): an
/// engine that tm hands its files (ShellCheck) lists them with git. A git that cannot run or exits non-zero is an
/// error, the folder removed first.
fn make_folder() -> io::Result<PathBuf> {
    let folder = loop {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let folder = std::env::temp_dir().join(format!("tm-baseline-{}-{n}", std::process::id()));
        match fs::create_dir(&folder) {
            Ok(()) => break folder,
            Err(e) if e.kind() == ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e),
        }
    };
    git(&folder, "init", &["init", "-q"]).inspect_err(|_| {
        let _ = fs::remove_dir_all(&folder);
    })?;
    Ok(folder)
}

/// Stages every file in `folder` (`git()`, no hook runs) and commits it as `base` through ccvcslib's
/// `commit_index`: tm's own name, no signing so no user setting is needed, no hook, and an empty commit when there is
/// nothing to commit. The new commit's id.
fn commit_all(folder: &Path) -> io::Result<String> {
    git(folder, "add", &["add", "-A"])?;
    VCS.commit_index(folder, "tm", "tm@localhost", "base")
        .map(|revision| revision.0)
        .map_err(|e| io::Error::other(format!("git commit: {e}")))
}

/// Stages every file in `folder` (`git()`), so a new file's lines count as added, then the lines each file gained
/// since `base`, from ccvcslib's `added_lines` over the whole folder: each file's path relative to the folder, its
/// added lines as 1-based half-open ranges (none when it only lost lines).
fn added_since(folder: &Path, base: &str) -> io::Result<BTreeMap<PathBuf, Vec<Range<u32>>>> {
    git(folder, "add", &["add", "-A"])?;
    let added = VCS.added_lines(folder, &Revision(base.to_string()), &[]);
    let added = added.map_err(|e| io::Error::other(format!("git diff: {e}")))?;
    Ok(added.into_iter().map(|file| (file.path, file.lines.iter().map(|l| l.start..l.end).collect())).collect())
}

/// The git every case folder is asked of, with the environment policy the rest of tm uses.
const VCS: GitVcs = GitVcs { policy: GitEnvPolicy::Lifecycle };

/// Turns every git hook off for `git()`, the user's global `core.hooksPath` and the repository's own `.git/hooks`
/// alike: git reads `core.hooksPath` as the folder its hooks live in, `/dev/null` holds none, and a `-c` setting
/// outranks every config file.
const NO_HOOKS: [&str; 2] = ["-c", "core.hooksPath=/dev/null"];

/// Runs git with `NO_HOOKS` then `args` on `folder` through `procio::git_cmd` (`-C <folder>`, and no inherited
/// variable such as `GIT_DIR` points it at another repository), so no hook runs; a git that cannot run, or exits
/// non-zero, is an error naming `step` (and the exit code and stderr). Its two uses are `init -q` and `add -A`.
// Not ccvcslib's `make_repository` and `stage_all`: in ccvcslib v0.2.0 (git_make_repository.rs:15,
// git_stage_all.rs:15) they set no `core.hooksPath`, so a global reference-transaction hook runs on init's HEAD write
// and post-index-change on add; global_hooks_never_run_in_case_repositories
// (crates/cli/tests/world/baseline_git_env.rs) pins both. This goes when a ccvcslib tag whose `make_repository` and
// `stage_all` turn hooks off is pinned.
fn git(folder: &Path, step: &str, args: &[&str]) -> io::Result<()> {
    match procio::run(&procio::git_cmd(folder, NO_HOOKS.iter().chain(args).map(Into::into).collect())) {
        Ok(output) if output.code == Some(0) => Ok(()),
        Ok(output) => Err(io::Error::other(format!(
            "git {step} exited {:?}: {}",
            output.code,
            String::from_utf8_lossy(&output.stderr).trim()
        ))),
        Err(e) => Err(io::Error::other(format!("git {step} could not run: {e}"))),
    }
}

/// Writes `text` at `path`, making its parent folders first. The path comes from `case_path`, which keeps it inside
/// the case folder.
fn write_file(path: &Path, text: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, text)
}
