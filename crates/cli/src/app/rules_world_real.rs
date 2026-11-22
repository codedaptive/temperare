//! `RulesWorld::real`: the world wired to the real git, disk, engines and clock.

use super::now_secs::now_secs;
use super::RulesWorld;
use rules::RuleFile;
use std::path::Path;

impl RulesWorld<'static> {
    /// The world wired to the real git, disk, PATH and clock: `procio::toplevel`, `Path::is_file`, `RuleFile::read`,
    /// `engines::availability`, `now_secs`, `procio::rev_parse`, `procio::blob_at` and `engines::no_file_rules`. `run`
    /// uses it; tests of `run_rules` pass closures instead.
    pub fn real() -> RulesWorld<'static> {
        RulesWorld {
            toplevel: &procio::toplevel,
            exists: &Path::is_file,
            read: &RuleFile::read,
            availability: &engines::availability,
            now: &now_secs,
            rev_parse: &procio::rev_parse,
            blob_at: &procio::blob_at,
            no_file_rules: &engines::no_file_rules,
        }
    }
}
