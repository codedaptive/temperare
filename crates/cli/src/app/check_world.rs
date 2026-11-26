//! `CheckWorld`: what `tm check` needs from git, the disk, the engines, the report, HOME and the clock, as nine
//! seams.

use ccvcslib::VcsError;
use engines::{Availability, CheckedRules};
use report::{Oid, Report, ReportError};
use rules::{Engine, Problem, RelPath, RuleFile, RuleOutcome};
use std::path::{Path, PathBuf};

/// `CheckWorld::real()` wires `git_toplevel`, `Path::is_file`, `RuleFile::read`, `git_rev_parse` (git through
/// ccvcslib), `engines::run_rules_versioned_at` (given the base commit, `None` without one), `engines::availability`,
/// `report::build_report_with_no_files`, HOME from the environment and the system clock; a unit test passes closures,
/// so `tm check` is tested without running git or an engine, reading the disk or the environment, or looking at PATH.
pub struct CheckWorld<'a> {
    /// The top of the repository holding a folder.
    pub toplevel: &'a dyn Fn(&Path) -> Result<PathBuf, VcsError>,
    /// Whether a rule file is there.
    pub exists: &'a dyn Fn(&Path) -> bool,
    /// The rule file read and checked, or its problems.
    pub read: &'a dyn Fn(&Path) -> Result<RuleFile, Vec<Problem>>,
    /// The commit a revision names in a repository; None when it names none.
    pub rev_parse: &'a dyn Fn(&Path, &str) -> Result<Option<Oid>, VcsError>,
    /// Every rule run by its engine over the rule file's folder (relative to the repository), cut to the paths given,
    /// against the base commit given (`None` without one), in rule-file order, each with its engine and the
    /// characters its counted run printed; and each distinct engine the rules name, once, with its version (None when
    /// it could not be read).
    pub run_rules: &'a dyn Fn(&RuleFile, &Path, &[RelPath], Option<&str>) -> CheckedRules,
    /// Whether tm finds an engine here, from a PATH lookup; nothing is run.
    pub availability: &'a dyn Fn(Engine) -> Availability,
    /// The report of the outcomes, new findings only when given `(repository, base commit)`; the last argument is the
    /// rules the engines found no file for, each (rule id, the engines' why), in run order.
    pub build_report: &'a dyn Fn(
        &RuleFile,
        Vec<(String, RuleOutcome)>,
        Option<(&Path, &Oid)>,
        Vec<(String, String)>,
    ) -> Result<Report, ReportError>,
    /// The user's home folder, where the savings switch and log live; None when HOME is unset or empty.
    pub home: &'a dyn Fn() -> Option<PathBuf>,
    /// Seconds since the Unix epoch, for the savings log line.
    pub now: &'a dyn Fn() -> u64,
}
