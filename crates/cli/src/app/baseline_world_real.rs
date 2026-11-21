//! `BaselineWorld::real`: the world wired to the sets built into tm, case folders under the system temp folder, the
//! real disk, git through procio and the real engines.

use super::BaselineWorld;
use crate::UsageError;
use rules::{Rule, RuleFile};
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, ErrorKind};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// The ready sets, tested without `--set`: standard@1. house@1 is not ready and runs only when named with `--set`.
/// rules has no public list of the sets and their ready marks yet; cli reads that list and drops this one once rules
/// ships it (Orc #1607, #1601).
const DEFAULT_SETS: &[&str] = &["standard@1"];

/// The next case folder number: shared by the whole process, so no two cases of one run get the same folder name,
/// even after the first folder is removed.
static NEXT: AtomicU64 = AtomicU64::new(0);

impl BaselineWorld<'static> {
    /// The real world: the ready set `standard@1` by default, a set's rules read through `RuleFile::parse`, each
    /// case folder a new git repository under the system temp folder, `fs` for writing and removing, git through
    /// procio for a change case's base and added lines, and `engines::run_rules_versioned_at`. Tests pass closures.
    pub fn real() -> BaselineWorld<'static> {
        BaselineWorld {
            default_sets: DEFAULT_SETS,
            rules_of: &rules_of,
            make_folder: &make_folder,
            write_file: &write_file,
            commit_all: &commit_all,
            added_since: &added_since,
            remove_folder: &|folder| fs::remove_dir_all(folder),
            run_rules: &engines::run_rules_versioned_at,
        }
    }
}

/// The rules of `set` ("name@version") in the set's order, not-applicable rules in their place: a rule file that
/// takes only that set, parsed by rules, which judges the entry (malformed, unknown set, unknown version). The toml
/// crate quotes `set`, so any text is one entry. A rule file with problems is the first problem as a usage error.
fn rules_of(set: &str) -> Result<Vec<Rule>, UsageError> {
    let text = format!("version = 1\nbaseline = [{}]\n", toml::Value::String(set.to_string()));
    match RuleFile::parse(&text, Path::new(".")) {
        Ok(rf) => Ok(rf.rules_in_order().into_iter().cloned().collect()),
        Err(problems) => Err(problems.into_iter().next().map_or_else(
            || UsageError {
                what: format!("cannot read baseline set \"{set}\""),
                fix: "Run tm help to see the commands.".to_string(),
            },
            |first| UsageError { what: first.what, fix: first.fix },
        )),
    }
}

/// A new empty folder `tm-baseline-<pid>-<n>` under the system temp folder, `n` the process's next number (the one
/// after when that name is already there), made a git repository with `git init -q`: an engine that tm hands its
/// files (ShellCheck) lists them with git. A git that cannot run or exits non-zero is an error, the folder removed
/// first.
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

/// Stages every file in `folder` and commits it as `base` (an empty commit when there is none), with tm's own name
/// and no signing so no user setting is needed; no hook of any kind runs, as `git()` passes `-c core.hooksPath=/dev/null`
/// on each of its calls (`git rev-parse` runs no hook). The commit's id from `git rev-parse`.
fn commit_all(folder: &Path) -> io::Result<String> {
    git(folder, "add", &["add", "-A"])?;
    let config = ["-c", "user.name=tm", "-c", "user.email=tm@localhost", "-c", "commit.gpgsign=false"];
    git(folder, "commit", &[&config[..], &["commit", "-q", "--allow-empty", "-m", "base"]].concat())?;
    match procio::rev_parse(folder, "HEAD") {
        Ok(Some(oid)) => Ok(oid.as_str().to_string()),
        Ok(None) => Err(io::Error::other("git rev-parse HEAD names no commit")),
        Err(e) => Err(io::Error::other(format!("git rev-parse HEAD: {e}"))),
    }
}

/// Stages every file in `folder`, so a new file's lines count as added, then the lines each file gained since
/// `base` (`procio::added_lines` over the whole folder).
fn added_since(folder: &Path, base: &str) -> io::Result<BTreeMap<PathBuf, Vec<Range<u32>>>> {
    git(folder, "add", &["add", "-A"])?;
    procio::added_lines(folder, base, &[]).map_err(|e| io::Error::other(format!("git diff: {e}")))
}

/// Turns every git hook off in a case repository, the user's global `core.hooksPath` and the repository's own
/// `.git/hooks` alike: git reads `core.hooksPath` as the folder its hooks live in, `/dev/null` holds none, and a `-c`
/// setting outranks every config file. `--no-verify` would skip only pre-commit and commit-msg.
const NO_HOOKS: [&str; 2] = ["-c", "core.hooksPath=/dev/null"];

/// Runs git with `NO_HOOKS` then `args` on `folder` through `procio::git_cmd` (`-C <folder>`, and no inherited
/// variable such as `GIT_DIR` points it at another repository), so no hook runs; a git that cannot run, or exits
/// non-zero, is an error naming `step` (and the exit code and stderr).
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
