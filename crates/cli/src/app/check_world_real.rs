//! `CheckWorld::real`: the world wired to the real git, disk, engines, report, HOME and clock.

use super::CheckWorld;
use rules::RuleFile;
use std::path::{Path, PathBuf};

impl CheckWorld<'static> {
    /// The real world: `procio::toplevel`, `Path::is_file`, `RuleFile::read`, `procio::rev_parse`,
    /// `engines::run_rules_versioned`, `engines::availability`, `report::build_report`, HOME (`home`) and the clock
    /// (`now`). Tests pass closures.
    pub fn real() -> CheckWorld<'static> {
        CheckWorld {
            toplevel: &procio::toplevel,
            exists: &Path::is_file,
            read: &RuleFile::read,
            rev_parse: &procio::rev_parse,
            run_rules: &engines::run_rules_versioned,
            availability: &engines::availability,
            build_report: &report::build_report,
            home: &home,
            now: &now,
        }
    }
}

/// HOME from the environment; None when it is unset or empty.
fn home() -> Option<PathBuf> {
    std::env::var_os("HOME").filter(|home| !home.is_empty()).map(PathBuf::from)
}

/// Whole seconds since the Unix epoch; 0 for a clock set before it.
fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |since| since.as_secs())
}
