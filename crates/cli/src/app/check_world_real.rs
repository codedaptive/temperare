//! `CheckWorld::real`: the world wired to the real git, disk, engines and report.

use super::CheckWorld;
use rules::RuleFile;
use std::path::Path;

impl CheckWorld<'static> {
    /// The world wired to the real git, disk and PATH: `procio::toplevel`, `Path::is_file`, `RuleFile::read`,
    /// `procio::rev_parse`, `engines::run_rules` and `report::build_report`. Tests of `tm check` pass closures instead.
    pub fn real() -> CheckWorld<'static> {
        CheckWorld {
            toplevel: &procio::toplevel,
            exists: &Path::is_file,
            read: &RuleFile::read,
            rev_parse: &procio::rev_parse,
            run_rules: &engines::run_rules,
            build_report: &report::build_report,
        }
    }
}
