//! `BaselineWorld`: what `tm baseline test` needs from the built-in sets, the disk, git and the engines, as eight
//! seams.

use engines::CheckedRules;
use rules::{Problem, RelPath, Rule, RuleFile};
use std::collections::BTreeMap;
use std::ops::Range;
use std::path::{Path, PathBuf};

/// `BaselineWorld::real()` wires the sets built into tm, case folders under the system temp folder, the real disk,
/// `git init` and staging through procio, commit and added lines through ccvcslib, and
/// `engines::run_rules_versioned_at` (given the case's base commit, `None` without one); a unit test passes closures,
/// so `tm baseline test` is tested without touching the disk or running git or an engine.
pub struct BaselineWorld<'a> {
    /// Every set tm carries, as ("name@version", ready), in tm's order: without `--set`, the ready ones are tested in
    /// this order.
    pub default_sets: &'a [(String, bool)],
    /// The rules of the set "name@version", in the set's order, whatever its ready mark; a set that cannot be read is
    /// its problems (rules' `Problem`s: their line and column index rules' own text, not the user's).
    pub rules_of: &'a dyn Fn(&str) -> Result<Vec<Rule>, Vec<Problem>>,
    /// A new empty folder for one case.
    pub make_folder: &'a dyn Fn() -> std::io::Result<PathBuf>,
    /// Writes text at a path, making its parent folders.
    pub write_file: &'a dyn Fn(&Path, &str) -> std::io::Result<()>,
    /// Stages every file in a case folder and commits it (an empty commit when there is none); the commit's id.
    pub commit_all: &'a dyn Fn(&Path) -> std::io::Result<String>,
    /// Stages every file in a case folder, then the lines each file gained since the commit given: keys relative to
    /// the folder, 1-based half-open line ranges (the real world asks ccvcslib's `added_lines`).
    pub added_since: &'a dyn Fn(&Path, &str) -> std::io::Result<BTreeMap<PathBuf, Vec<Range<u32>>>>,
    /// Removes a case folder and all in it.
    pub remove_folder: &'a dyn Fn(&Path) -> std::io::Result<()>,
    /// Every rule of the rule file run by its engine in the repository given, cut to the paths given (none cuts
    /// nothing), against the base commit given (`None` without one), each with its outcome.
    pub run_rules: &'a dyn Fn(&RuleFile, &Path, &[RelPath], Option<&str>) -> CheckedRules,
}
