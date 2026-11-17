//! `CheckWorld`: what `tm check` needs from git, the disk, the engines and the report, as six seams.

use procio::{GitError, Oid};
use report::{Report, ReportError};
use rules::{Problem, RelPath, RuleFile, RuleOutcome};
use std::path::{Path, PathBuf};

/// `CheckWorld::real()` wires `procio::toplevel`, `Path::is_file`, `RuleFile::read`, `procio::rev_parse`,
/// `engines::run_rules` and `report::build_report`; a unit test passes closures, so `tm check` is tested without
/// running git or an engine, reading the disk or looking at PATH.
pub struct CheckWorld<'a> {
    /// The top of the repository holding a folder.
    pub toplevel: &'a dyn Fn(&Path) -> Result<PathBuf, GitError>,
    /// Whether a rule file is there.
    pub exists: &'a dyn Fn(&Path) -> bool,
    /// The rule file read and checked, or its problems.
    pub read: &'a dyn Fn(&Path) -> Result<RuleFile, Vec<Problem>>,
    /// The commit a revision names in a repository; None when it names none.
    pub rev_parse: &'a dyn Fn(&Path, &str) -> Result<Option<Oid>, GitError>,
    /// Every rule run by its engine over the rule file's folder (relative to the repository), cut to the paths given.
    pub run_rules: &'a dyn Fn(&RuleFile, &Path, &[RelPath]) -> Vec<(String, RuleOutcome)>,
    /// The report of the outcomes, new findings only when given `(repository, base commit)`.
    pub build_report:
        &'a dyn Fn(&RuleFile, Vec<(String, RuleOutcome)>, Option<(&Path, &Oid)>) -> Result<Report, ReportError>,
}
