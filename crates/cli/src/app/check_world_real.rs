//! `CheckWorld::real`: the world wired to the real git, disk, engines, report, HOME and clock.

use super::git_rev_parse::git_rev_parse;
use super::git_toplevel::git_toplevel;
use super::now_secs::now_secs;
use super::CheckWorld;
use rules::RuleFile;
use std::path::{Path, PathBuf};

impl CheckWorld<'static> {
    /// The real world: `git_toplevel`, `Path::is_file`, `RuleFile::read`, `git_rev_parse` (git through ccvcslib),
    /// `engines::run_rules_versioned_at`, `engines::availability`, `report::build_report_with_no_files`, HOME (`home`)
    /// and the clock (`now_secs`). Tests pass closures.
    pub fn real() -> CheckWorld<'static> {
        CheckWorld {
            toplevel: &git_toplevel,
            exists: &Path::is_file,
            read: &RuleFile::read,
            rev_parse: &git_rev_parse,
            run_rules: &engines::run_rules_versioned_at,
            availability: &engines::availability,
            build_report: &report::build_report_with_no_files,
            home: &home,
            now: &now_secs,
        }
    }
}

/// HOME from the environment; None when it is unset or empty.
fn home() -> Option<PathBuf> {
    std::env::var_os("HOME").filter(|home| !home.is_empty()).map(PathBuf::from)
}
