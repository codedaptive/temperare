//! `RulesWorld`: what `cctao rules` needs from git, the disk, the engines and the clock, as eight seams.

use ccvcslib::VcsError;
use report::Oid;
use rules::{Problem, RuleFile};
use std::path::{Path, PathBuf};

/// `run` passes `git_toplevel`, `Path::is_file`, `RuleFile::read`, `availability_at`, `now_secs`,
/// `git_rev_parse`, a ccvcslib file read for `blob_at` and `engines::no_file_rules`; a unit test passes closures, so
/// `run_rules` is tested without running git, reading the disk or the clock, or looking at PATH.
pub struct RulesWorld<'a> {
    /// The top of the repository holding a folder.
    pub toplevel: &'a dyn Fn(&Path) -> Result<PathBuf, VcsError>,
    /// Whether a rule file is there.
    pub exists: &'a dyn Fn(&Path) -> bool,
    /// The rule file read and checked, or its problems.
    pub read: &'a dyn Fn(&Path) -> Result<RuleFile, Vec<Problem>>,
    /// Whether cctao finds an engine for a rule file in the folder given (its program, from the folder's
    /// node_modules/.bin for ESLint, else PATH); nothing is run.
    pub availability: &'a dyn Fn(rules::Engine, &Path) -> engines::Availability,
    /// Whole seconds since the Unix epoch; the UTC day holding it decides which exceptions have expired.
    pub now: &'a dyn Fn() -> u64,
    /// The commit a revision names in the repository holding a folder; None when it names none.
    pub rev_parse: &'a dyn Fn(&Path, &str) -> Result<Option<Oid>, VcsError>,
    /// A file's bytes at a commit in the repository holding a folder, the path relative to that folder (a file name
    /// alone, no `./`); None when the commit holds no such path.
    pub blob_at: &'a dyn Fn(&Path, &Oid, &str) -> Result<Option<Vec<u8>>, VcsError>,
    /// The rules of a rule file that `cctao check`, run in a folder, settles as having no file before any engine runs.
    pub no_file_rules: &'a dyn Fn(&RuleFile, &Path) -> Vec<engines::NoFileRule>,
}
