//! `RulesWorld`: what `tm rules` needs from git, the disk and the engines, as four seams.

use procio::GitError;
use rules::{Problem, RuleFile};
use std::path::{Path, PathBuf};

/// `run` passes `procio::toplevel`, `Path::is_file`, `RuleFile::read` and `engines::availability`; a unit test passes
/// closures, so `run_rules` is tested without running git, reading the disk or looking at PATH.
pub struct RulesWorld<'a> {
    /// The top of the repository holding a folder.
    pub toplevel: &'a dyn Fn(&Path) -> Result<PathBuf, GitError>,
    /// Whether a rule file is there.
    pub exists: &'a dyn Fn(&Path) -> bool,
    /// The rule file read and checked, or its problems.
    pub read: &'a dyn Fn(&Path) -> Result<RuleFile, Vec<Problem>>,
    /// Whether a rule's engine can be run here.
    pub availability: &'a dyn Fn(rules::Engine) -> engines::Availability,
}
