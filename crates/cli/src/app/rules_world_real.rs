//! `RulesWorld::real`: the world wired to the real git, disk, engines and clock.

use super::availability_at::availability_at;
use super::git_rev_parse::git_rev_parse;
use super::git_toplevel::git_toplevel;
use super::now_secs::now_secs;
use super::RulesWorld;
use ccvcslib::{GitEnvPolicy, GitVcs, Revision, Vcs, VcsError};
use report::Oid;
use rules::RuleFile;
use std::path::Path;

impl RulesWorld<'static> {
    /// The world wired to the real git (through ccvcslib), disk, PATH and clock: `git_toplevel`, `Path::is_file`,
    /// `RuleFile::read`, `availability_at`, `now_secs`, `git_rev_parse`, `blob_at` and `engines::no_file_rules`.
    /// `run` uses it; tests of `run_rules` pass closures instead.
    pub fn real() -> RulesWorld<'static> {
        RulesWorld {
            toplevel: &git_toplevel,
            exists: &Path::is_file,
            read: &RuleFile::read,
            availability: &availability_at,
            now: &now_secs,
            rev_parse: &git_rev_parse,
            blob_at: &blob_at,
            no_file_rules: &engines::no_file_rules,
        }
    }
}

/// The bytes of `path` (relative to `dir`) in commit `oid`; None when the commit holds no such path.
fn blob_at(dir: &Path, oid: &Oid, path: &str) -> Result<Option<Vec<u8>>, VcsError> {
    GitVcs { policy: GitEnvPolicy::Lifecycle }.file_bytes_at_revision(dir, &Revision(oid.as_str().to_string()), path)
}
