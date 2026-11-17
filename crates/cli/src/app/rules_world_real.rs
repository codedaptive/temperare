//! `RulesWorld::real`: the world wired to the real git, disk and engines.

use super::RulesWorld;
use rules::RuleFile;
use std::path::Path;

impl RulesWorld<'static> {
    /// The world wired to the real git, disk and PATH: `procio::toplevel`, `Path::is_file`, `RuleFile::read` and
    /// `engines::availability`. `run` uses it; tests of `run_rules` pass closures instead.
    pub fn real() -> RulesWorld<'static> {
        RulesWorld {
            toplevel: &procio::toplevel,
            exists: &Path::is_file,
            read: &RuleFile::read,
            availability: &engines::availability,
        }
    }
}
