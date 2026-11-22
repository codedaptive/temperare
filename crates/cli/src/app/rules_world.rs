//! `RulesWorld`: what `tm rules` needs from git, the disk, the engines and the clock, as eight seams.

use procio::GitError;
use rules::{Problem, RuleFile};
use std::path::{Path, PathBuf};

/// `run` passes `procio::toplevel`, `Path::is_file`, `RuleFile::read`, `engines::availability`, `now_secs`,
/// `procio::rev_parse`, `procio::blob_at` and `engines::no_file_rules`; a unit test passes closures, so `run_rules` is
/// tested without running git, reading the disk or the clock, or looking at PATH.
pub struct RulesWorld<'a> {
    /// The top of the repository holding a folder.
    pub toplevel: &'a dyn Fn(&Path) -> Result<PathBuf, GitError>,
    /// Whether a rule file is there.
    pub exists: &'a dyn Fn(&Path) -> bool,
    /// The rule file read and checked, or its problems.
    pub read: &'a dyn Fn(&Path) -> Result<RuleFile, Vec<Problem>>,
    /// Whether a rule's engine can be run here.
    pub availability: &'a dyn Fn(rules::Engine) -> engines::Availability,
    /// Whole seconds since the Unix epoch; the UTC day holding it decides which exceptions have expired.
    pub now: &'a dyn Fn() -> u64,
    /// The commit a revision names in the repository holding a folder; None when it names none.
    pub rev_parse: &'a dyn Fn(&Path, &str) -> Result<Option<procio::Oid>, GitError>,
    /// A file's bytes at a revision in the repository holding a folder, the path as git reads it there (`./<name>` is
    /// relative to that folder); None when the revision holds no such path.
    pub blob_at: &'a dyn Fn(&Path, &str, &Path) -> Result<Option<Vec<u8>>, GitError>,
    /// The rules of a rule file that `tm check`, run in a folder, settles as having no file before any engine runs.
    pub no_file_rules: &'a dyn Fn(&RuleFile, &Path) -> Vec<engines::NoFileRule>,
}
